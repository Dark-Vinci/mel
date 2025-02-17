use {
    crate::models::others::auth::create::{
        CreateUserRequest, UpdateUserRequest,
    },
    chrono::Utc,
    sea_orm::{entity::prelude::*, ActiveValue::Set},
    serde::{Deserialize, Serialize},
};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "users", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub first_name: String,

    pub last_name: String,

    pub date_of_birth: DateTime,

    #[sea_orm(unique, indexed)]
    pub email: String,

    pub password: String,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl From<CreateUserRequest> for ActiveModel {
    fn from(val: CreateUserRequest) -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            first_name: Set(val.first_name),
            last_name: Set(val.last_name),
            date_of_birth: Set(val.date_of_birth),
            email: Set(val.email),
            password: Set(val.password),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            deleted_at: Set(None),
        }
    }
}

impl From<UpdateUserRequest> for ActiveModel {
    fn from(fro: UpdateUserRequest) -> Self {
        let mut val: ActiveModel = Self {
            ..Default::default()
        };

        if let Some(first_name) = &fro.first_name {
            val.first_name = Set(first_name.into());
        }

        // todo: fill other fields

        val
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        super::channel_user::Relation::Channel.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::channel_user::Relation::User.def().rev())
    }
}

impl Related<super::workspace::Entity> for Entity {
    fn to() -> RelationDef {
        super::workspace_user::Relation::Workspace.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::workspace_user::Relation::User.def().rev())
    }
}
