use crate::errors::SidecarError;
use crate::server::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, HeaderName, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use compact_str::CompactString;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct ProxyState {
    pub sidecar_state: Arc<AppState>,
    pub llm_api_base: CompactString,
    pub client: Client,
}

const SKILL_INJECTION_HEADER: &str = "\n---\nRelevant Skills\nYou have access to the following guidelines. Apply when relevant:\n\n";
const MAX_CONTEXT_BYTES: usize = 4000;

#[derive(Deserialize, Serialize)]
struct ChatMessage {
    role: CompactString,
    content: CompactString,
}

#[derive(Deserialize, Serialize)]
struct ChatCompletionRequest {
    messages: Vec<ChatMessage>,
    #[serde(flatten)]
    extra: serde_json::Value,
}

pub fn create_proxy_router(sidecar_state: Arc<AppState>, llm_api_base: CompactString) -> Router {
    let state = Arc::new(ProxyState {
        sidecar_state,
        llm_api_base,
        client: Client::builder()
            .http2_keep_alive_interval(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client"),
    });

    Router::new()
        .route("/v1/chat/completions", post(chat_completions_handler))
        .with_state(state)
}

fn truncate_utf8_safe(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut idx = s.len() - max_bytes;
    while idx < s.len() && !s.is_char_boundary(idx) {
        idx += 1;
    }
    &s[idx..]
}

async fn chat_completions_handler(
    State(state): State<Arc<ProxyState>>,
    headers: HeaderMap,
    Json(mut payload): Json<ChatCompletionRequest>,
) -> impl IntoResponse {
    let engine = &state.sidecar_state.engine;

    let context: CompactString = payload
        .messages
        .iter()
        .map(|m| m.content.as_str())
        .collect::<Vec<_>>()
        .join(" ")
        .into();

    let truncated = truncate_utf8_safe(&context, MAX_CONTEXT_BYTES);

    // ZERO-LOOPBACK IN-MEMORY GOVERNANCE & RETRIEVAL
    let epoch_1 = engine.get_governance_epoch();
    let skills = engine.search_skills(truncated).await.unwrap_or_default();
    let epoch_2 = engine.get_governance_epoch();

    if epoch_1 != epoch_2 {
        return SidecarError::EpochMutation {
            expected: epoch_1,
            actual: epoch_2,
        }
        .into_response();
    }

    if !skills.is_empty() {
        let mut injection = CompactString::new(SKILL_INJECTION_HEADER.len() + skills.len() * 256);
        injection.push_str(SKILL_INJECTION_HEADER);
        for s in &skills {
            injection.push_str("Skill: ");
            injection.push_str(&s.skill_id);
            injection.push('\n');
            injection.push_str(&s.body);
            injection.push_str("\n\n");
        }
        injection.push_str("---\n");

        let injected = payload.messages.iter_mut().any(|m| {
            if m.role.as_str() == "system" {
                m.content = CompactString::new(m.content.len() + injection.len())
                    + m.content.as_str()
                    + injection.as_str();
                true
            } else {
                false
            }
        });

        if !injected {
            payload.messages.insert(
                0,
                ChatMessage {
                    role: CompactString::const_new("system"),
                    content: CompactString::new(injection.len() + 32)
                        + "You are a helpful assistant."
                        + injection.as_str(),
                },
            );
        }
    }

    let upstream_url = format!("{}/chat/completions", state.llm_api_base.trim_end_matches('/'));
    let mut req_builder = state.client.post(&upstream_url).json(&payload);

    for (k, v) in headers.iter() {
        if should_forward_header(k) {
            req_builder = req_builder.header(k, v);
        }
    }

    match req_builder.send().await {
        Ok(upstream_resp) => {
            let status = upstream_resp.status();
            // TRUE SSE STREAM PASSTHROUGH: O(1) Memory footprint
            let body = Body::from_stream(upstream_resp.bytes_stream());
            Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .body(body)
                .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
        Err(e) => {
            tracing::error!(error = %e, "Upstream LLM proxy connection failed");
            StatusCode::BAD_GATEWAY.into_response()
        }
    }
}

#[inline]
fn should_forward_header(name: &HeaderName) -> bool {
    !matches!(
        name.as_str(),
        "host" | "content-length" | "connection" | "transfer-encoding" | "upgrade"
    )
}
