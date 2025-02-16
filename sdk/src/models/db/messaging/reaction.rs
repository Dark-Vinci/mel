use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::others::messaging::CreateReaction;

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "reactions", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed)]
    pub message_id: Uuid,

    pub emoji_id: Uuid,

    pub count: u32,

    pub workspace_user_id: Uuid,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<CreateReaction> for Model {
    fn from(reaction: CreateReaction) -> Self {
        todo!()
    }
}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}