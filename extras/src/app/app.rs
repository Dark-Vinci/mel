use {
    crate::{
        app::interface::{
            ChatMedia, Mailer, ProfileMedia, ShortUrl, ShortUrlTrack,
        },
        config::config::Config,
        connections::db::DB,
        downstream::downstream::{Downstream, DownstreamImpl},
        email::{
            email::{EmailClient, EmailClientConfigs::Smtp, EmailSettings},
            providers::smtp::{SmtpServer, SmtpServerConfig},
        },
        repository::{
            chat_media::{ChatMediaRepo, ChatMediaRepository},
            profile_media::{ProfileMediaRepo, ProfileMediaRepository},
            short_url::{ShortUrlRepo, ShortUrlRepository},
            short_url_track::{ShortUrlTrackRepo, ShortUrlTrackRepository},
        },
    },
    sdk::constants::Boxed,
    uuid::Uuid,
};

pub struct App {
    pub db: DB,
    pub config: Config,
    pub mailer: Boxed<dyn EmailClient<RichText = String>>,
    pub downstream: Boxed<dyn Downstream>,
    pub profile_media_repo: Boxed<dyn ProfileMediaRepository>,
    pub chat_media_repo: Boxed<dyn ChatMediaRepository>,
    pub short_url_repo: Boxed<dyn ShortUrlRepository>,
    pub short_url_track_repo: Boxed<dyn ShortUrlTrackRepository>,
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
                },
            },
        );

        let smtp_config = SmtpServerConfig {
            host: "".to_string(),
            port: 0,
            time_out: 0,
            username: None,
            password: None,
            connection: Default::default(),
        };

        let mailer = SmtpServer::create(&settings, smtp_config);

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

pub trait Operations:
    ShortUrl + ShortUrlTrack + ChatMedia + ProfileMedia + Mailer
{
}

impl Operations for App {}
