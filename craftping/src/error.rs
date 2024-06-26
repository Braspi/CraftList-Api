use elytra_ping::PingError;
use thiserror::Error;
use trust_dns_resolver::error::{ResolveError, ResolveErrorKind};

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CraftPingErrorKind {
    #[error("Ping timed out")]
    Timeout,

    #[error("Connection failed to finish")]
    Failed,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct CraftPingError {
    #[from]
    pub kind: CraftPingErrorKind,
}

impl From<PingError> for CraftPingError {
    fn from(value: PingError) -> Self {
        match value {
            PingError::Timeout { .. } => CraftPingErrorKind::Timeout,
            PingError::Protocol { .. } => CraftPingErrorKind::Failed,
        }
        .into()
    }
}

impl From<ResolveError> for CraftPingError {
    fn from(value: ResolveError) -> Self {
        match value.kind() {
            ResolveErrorKind::Timeout => CraftPingErrorKind::Timeout,
            ResolveErrorKind::NoRecordsFound { .. } => CraftPingErrorKind::Failed,
            _ => CraftPingErrorKind::Failed,
        }
        .into()
    }
}
