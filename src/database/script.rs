use sea_orm::{
    entity::prelude::*, ActiveModelTrait, ActiveValue, TransactionError, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::common;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ScriptRunType {
    Disabled = 0,
    BeforeStart,
    AfterStart,
    BeforeClose,
    AfterClose,
}

impl Into<u8> for ScriptRunType {
    fn into(self) -> u8 {
        match self {
            Self::Disabled => 0,
            Self::BeforeStart => 1,
            Self::AfterStart => 2,
            Self::BeforeClose => 3,
            Self::AfterClose => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "script")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub tag: String,
    pub content: String,
    pub run_type: u8,
}

impl Model {
    fn to_active_model(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::set(self.id),
            tag: ActiveValue::set(self.tag),
            content: ActiveValue::set(self.content),
            run_type: ActiveValue::set(self.run_type),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// Add Script
pub(crate) async fn add_script(
    conn: &sea_orm::DatabaseConnection,
    mut script: Model,
) -> Result<Model, super::Error> {
    if script.tag.is_empty() {
        return Err(super::Error::ScriptMissingTag);
    }
    if script.id.is_empty() {
        script.id = common::random_uuid().replace("-", "");
    }
    conn.transaction(|tx| {
        Box::pin(async move {
            let result = Entity::find_by_id(&script.id)
                .one(tx)
                .await
                .map_err(|e| super::Error::DBError(e))?;

            if let Some(_) = result {
                return Err(super::Error::ScriptDuplicateTag);
            }

            script
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

// Get Script
pub(crate) async fn get_script(
    conn: &sea_orm::DatabaseConnection,
    id: String,
) -> Result<Model, super::Error> {
    if id.is_empty() {
        return Err(super::Error::ScriptMissingID);
    }
    Entity::find_by_id(&id)
        .one(conn)
        .await
        .map_err(|e| super::Error::DBError(e))
        .and_then(|result| result.ok_or(super::Error::ScriptNotFound(id)))
}

// Modify Script
pub(crate) async fn modify_script(
    conn: &sea_orm::DatabaseConnection,
    id: String,
    mut script: ActiveModel,
) -> Result<Model, super::Error> {
    if id.is_empty() {
        return Err(super::Error::ScriptMissingID);
    }
    script.id = ActiveValue::set(id);
    script
        .update(conn)
        .await
        .map_err(|e| super::Error::DBError(e))
}

// Delete Script
pub(crate) async fn delete_script(
    conn: &sea_orm::DatabaseConnection,
    id: String,
) -> Result<(), super::Error> {
    if id.is_empty() {
        return Err(super::Error::ScriptMissingID);
    }
    conn.transaction::<_, (), super::Error>(|tx| {
        Box::pin(async move {
            let result = Entity::find_by_id(&id)
                .one(tx)
                .await
                .map_err(|e| super::Error::DBError(e))?;

            if let Some(script) = result {
                script
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

// Bulk Delete Scripts
pub(crate) async fn bulk_delete_script(
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

// List Script
pub(crate) async fn list_script(
    conn: &sea_orm::DatabaseConnection,
) -> Result<Vec<Model>, super::Error> {
    Entity::find()
        .all(conn)
        .await
        .map_err(|e| super::Error::DBError(e))
}

// Clean Script Run Type
pub(crate) async fn clean_script_type(
    conn: &sea_orm::DatabaseConnection,
    id: String,
) -> Result<(), super::Error> {
    Entity::update_many()
        .col_expr(
            Column::RunType,
            Expr::value(Value::TinyUnsigned(Some(ScriptRunType::Disabled.into()))),
        )
        .filter(Column::Id.eq(id))
        .exec(conn)
        .await
        .map_err(|e| super::Error::DBError(e))?;
    Ok(())
}

// Set Run Type Script
#[macro_export]
macro_rules! set_run_type_script_macro {
    ($name:ident, $label:expr) => {
        pub(crate) async fn $name(
            conn: &sea_orm::DatabaseConnection,
            id: String,
        ) -> Result<(), super::Error> {
            conn.transaction(|tx| {
                Box::pin(async move {
                    Entity::update_many()
                        .col_expr(
                            Column::RunType,
                            Expr::value(Value::TinyUnsigned(Some(ScriptRunType::Disabled.into()))),
                        )
                        .filter(Column::RunType.eq(Value::TinyUnsigned(Some($label.into()))))
                        .exec(tx)
                        .await
                        .map_err(|e| super::Error::DBError(e))?;

                    Entity::update_many()
                        .col_expr(
                            Column::RunType,
                            Expr::value(Value::TinyUnsigned(Some($label.into()))),
                        )
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
    };
}

// Get Run Type Script
#[macro_export]
macro_rules! get_run_type_script_macro {
    ($name:ident, $label:expr) => {
        pub(crate) async fn $name(
            conn: &sea_orm::DatabaseConnection,
        ) -> Result<Option<Model>, super::Error> {
            Entity::find()
                .filter(Column::RunType.eq(Value::TinyUnsigned(Some($label.into()))))
                .one(conn)
                .await
                .map_err(|e| super::Error::DBError(e))
        }
    };
}

set_run_type_script_macro!(set_before_start_script, ScriptRunType::BeforeStart);
get_run_type_script_macro!(get_before_start_script, ScriptRunType::BeforeStart);
set_run_type_script_macro!(set_after_start_script, ScriptRunType::AfterStart);
get_run_type_script_macro!(get_after_start_script, ScriptRunType::AfterStart);
set_run_type_script_macro!(set_before_close_script, ScriptRunType::BeforeClose);
get_run_type_script_macro!(get_before_close_script, ScriptRunType::BeforeClose);
set_run_type_script_macro!(set_after_close_script, ScriptRunType::AfterClose);
get_run_type_script_macro!(get_after_close_script, ScriptRunType::AfterClose);
