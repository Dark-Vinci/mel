use {
    crate::models::others::auth::create::{
        CreateUserRequest, UpdateUserRequest,
    },
    sea_orm::{entity::prelude::*, ActiveValue::Set},
    serde::{Deserialize, Serialize},
};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "users", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    id: Uuid,

    pub first_name: String,

    pub last_name: String,

    pub date_of_birth: DateTime,

    #[sea_orm(unique, indexed)]
    pub email: String,

    pub password: String,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTimeLocal,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: DateTimeLocal,

    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTimeLocal>,
}

impl From<CreateUserRequest> for ActiveModel {
    fn from(val: CreateUserRequest) -> Self {
        Self {
            id: Default::default(),
            first_name: Set(val.first_name),
            last_name: Set(val.last_name),
            date_of_birth: Set(val.date_of_birth),
            email: Set(val.email),
            password: Set(val.password),
            created_at: Default::default(),
            updated_at: Default::default(),
            deleted_at: Default::default(),
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
