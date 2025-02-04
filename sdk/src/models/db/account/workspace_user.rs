use sea_orm::ActiveValue::Set;
use {
    chrono::Utc,
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};
use crate::models::others::auth::create::{CreateUserRequest, UpdateUserRequest};
use crate::models::others::auth::workspace::{CreateWorkspaceUser, UpdateWorkspaceUser};

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

    #[sea_orm(nullable)]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Workspace,
}

impl From<CreateWorkspaceUser> for ActiveModel {
    fn from(val: CreateWorkspaceUser) -> Self {
        todo!()
    }
}

impl From<UpdateWorkspaceUser> for ActiveModel {
    fn from(fro: UpdateWorkspaceUser) -> Self {
        todo!()
    }
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
