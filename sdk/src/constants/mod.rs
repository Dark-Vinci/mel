// pub const request_id: &'static str = "REQUEST_ID";
pub const REQUEST_ID: &'static str = "request_id";
pub const USER_AGENT: &'static str = "USER_AGENT";
pub const AUTH_TOKEN: &'static str = "AUTH_TOKEN";
pub const REFRESH_TOKEN: &'static str = "REFRESH_TOKEN";
pub const TIME_ZONE: &'static str = "TIME_ZONE";
pub const UTC: &'static str = "UTC";
pub const USER_ID: &'static str = "USER_ID";
pub const CHROME: &'static str = "CHROME";

// pub const
pub mod constant;

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Environment {
    Production,
    #[default]
    Development,
    Testing,
}
