use std::fmt;

pub enum Failure {
    Reject(String),
    Error(anyhow::Error),
}

impl fmt::Debug for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reject(e) => fmt::Debug::fmt(e, f),
            Self::Error(e) => fmt::Debug::fmt(e, f),
        }
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reject(e) => fmt::Display::fmt(e, f),
            Self::Error(e) => fmt::Debug::fmt(e, f),
        }
    }
}

impl From<String> for Failure {
    fn from(value: String) -> Self {
        Self::Reject(value)
    }
}

impl From<anyhow::Error> for Failure {
    fn from(value: anyhow::Error) -> Self {
        Self::Error(value)
    }
}
