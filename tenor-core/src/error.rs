use thiserror::Error;

/// Core error type for Tenor operations
#[derive(Error, Debug)]
pub enum EngineError {
    /// User-actionable error (permission denied, daemon down, not found)
    #[error("{message}")]
    UserActionable {
        message: String,
        hint: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Retryable error (temporary network issue, timeout)
    #[error("{message}")]
    Retryable {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Bug/unexpected error (parsing failure, invariant violation)
    #[error("{message}")]
    Bug {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl EngineError {
    pub fn user_actionable(message: impl Into<String>, hint: Option<String>) -> Self {
        Self::UserActionable {
            message: message.into(),
            hint,
            source: None,
        }
    }

    pub fn user_actionable_with_source(
        message: impl Into<String>,
        hint: Option<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::UserActionable {
            message: message.into(),
            hint,
            source: Some(Box::new(source)),
        }
    }

    pub fn retryable(message: impl Into<String>) -> Self {
        Self::Retryable {
            message: message.into(),
            source: None,
        }
    }

    pub fn retryable_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Retryable {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    pub fn bug(message: impl Into<String>) -> Self {
        Self::Bug {
            message: message.into(),
            source: None,
        }
    }

    pub fn bug_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Bug {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
}

pub type EngineResult<T> = Result<T, EngineError>;
