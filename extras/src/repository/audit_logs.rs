use sea_orm::{ActiveModelTrait, DbErr};
use tracing::{debug, error};
use uuid::Uuid;
use {crate::connections::db::DB, async_trait::async_trait};
use sdk::models::others::extras::CreateAuditLogs;

use  sdk::{
    errors::RepoError,
    models::{
        db::extras::audit_logs::{
            ActiveModel, Column, Entity as AuditLogEntity, Model as AuditLog,
        },
    },
};

#[async_trait]
pub trait AuditLogRepository {
    async fn create(&self, payload: CreateAuditLogs, request_id: Uuid) -> Result<AuditLog, RepoError>;
}

pub struct AuditLogsRepo(DB);

impl AuditLogsRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl AuditLogRepository for AuditLogsRepo {
    #[tracing::instrument(name = "ShortUrlRepo::get_by_id", skip(self))]
    async fn create(&self, payload: CreateAuditLogs, request_id: Uuid) -> Result<AuditLog, RepoError> {
        debug!(
            request_id = request_id,
            "Got a request to create audit logs"
        );

        let audit_log: ActiveModel = payload.into();

        let result = audit_log.insert(&self.0.connection).await.map_err(|err| {
            error!(
                display = %err,
                debug = ?err,
                "Unable to create audit log"
            );

            if err == DbErr::RecordNotInserted {
                return RepoError::FailedToInsert;
            }

            return RepoError::SomethingWentWrong;
        })?;


        Ok(result)
    }
}
