use {
    chrono::{DateTime, Utc},
    sea_orm::{
        ActiveModelBehavior, DeriveActiveEnum, DeriveEntityModel,
        DeriveRelation, EnumIter,
    },
    serde::{Deserialize, Serialize},
    uuid::Uuid,
    crate::models::others::extras::CreateAuditLogs
};

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
enum Status {
    #[sea_orm(string_value = "success")]
    Success,
    #[sea_orm(string_value = "error")]
    Error,
    #[sea_orm(string_value = "failure")]
    Failure,
}

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "audit_logs", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub user_id: Uuid,

    pub action: String,

    pub ip_address: String,

    pub user_agent: String,

    pub comment: String,

    pub object_type: String,

    pub object_id: Uuid,

    pub status: Status,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}

impl From<CreateAuditLogs> for ActiveModel {
    fn from(_value: CreateAuditLogs) -> Self {
        todo!()
    }
}
