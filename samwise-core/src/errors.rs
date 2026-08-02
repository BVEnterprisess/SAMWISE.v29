use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use compact_str::CompactString;
use miette::Diagnostic;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategory {
    Transient,
    Permanent,
    Governance,
    Backpressure,
}

#[derive(Error, Debug, Diagnostic)]
pub enum SidecarError {
    #[error("Backpressure: SQLite connection pool saturated")]
    #[diagnostic(code(samwise::backpressure))]
    Backpressure(#[source] sqlx::Error),

    #[error("Governance escalation active at epoch {epoch}")]
    #[diagnostic(code(samwise::governance_escalation))]
    GovernanceEscalation { epoch: u64 },

    #[error("Epoch mutation detected mid-flight: expected {expected}, got {actual}")]
    #[diagnostic(code(samwise::epoch_mutation))]
    EpochMutation { expected: u64, actual: u64 },

    #[error("Circuit Breaker is OPEN")]
    #[diagnostic(code(samwise::circuit_open))]
    CircuitOpen,

    #[error("Permanent system failure: {0}")]
    #[diagnostic(code(samwise::permanent))]
    Permanent(CompactString),

    #[error("Transient system failure: {0}")]
    #[diagnostic(code(samwise::transient))]
    Transient(CompactString),
}

impl SidecarError {
    #[inline]
    pub const fn category(&self) -> ErrorCategory {
        match self {
            Self::Backpressure(_) => ErrorCategory::Backpressure,
            Self::GovernanceEscalation { .. } | Self::EpochMutation { .. } => {
                ErrorCategory::Governance
            }
            Self::CircuitOpen | Self::Transient(_) => ErrorCategory::Transient,
            Self::Permanent(_) => ErrorCategory::Permanent,
        }
    }

    #[inline]
    pub const fn is_retriable(&self) -> bool {
        matches!(
            self,
            Self::Backpressure(_)
                | Self::EpochMutation { .. }
                | Self::CircuitOpen
                | Self::Transient(_)
        )
    }
}

impl IntoResponse for SidecarError {
    fn into_response(self) -> Response {
        let status = match self.category() {
            ErrorCategory::Governance => StatusCode::LOCKED,
            ErrorCategory::Backpressure => StatusCode::TOO_MANY_REQUESTS,
            ErrorCategory::Transient => StatusCode::SERVICE_UNAVAILABLE,
            ErrorCategory::Permanent => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(serde_json::json!({
            "error": self.to_string(),
            "category": self.category(),
            "retriable": self.is_retriable(),
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for SidecarError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => Self::Backpressure(err),
            _ => Self::Permanent(CompactString::from(err.to_string())),
        }
    }
}
