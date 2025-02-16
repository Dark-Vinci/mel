use sea_orm::DeriveEntityModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "emojis", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub unicode: String,
}