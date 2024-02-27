use std::fmt;

pub(crate) enum Error {
    ConfigNotSet,
    GetConfigFailed(String),
    CorePathNotSet,
    GetCorePathFailed(String),
    StartServiceFailed(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConfigNotSet => write!(f, "config is not set"),
            Self::GetConfigFailed(s) => write!(f, "get config failed: {}", s),
            Self::CorePathNotSet => write!(f, "core path is not set"),
            Self::GetCorePathFailed(s) => write!(f, "get core path failed: {}", s),
            Self::StartServiceFailed(s) => write!(f, "start service failed: {}", s),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConfigNotSet => write!(f, "config is not set"),
            Self::GetConfigFailed(s) => write!(f, "get config failed: {}", s),
            Self::CorePathNotSet => write!(f, "core path is not set"),
            Self::GetCorePathFailed(s) => write!(f, "get core path failed: {}", s),
            Self::StartServiceFailed(s) => write!(f, "start service failed: {}", s),
        }
    }
}

impl std::error::Error for Error {}
