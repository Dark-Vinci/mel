use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter, Related, RelationDef, RelationTrait};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::others::auth::create::UpdateUserRequest;
use crate::models::others::extras::CreateShortUrl;

#[derive(Debug, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "short_url", schame_name="public")]
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

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation{
    #[sea_orm(
        belongs_to = "super::short_url_track::Entity",
        from = "Column::ShortUrlTrackID",
        to = "super::short_url_track::Column::Id",
    )]
    ShortUrlTrack
}

impl Related<super::short_url_track::Entity> for Relation {
    fn to() -> RelationDef {
        Relation::ShortUrlTrack.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<CreateShortUrl> for ActiveModel {
    fn from(fro: UpdateUserRequest) -> Self {
        let mut val: ActiveModel = Self {
            ..Default::default()
        };

        // todo: fill other fields

        val
    }
}