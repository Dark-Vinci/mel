use {
    crate::downstream::extras::extras::{Extras, ExtrasOperations},
    async_trait::async_trait,
};

#[async_trait]
impl ExtrasOperations for Extras {}
