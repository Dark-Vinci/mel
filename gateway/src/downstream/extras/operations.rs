use async_trait::async_trait;
use crate::downstream::extras::extras::{Extras, ExtrasOperations};

#[async_trait]
impl ExtrasOperations for Extras {}