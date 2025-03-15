use {
    sdk::constants::constant::{
        OBJECT_STORE_ACCESS_KEY_ID, OBJECT_STORE_PROVIDER_NAME,
        OBJECT_STORE_SECRET_ACCESS_KEY, OBJECT_STORE_URL,
    },
    std::env,
};

#[derive(Clone, Debug)]
pub struct ObjectStore {
    pub url: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub provider_name: String,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            url: env::var(OBJECT_STORE_URL).unwrap_or_default(),
            access_key_id: env::var(OBJECT_STORE_ACCESS_KEY_ID)
                .unwrap_or_default(),
            secret_access_key: env::var(OBJECT_STORE_SECRET_ACCESS_KEY)
                .unwrap_or_default(),
            provider_name: env::var(OBJECT_STORE_PROVIDER_NAME)
                .unwrap_or_default(),
        }
    }
}
