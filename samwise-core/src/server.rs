use crate::circuit_breaker::CircuitBreaker;
use crate::engine_wrapper::{EngineWrapper, SkillRecord};
use crate::errors::SidecarError;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct AppState {
    pub engine: Arc<EngineWrapper>,
    pub breaker: CircuitBreaker,
}

#[derive(Deserialize)]
pub struct BufferTurnReq {
    pub content: CompactString,
    #[serde(default = "default_memory_type")]
    pub memory_type: CompactString,
    pub metadata: Option<Metadata>,
}

fn default_memory_type() -> CompactString {
    CompactString::const_new("episodic")
}

#[derive(Deserialize)]
pub struct Metadata {
    pub importance: Option<f64>,
}

#[derive(Deserialize)]
pub struct FlushSessionReq {
    pub session_id: CompactString,
    #[serde(default)]
    pub final_flush: bool,
    #[serde(default = "default_batch_size")]
    pub batch_size: i64,
}

fn default_batch_size() -> i64 {
    50
}

#[derive(Deserialize)]
pub struct SkillSearchReq {
    pub context: CompactString,
}

#[derive(Deserialize)]
pub struct SkillDefineReq {
    pub skill_id: CompactString,
    pub body: CompactString,
    #[serde(default = "default_skill_type")]
    pub skill_type: CompactString,
    #[serde(default)]
    pub applies_to: Vec<CompactString>,
    #[serde(default = "default_cluster")]
    pub cluster_id: CompactString,
}

fn default_skill_type() -> CompactString {
    CompactString::const_new("procedural")
}
fn default_cluster() -> CompactString {
    CompactString::const_new("default")
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: &'static str,
}

#[derive(Serialize)]
pub struct FlushResponse {
    pub status: &'static str,
    pub count: usize,
}

pub fn create_router(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/governance/epoch", get(epoch_handler))
        .route("/v1/memory/buffer_turn", post(buffer_turn_handler))
        .route("/v1/memory/flush_session", post(flush_session_handler))
        .route("/skills/search", post(search_skills_handler))
        .route("/skills/define", post(define_skill_handler))
        .with_state(shared_state)
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "engine": "yantrikdb",
        "embedder": "potion-base-2M",
        "persistence": "wal"
    }))
}

async fn epoch_handler(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "epoch": state.engine.get_governance_epoch()
    }))
}

async fn buffer_turn_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BufferTurnReq>,
) -> Result<Json<StatusResponse>, SidecarError> {
    let importance = req.metadata.and_then(|m| m.importance).unwrap_or(0.5);
    let engine = Arc::clone(&state.engine);

    state
        .breaker
        .call(move || async move {
            engine
                .buffer_turn(req.content, req.memory_type, importance)
                .await
        })
        .await?;

    Ok(Json(StatusResponse { status: "ok" }))
}

async fn flush_session_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<FlushSessionReq>,
) -> Result<Json<FlushResponse>, SidecarError> {
    let engine = Arc::clone(&state.engine);
    let batch_size = req.batch_size;

    let claimed = state
        .breaker
        .call(move || async move { engine.claim_unprocessed_traces(batch_size).await })
        .await?;

    Ok(Json(FlushResponse {
        status: "traces_claimed",
        count: claimed.len(),
    }))
}

async fn search_skills_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SkillSearchReq>,
) -> Result<Json<Vec<SkillRecord>>, SidecarError> {
    let engine = Arc::clone(&state.engine);
    let skills = state
        .breaker
        .call(move || async move { engine.search_skills(req.context.as_str()).await })
        .await?;

    Ok(Json(skills))
}

async fn define_skill_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SkillDefineReq>,
) -> Result<Json<StatusResponse>, SidecarError> {
    let engine = Arc::clone(&state.engine);
    state
        .breaker
        .call(move || async move {
            engine
                .define_skill(
                    req.skill_id,
                    req.body,
                    req.skill_type,
                    req.applies_to,
                    req.cluster_id,
                )
                .await
        })
        .await?;

    Ok(Json(StatusResponse { status: "draft" }))
}
