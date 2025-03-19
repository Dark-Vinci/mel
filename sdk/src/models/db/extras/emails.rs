use {
    crate::models::others::extras::CreateEmail,
    chrono::{DateTime, Utc},
    sea_orm::{
        prelude::*, ActiveModelBehavior, DeriveEntityModel, DeriveRelation,
        EnumIter, Set,
    },
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

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

    #[sea_orm(type = "TEXT")]
    pub seen_url: String,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,

    pub seen_at: Option<DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}

impl From<CreateEmail> for ActiveModel {
    fn from(_value: CreateEmail) -> Self {
        todo!()
    }
}

// the only update should be on the {seen_at} field
impl ActiveModel {
    pub fn set_seen(&mut self) {
        self.seen_at = Set(Some(Utc::now()));
    }
}
