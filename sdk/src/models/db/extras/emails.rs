use chrono::{DateTime, Utc};
use sea_orm::DeriveEntityModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "emails", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub user_id: Uuid,

    pub workspace_id: Uuid,

    pub content: serde_json::Value,

    pub seen: bool,

    #[sea_orm(type = "TEXT")]
    pub seen_url: String,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,

    pub seen_at: Option<DateTime<Utc>>,
}