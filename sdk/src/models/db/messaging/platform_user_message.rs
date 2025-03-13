use {
    chrono::{DateTime, Utc},
    sea_orm::{prelude::*, Set},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};
use crate::models::others::messaging::{CreatePlatformUserMessage, UpdatePlatformUserMessage};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "channel_user_message", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed)]
    pub message_id: Uuid,

    pub is_private_message: bool,

    #[sea_orm(indexed)]
    pub user_id: Uuid,

    #[sea_orm(indexed)]
    pub platform_id: Uuid,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: DateTime<Utc>,

    pub seen_at: Option<DateTime<Utc>>,

    pub deleted_at: Option<DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<CreatePlatformUserMessage> for Model {
    fn from(reaction: CreatePlatformUserMessage) -> Self {
        let mut value = Model {
            ..Default::default()
        };

        value.id = Uuid::new_v4();
        value.user_id = reaction.user_id;
        value.message_id = reaction.message_id;
        value.seen_at = None;
        value.is_private_message = reaction.is_private_message;
        value.created_at = Utc::now();
        value.updated_at = Utc::now();

        value
    }
}

impl From<(UpdatePlatformUserMessage, ActiveModel)> for ActiveModel {
    fn from(value: (UpdatePlatformUserMessage, ActiveModel)) -> Self {
        let reaction = value.0;
        let mut active_model = value.1;

        if reaction.seen {
            active_model.seen_at = Set(Some(Utc::now()));
        } else {
            active_model.seen_at = Set(None);
        }

        active_model.updated_at = Set(Utc::now());

        active_model
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::message::Entity",
        from = "Column::MessageId",
        to = "super::message::Column::Id"
    )]
    Message,

    #[sea_orm(
        belongs_to = "super::super::extras::emoji::Entity",
        from = "Column::EmojiId",
        to = "super::super::extras::emoji::Column::Id"
    )]
    Emoji,
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}
