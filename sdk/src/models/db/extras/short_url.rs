use {
    crate::models::others::extras::CreateShortUrl,
    chrono::{DateTime, Utc},
    sea_orm::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Clone, Debug, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "short_url", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(column_type = "Text")]
    pub url: String,

    #[sea_orm(unique, indexed)]
    pub short: String,

    created_by: Uuid,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    created_at: DateTime<Utc>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    updated_at: DateTime<Utc>,

    #[sea_orm(nullable)]
    deleted_at: Option<DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<CreateShortUrl> for ActiveModel {
    fn from(_fro: CreateShortUrl) -> Self {
        let val: ActiveModel = Self {
            ..Default::default()
        };

        // todo: fill other fields

        val
    }
}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::short_url_track::Entity",
        from = "Column::Id",
        to = "super::short_url_track::Column::Id"
    )]
    ShortUrlTrack,

    #[sea_orm(
        belongs_to = "super::message::Entity",
        from = "Column::MessageId",
        to = "super::super::messaging::message::Column::Id"
    )]
    Message,
}

impl Related<super::short_url_track::Entity> for Relation {
    fn to() -> RelationDef {
        Relation::ShortUrlTrack.def()
    }
}

impl Related<super::super::messaging::message::Entity> for Relation {
    fn to() -> RelationDef {
        Relation::ShortUrlTrack.def()
    }
}

