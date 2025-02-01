use {
    chrono::Utc,
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "workspace_user", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub user_id: Uuid,

    pub workspace_id: Uuid,

    pub invited_by: Option<Uuid>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: chrono::DateTime<Utc>,

    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Workspace,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => {
                Entity::belongs_to(super::user::Entity)
                    .from(Column::UserId)
                    .to(super::user::Column::Id)
                    .into()
            },

            Self::Workspace => {
                Entity::belongs_to(super::workspace::Entity)
                    .from(Column::WorkspaceId)
                    .to(super::workspace::Column::Id)
                    .into()
            },
        }
    }
}
