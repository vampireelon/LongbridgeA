use longbridgea_core::CoreError;

#[derive(Debug, thiserror::Error)]
pub enum QuoteError {
    #[error("empty quote response")]
    EmptyResponse,
    #[error("bad quote format: {0}")]
    BadFormat(String),
    #[error("too few fields in quote payload: {0}")]
    TooFewFields(usize),
    #[error(transparent)]
    Core(#[from] CoreError),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Msg(String),
}

pub type Result<T> = std::result::Result<T, QuoteError>;
