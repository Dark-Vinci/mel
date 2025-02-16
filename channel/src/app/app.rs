use {
    crate::{
        app::interface::{
            Account, BookMarkTrait, ChannelTrait, ChannelUserTrait, PinTrait,
            Settings,
        },
        config::config::Config,
        connections::db::DB,
        repository::{
            bookmarks::{BookMarkRepo, BookMarkRepository},
            channel::{ChannelRepo, ChannelRepository},
            channel_user::{ChannelUserRepo, ChannelUserRepository},
            pins::{PinRepo, PinRepository},
        },
    },
    uuid::Uuid,
};

pub struct App {
    pub db: DB,
    pub config: Config,
    pub bookmark_repo: Box<dyn BookMarkRepository + Sync + Send>,
    pub pin_repo: Box<dyn PinRepository + Sync + Send>,
    pub channel_repo: Box<dyn ChannelRepository + Sync + Send>,
    pub channel_user_repo: Box<dyn ChannelUserRepository + Sync + Send>,
}

impl App {
    pub async fn new(c: &Config) -> Self {
        let db = DB::new(&c).await.unwrap();

        let pin_repo = PinRepo::new(db.clone());
        let channel_repo = ChannelRepo::new(db.clone());
        let channel_user_repo = ChannelUserRepo::new(db.clone());
        let bookmark_repo = BookMarkRepo::new(db.clone());

        Self {
            db,
            channel_repo: Box::new(channel_repo),
            bookmark_repo: Box::new(bookmark_repo),
            channel_user_repo: Box::new(channel_user_repo),
            pin_repo: Box::new(pin_repo),
            config: Config::new(),
        }
    }
}

impl App {
    pub fn ping(&self, id: Uuid) -> String {
        format!("PING FROM ACCOUNT SERVICE: {}", id)
    }
}

pub trait ChannelInterface:
    ChannelTrait + ChannelUserTrait + Account + Settings + BookMarkTrait + PinTrait
{
}

impl ChannelInterface for App {}
