use {
    chrono::Utc,
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "workspace", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub created_by: Uuid, // workspace owner

    pub description: Option<String>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}

impl Related<super::user::Entity> for Entity {
    // The final relation is Workspace -> WorkspaceUser -> User
    fn to() -> RelationDef {
        super::workspace_user::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::workspace_user::Relation::Workspace.def().rev())
    }
}
