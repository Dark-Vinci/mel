// pub const request_id: &'static str = "REQUEST_ID";
pub const REQUEST_ID: &'static str = "request_id";

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Environment {
    Production,
    #[default]
    Development,
    Testing,
}
