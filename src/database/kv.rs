use sea_orm::{
    entity::prelude::*, ActiveModelTrait, ActiveValue, TransactionError, TransactionTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "kv")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub key: String,
    pub value: serde_json::Value,
}

impl Model {
    pub(crate) fn to_active_model(self) -> ActiveModel {
        ActiveModel {
            key: ActiveValue::set(self.key),
            value: ActiveValue::set(self.value),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// Set(Add/Modify) Kv
pub(crate) async fn set_kv(
    conn: &sea_orm::DatabaseConnection,
    kv: Model,
) -> Result<Model, super::Error> {
    if kv.key.is_empty() {
        return Err(super::Error::KvMissingKey);
    }
    conn.transaction(|tx| {
        Box::pin(async move {
            let result = Entity::find_by_id(&kv.key)
                .one(tx)
                .await
                .map_err(|e| super::Error::DBError(e))?;
            match result {
                Some(mut model) => {
                    model.value = kv.value.clone();
                    model
                        .to_active_model()
                        .update(tx)
                        .await
                        .map_err(|e| super::Error::DBError(e))?;
                    Ok(kv)
                }
                None => {
                    kv.clone()
                        .to_active_model()
                        .insert(tx)
                        .await
                        .map_err(|e| super::Error::DBError(e))?;
                    Ok(kv)
                }
            }
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Connection(e) => super::Error::DBError(e),
        TransactionError::Transaction(e) => e,
    })
}

// Get Kv
pub(crate) async fn get_kv<K: Into<String>>(
    conn: &sea_orm::DatabaseConnection,
    key: K,
) -> Result<Model, super::Error> {
    let key: String = key.into();
    if key.is_empty() {
        return Err(super::Error::KvMissingKey);
    }
    Entity::find_by_id(&key)
        .one(conn)
        .await
        .map_err(|e| super::Error::DBError(e))
        .and_then(|opt| opt.ok_or_else(|| super::Error::KvNotFound(key)))
}

// Delete Kv
pub(crate) async fn delete_kv(
    conn: &sea_orm::DatabaseConnection,
    key: String,
) -> Result<(), super::Error> {
    if key.is_empty() {
        return Err(super::Error::KvMissingKey);
    }
    conn.transaction::<_, (), super::Error>(|tx| {
        Box::pin(async move {
            let result = Entity::find_by_id(&key)
                .one(tx)
                .await
                .map_err(|e| super::Error::DBError(e))?;

            if let Some(kv) = result {
                kv.delete(tx).await.map_err(|e| super::Error::DBError(e))?;
            }

            Ok(())
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Connection(e) => super::Error::DBError(e),
        TransactionError::Transaction(e) => e,
    })
}

// List Kv
pub(crate) async fn list_kv(
    conn: &sea_orm::DatabaseConnection,
) -> Result<Vec<Model>, super::Error> {
    Entity::find()
        .all(conn)
        .await
        .map_err(|e| super::Error::DBError(e))
}
