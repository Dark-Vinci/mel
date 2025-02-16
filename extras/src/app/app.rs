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
            user::{UserRepo, UserRepository},
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
    pub user_repo: Box<dyn UserRepository + Sync + Send>,
    pub short_url_repo: Box<dyn ShortUrlRepository + Sync + Send>,
    pub short_url_track_repo: Box<dyn ShortUrlTrackRepository + Send + Sync>,
}

impl App {
    pub async fn new(c: &Config) -> Self {
        let db = DB::new(&c).await.unwrap();

        let user = UserRepo::new(db.clone());
        let short_repo = ShortUrlRepo::new(db.clone());
        let short_track = ShortUrlTrackRepo::new(db.clone());
        let profile_media_repo = ProfileMediaRepo::new(db.clone());
        let chat_media_repo = ChatMediaRepo::new(db.clone());

        Self {
            db,
            user_repo: Box::new(user),
            config: Config::new(),
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
