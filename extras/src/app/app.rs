use {
    crate::{
        app::interface::{ChatMedia, ProfileMedia, ShortUrl, ShortUrlTrack},
        config::config::Config,
        connections::db::DB,
        downstream::downstream::{Downstream, DownstreamImpl},
        repository::{
            audit_logs::{AuditLogRepository, AuditLogsRepo},
            chat_media::{ChatMediaRepo, ChatMediaRepository},
            emails::{EmailRepository, EmailRepo},
            history::{HistoryRepo, HistoryRepository},
            profile_media::{ProfileMediaRepo, ProfileMediaRepository},
            search::{SearchRepo, SearchRepository},
            short_url::{ShortUrlRepo, ShortUrlRepository},
            short_url_track::{ShortUrlTrackRepo, ShortUrlTrackRepository},
        },
    },
    uuid::Uuid,
};

pub struct App {
    pub db: DB,
    pub config: Config,
    pub downstream: Box<dyn Downstream + Sync + Send>,
    pub profile_media_repo: Box<dyn ProfileMediaRepository + Sync + Send>,
    pub chat_media_repo: Box<dyn ChatMediaRepository + Sync + Send>,
    pub short_url_repo: Box<dyn ShortUrlRepository + Sync + Send>,
    pub search_repo: Box<dyn SearchRepository + Sync + Send>,
    pub short_url_track_repo: Box<dyn ShortUrlTrackRepository + Send + Sync>,
    pub history_repo: Box<dyn HistoryRepository + Sync + Send>,
    pub audit_log_repo: Box<dyn AuditLogRepository + Sync + Send>,
    pub email_repo: Box<dyn EmailRepository + Sync + Send>,
}

impl App {
    pub async fn new(c: &Config) -> Self {
        let db = DB::new(&c).await.unwrap();

        let short_repo = ShortUrlRepo::new(db.clone());
        let short_track = ShortUrlTrackRepo::new(db.clone());
        let profile_media_repo = ProfileMediaRepo::new(db.clone());
        let chat_media_repo = ChatMediaRepo::new(db.clone());
        let search_repo = SearchRepo::new(db.clone());
        let history_repo = HistoryRepo::new(db.clone());
        let audit_log_repo = AuditLogsRepo::new(db.clone());
        let email_repo = EmailRepo::new(db.clone());

        Self {
            db,
            config: Config::new(),
            downstream: Box::new(DownstreamImpl::new()),
            short_url_repo: Box::new(short_repo),
            search_repo: Box::new(search_repo),
            short_url_track_repo: Box::new(short_track),
            profile_media_repo: Box::new(profile_media_repo),
            chat_media_repo: Box::new(chat_media_repo),
            history_repo: Box::new(history_repo),
            audit_log_repo: Box::new(audit_log_repo),
            email_repo: Box::new(email_repo),
        }
    }
}

impl App {
    pub fn ping(&self, id: Uuid) -> String {
        format!("PING FROM ACCOUNT SERVICE: {}", id)
    }
}

pub trait ExtrasInterface:
    ShortUrl + ShortUrlTrack + ChatMedia + ProfileMedia
{
}

impl ExtrasInterface for App {}
