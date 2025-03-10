use {
    std::fmt::{Display, Formatter},
    uuid::Uuid,
};

#[derive(Debug)]
pub struct CTX<'a> {
    user_agent: &'a str,
    auth_token: Option<&'a str>,
    refresh_token: Option<&'a str>,
    user_id: Option<Uuid>,
    time_zone: &'a str,
    request_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct Ctx {
    pub user_agent: String,
    pub auth_token: Option<String>,
    pub refresh_token: Option<String>,
    pub user_id: Option<Uuid>,
    pub time_zone: String,
    pub request_id: Uuid,
}

impl Ctx {
    pub fn new(
        user_agent: String,
        request_id: Uuid,
        time_zone: String,
        auth_token: Option<String>,
        refresh_token: Option<String>,
        user_id: Option<Uuid>,
    ) -> Self {
        Self {
            user_agent,
            auth_token,
            refresh_token,
            user_id,
            time_zone,
            request_id,
        }
    }
}

impl<'a> Default for CTX<'a> {
    fn default() -> Self {
        Self {
            user_agent: "",
            auth_token: None,
            refresh_token: None,
            user_id: None,
            time_zone: "",
            request_id: Default::default(),
        }
    }
}

impl Display for CTX<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "context")
    }
}

impl<'a> CTX<'a> {
    pub fn new(
        user_agent: &'a str,
        auth_token: Option<&'a str>,
        refresh_token: Option<&'a str>,
    ) -> Self {
        Self {
            user_agent,
            auth_token,
            refresh_token,
            user_id: None,
            time_zone: "",
            request_id: Default::default(),
        }
    }

    pub fn get_user_id(&self) -> Option<Uuid> {
        self.user_id
    }

    pub fn get_auth_token(&self) -> Option<&'a str> {
        self.auth_token
    }

    pub fn get_refresh_token(&self) -> Option<&'a str> {
        self.refresh_token
    }

    pub fn get_user_agent(&self) -> &'a str {
        self.user_agent
    }
}
