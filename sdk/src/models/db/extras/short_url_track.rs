use {
    chrono::{DateTime, Utc},
    sea_orm::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};
use crate::models::others::extras::CreateShortUrlTrack;

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "short_url_track", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub short_url_id: Uuid,

    pub workspace_user_id: Uuid,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::short_url::Entity",
        from = "Column::ShortUrlId",
        to = "super::short_url::Column::Id"
    )]
    ShortUrl,
}

impl Related<super::short_url::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShortUrl.def()
    }
}

impl From<CreateShortUrlTrack> for ActiveModel {
    fn from(_model: CreateShortUrlTrack) -> Self {
        todo!()
    }
}
