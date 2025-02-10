use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter, Related, RelationDef, RelationTrait};
use sea_orm::tests_cfg::cake::{ActiveModel, Entity};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "short_url_track", schame_name="public")]
struct Model {
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
        from = "Column::ShortUrlID",
        to = "super::short_url::Column::Id",
    )]
    ShortUrl
}

impl Related<super::short_url::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShortUrl.def()
    }
}