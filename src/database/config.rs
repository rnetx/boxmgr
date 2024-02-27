use sea_orm::{
    entity::prelude::*, ActiveModelTrait, ActiveValue, TransactionError, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::common;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "config")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub tag: String,
    pub config: serde_json::Value,
    pub actived: bool,
}

impl Model {
    fn to_active_model(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::set(self.id),
            tag: ActiveValue::set(self.tag),
            config: ActiveValue::set(self.config),
            actived: ActiveValue::set(self.actived),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// Add Config
pub(crate) async fn add_config(
    conn: &sea_orm::DatabaseConnection,
    mut config: Model,
) -> Result<Model, super::Error> {
    if config.tag.is_empty() {
        return Err(super::Error::ConfigMissingTag);
    }
    match &config.config {
        serde_json::Value::Object(_) => {}
        serde_json::Value::Null => return Err(super::Error::ConfigMissingConfig),
        _ => return Err(super::Error::ConfigInvalidConfig),
    }
    if config.id.is_empty() {
        config.id = common::random_uuid().replace("-", "");
    }
    conn.transaction(|tx| {
        Box::pin(async move {
            let result = Entity::find_by_id(&config.id)
                .one(tx)
                .await
                .map_err(|e| super::Error::DBError(e))?;

            if let Some(_) = result {
                return Err(super::Error::ConfigDuplicateTag);
            }

            config
                .to_active_model()
                .insert(tx)
                .await
                .map_err(|e| super::Error::DBError(e))
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Connection(e) => super::Error::DBError(e),
        TransactionError::Transaction(e) => e,
    })
}

// Get Config
pub(crate) async fn get_config(
    conn: &sea_orm::DatabaseConnection,
    id: String,
) -> Result<Model, super::Error> {
    if id.is_empty() {
        return Err(super::Error::ConfigMissingID);
    }
    Entity::find_by_id(&id)
        .one(conn)
        .await
        .map_err(|e| super::Error::DBError(e))
        .and_then(|result| result.ok_or(super::Error::ConfigNotFound(id)))
}

// Modify Config
pub(crate) async fn modify_config(
    conn: &sea_orm::DatabaseConnection,
    id: String,
    mut config: ActiveModel,
) -> Result<Model, super::Error> {
    if id.is_empty() {
        return Err(super::Error::ConfigMissingID);
    }
    if let ActiveValue::Set(v) = &config.config {
        match &v {
            serde_json::Value::Object(_) => {}
            serde_json::Value::Null => return Err(super::Error::ConfigMissingConfig),
            _ => return Err(super::Error::ConfigInvalidConfig),
        }
    }
    config.id = ActiveValue::set(id);
    config
        .update(conn)
        .await
        .map_err(|e| super::Error::DBError(e))
}

// Delete Config
pub(crate) async fn delete_config(
    conn: &sea_orm::DatabaseConnection,
    id: String,
) -> Result<(), super::Error> {
    if id.is_empty() {
        return Err(super::Error::ConfigMissingID);
    }
    conn.transaction::<_, (), super::Error>(|tx| {
        Box::pin(async move {
            let result = Entity::find_by_id(&id)
                .one(tx)
                .await
                .map_err(|e| super::Error::DBError(e))?;

            if let Some(config) = result {
                config
                    .delete(tx)
                    .await
                    .map_err(|e| super::Error::DBError(e))?;
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

// Bulk Delete Configs
pub(crate) async fn bulk_delete_config(
    conn: &sea_orm::DatabaseConnection,
    ids: Vec<String>,
) -> Result<(), super::Error> {
    if ids.len() == 0 {
        return Ok(());
    }
    let mut filter: Option<sea_orm::sea_query::SimpleExpr> = None;
    for id in ids {
        filter = match filter {
            Some(f) => Some(f.or(Column::Id.eq(id))),
            None => Some(Column::Id.eq(id)),
        };
    }
    Entity::delete_many()
        .filter(filter.unwrap())
        .exec(conn)
        .await
        .map_err(|e| super::Error::DBError(e))?;
    Ok(())
}

// List Config
pub(crate) async fn list_config(
    conn: &sea_orm::DatabaseConnection,
) -> Result<Vec<Model>, super::Error> {
    Entity::find()
        .all(conn)
        .await
        .map_err(|e| super::Error::DBError(e))
}

// Set Active Config
pub(crate) async fn set_active_config(
    conn: &sea_orm::DatabaseConnection,
    id: String,
) -> Result<(), super::Error> {
    conn.transaction(|tx| {
        Box::pin(async move {
            Entity::update_many()
                .col_expr(Column::Actived, Expr::value(Value::Bool(Some(false))))
                .filter(Column::Actived.eq(Value::Bool(Some(true))))
                .exec(tx)
                .await
                .map_err(|e| super::Error::DBError(e))?;

            Entity::update_many()
                .col_expr(Column::Actived, Expr::value(Value::Bool(Some(true))))
                .filter(Column::Id.eq(id))
                .exec(tx)
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

// Get Active Config
pub(crate) async fn get_active_config(
    conn: &sea_orm::DatabaseConnection,
) -> Result<Option<Model>, super::Error> {
    Entity::find()
        .filter(Column::Actived.eq(Value::Bool(Some(true))))
        .one(conn)
        .await
        .map_err(|e| super::Error::DBError(e))
}
