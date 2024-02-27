use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, TransactionError, TransactionTrait,
};

use super::kv::{Column as KvColumn, Entity as KvEntity, Model as Kv};

const KEY_CORE_PATH: &str = "core_path";
const KEY_AUTO_START: &str = "auto_start";

// Set Core Path
pub(crate) async fn set_core_path(
    conn: &sea_orm::DatabaseConnection,
    path: String,
) -> Result<(), super::Error> {
    if path.is_empty() {
        return Err(super::Error::CustomErr(format!("missing path")));
    }
    conn.transaction(|tx| {
        Box::pin(async move {
            KvEntity::delete_many()
                .filter(KvColumn::Key.eq(KEY_CORE_PATH))
                .exec(tx)
                .await
                .map_err(|e| super::Error::DBError(e))?;

            Kv {
                key: KEY_CORE_PATH.to_string(),
                value: serde_json::Value::String(path),
            }
            .to_active_model()
            .insert(tx)
            .await
            .map_err(|e| super::Error::DBError(e))?;

            Ok(())
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Connection(e) => super::Error::DBError(e),
        TransactionError::Transaction(e) => e,
    })
}

// Get Core Path
pub(crate) async fn get_core_path(
    conn: &sea_orm::DatabaseConnection,
) -> Result<Option<String>, super::Error> {
    let kv = KvEntity::find_by_id(KEY_CORE_PATH)
        .one(conn)
        .await
        .map_err(|e| super::Error::DBError(e))?;

    match kv {
        Some(kv) => match kv.value {
            serde_json::Value::String(s) => Ok(Some(s)),
            _ => Ok(None),
        },
        None => Ok(None),
    }
}

// Set Auto Start
pub(crate) async fn set_auto_start(
    conn: &sea_orm::DatabaseConnection,
    enabled: bool,
) -> Result<(), super::Error> {
    conn.transaction(|tx| {
        Box::pin(async move {
            KvEntity::delete_many()
                .filter(KvColumn::Key.eq(KEY_AUTO_START))
                .exec(tx)
                .await
                .map_err(|e| super::Error::DBError(e))?;

            Kv {
                key: KEY_AUTO_START.to_string(),
                value: serde_json::Value::Bool(enabled),
            }
            .to_active_model()
            .insert(tx)
            .await
            .map_err(|e| super::Error::DBError(e))?;

            Ok(())
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Connection(e) => super::Error::DBError(e),
        TransactionError::Transaction(e) => e,
    })
}

// Get Auto Start
pub(crate) async fn get_auto_start(
    conn: &sea_orm::DatabaseConnection,
) -> Result<bool, super::Error> {
    let kv = KvEntity::find_by_id(KEY_AUTO_START)
        .one(conn)
        .await
        .map_err(|e| super::Error::DBError(e))?;

    match kv {
        Some(kv) => match kv.value {
            serde_json::Value::Bool(b) => Ok(b),
            _ => Ok(false),
        },
        None => Ok(false),
    }
}
