use {
    crate::{
        app::interface::{ChatMedia, ProfileMedia, ShortUrl, ShortUrlTrack},
        config::config::Config,
        connections::db::DB,
        downstream::downstream::{Downstream, DownstreamImpl},
        repository::{
            chat_media::{ChatMediaRepo, ChatMediaRepository},
            profile_media::{ProfileMediaRepo, ProfileMediaRepository},
            short_url::{ShortUrlRepo, ShortUrlRepository},
            short_url_track::{ShortUrlTrackRepo, ShortUrlTrackRepository},
        },
    },
    uuid::Uuid,
};
use crate::email::email::{EmailClient, EmailClientConfigs, EmailSettings};
use crate::email::email::EmailClientConfigs::Smtp;
use crate::email::providers::smtp::{SmtpServer, SmtpServerConfig};

pub struct App {
    pub db: DB,
    pub config: Config,
    pub mailer: Box<dyn EmailClient<RichText=String> + Sync + Send>,
    pub downstream: Box<dyn Downstream + Sync + Send>,
    pub profile_media_repo: Box<dyn ProfileMediaRepository + Sync + Send>,
    pub chat_media_repo: Box<dyn ChatMediaRepository + Sync + Send>,
    pub short_url_repo: Box<dyn ShortUrlRepository + Sync + Send>,
    pub short_url_track_repo: Box<dyn ShortUrlTrackRepository + Send + Sync>,
}

impl App {
    pub async fn new(c: &Config) -> Self {
        let db = DB::new(&c).await.unwrap();

        let short_repo = ShortUrlRepo::new(db.clone());
        let short_track = ShortUrlTrackRepo::new(db.clone());
        let profile_media_repo = ProfileMediaRepo::new(db.clone());
        let chat_media_repo = ChatMediaRepo::new(db.clone());

        let settings = EmailSettings::new(
            "".to_string(),
            0,
            Smtp {
                smtp: SmtpServerConfig {
                    host: "".to_string(),
                    port: 0,
                    time_out: 0,
                    username: None,
                    password: None,
                    connection: Default::default(),
                }
            }
        );

        let smpt_config = SmtpServerConfig{
            host: "".to_string(),
            port: 0,
            time_out: 0,
            username: None,
            password: None,
            connection: Default::default(),
        };

        let mailer = SmtpServer::create(&settings, smpt_config);

        Self {
            db,
            config: Config::new(),
            mailer: Box::new(mailer),
            downstream: Box::new(DownstreamImpl::new()),
            short_url_repo: Box::new(short_repo),
            short_url_track_repo: Box::new(short_track),
            profile_media_repo: Box::new(profile_media_repo),
            chat_media_repo: Box::new(chat_media_repo),
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
