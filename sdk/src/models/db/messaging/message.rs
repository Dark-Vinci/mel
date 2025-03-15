use {
    crate::models::others::messaging::{CreateMessage, UpdateMessage},
    chrono::{DateTime, Utc},
    sea_orm::{prelude::*, Set},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, DeriveEntityModel,
)]
#[sea_orm(table_name = "messages", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed)]
    pub channel_id: Uuid, // channel - chat id

    pub is_private_message: bool,

    #[sea_orm(indexed)]
    pub workspace_id: Uuid,

    pub created_by: Uuid,

    #[sea_orm(type = "TEXT")]
    pub content: String,

    #[sea_orm(nullable)]
    pub parent_id: Option<Uuid>, // self reference

    pub make_main_at: Option<DateTime<Utc>>, // when it was made to be a part of the main channel

    pub make_main_by: Option<Uuid>, // make it a part of the channel message{we should make main by sender id when the normal message is just created}

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,

    // for the sake of scheduled messaging
    pub activate_at: Option<DateTime<Utc>>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<CreateMessage> for Model {
    fn from(msg: CreateMessage) -> Self {
        let mut value = Model {
            ..Default::default()
        };

        value.channel_id = msg.channel_id.into();
        value.workspace_id = msg.workspace_id.into();
        value.created_by = msg.created_by.into();
        value.content = msg.body.into();
        value.created_at = Utc::now();
        value.updated_at = Utc::now();
        value.deleted_at = None;
        value.id = Uuid::new_v4();
        value.activate_at = msg.activate_at;
        value.is_private_message = msg.is_private_message;

        value
    }
}

impl From<(UpdateMessage, ActiveModel)> for ActiveModel {
    fn from(mut msg: (UpdateMessage, ActiveModel)) -> Self {
        let mut value = msg.1;

        // if value.
        value.updated_at = Set(Utc::now());

        if msg.0.body.is_some() {
            value.content = Set(msg.0.body.unwrap());
        }

        if value.make_main_by.is_set() {
            return value;
        }

        if msg.0.make_main_by.is_some() {
            value.make_main_by = Set(msg.0.make_main_by)
        }

        if msg.0.make_main_at.is_some() {
            value.make_main_at = Set(msg.0.make_main_at)
        }

        value
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity"
        from = "Column::ParentId",
        to = "Column::Id"
    )]
    Response,

    #[sea_orm(has_many = "super::reaction::Entity")]
    Reactions,

    #[sea_orm(has_many = "super::super::extras::short_url::Entity")]
    Links,

    #[sea_orm(has_many = "super::super::extras::chat_media::Entity")]
    Media,
}

// for responses
impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Response.def()
    }
}

//  for reactions
impl Related<super::reaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reactions.def()
    }
}

//  for links
impl Related<super::super::extras::short_url::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Links.def()
    }
}

//  for media
impl Related<super::super::extras::chat_media::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Media.def()
    }
}
