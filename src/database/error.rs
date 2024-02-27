use std::fmt;

pub(crate) enum Error {
    DBError(sea_orm::DbErr),
    // Config
    ConfigMissingID,
    ConfigMissingTag,
    ConfigMissingConfig,
    ConfigInvalidConfig,
    ConfigDuplicateTag,
    ConfigNotFound(String), // ID
    // Script
    ScriptMissingID,
    ScriptMissingTag,
    ScriptDuplicateTag,
    ScriptNotFound(String), // ID
    // Kv
    KvMissingKey,
    KvNotFound(String), // Key
    //
    CustomErr(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DBError(e) => write!(f, "database error: {:?}", e),
            Self::ConfigMissingID => write!(f, "config: missing id"),
            Self::ConfigMissingTag => write!(f, "config: missing tag"),
            Self::ConfigMissingConfig => write!(f, "config: missing config"),
            Self::ConfigInvalidConfig => write!(f, "config: invalid config"),
            Self::ConfigDuplicateTag => write!(f, "config: duplicate tag"),
            Self::ConfigNotFound(id) => write!(f, "config: not found, id: {}", id),
            Self::ScriptMissingID => write!(f, "script: missing id"),
            Self::ScriptMissingTag => write!(f, "script: missing tag"),
            Self::ScriptDuplicateTag => write!(f, "script: duplicate tag"),
            Self::ScriptNotFound(id) => write!(f, "script: not found, id: {}", id),
            Self::KvMissingKey => write!(f, "kv: missing key"),
            Self::KvNotFound(key) => write!(f, "kv: not found, key: {}", key),
            Self::CustomErr(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DBError(e) => write!(f, "database error: {:?}", e),
            Self::ConfigMissingID => write!(f, "config: missing id"),
            Self::ConfigMissingTag => write!(f, "config: missing tag"),
            Self::ConfigMissingConfig => write!(f, "config: missing config"),
            Self::ConfigInvalidConfig => write!(f, "config: invalid config"),
            Self::ConfigDuplicateTag => write!(f, "config: duplicate tag"),
            Self::ConfigNotFound(id) => write!(f, "config: not found, id: {}", id),
            Self::ScriptMissingID => write!(f, "script: missing id"),
            Self::ScriptMissingTag => write!(f, "script: missing tag"),
            Self::ScriptDuplicateTag => write!(f, "script: duplicate tag"),
            Self::ScriptNotFound(id) => write!(f, "script: not found, id: {}", id),
            Self::KvMissingKey => write!(f, "kv: missing key"),
            Self::KvNotFound(key) => write!(f, "kv: not found, key: {}", key),
            Self::CustomErr(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}
