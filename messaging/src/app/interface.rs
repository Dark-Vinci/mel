use async_trait::async_trait;

#[async_trait]
pub trait Message {}

#[async_trait]
pub trait Reaction {}

#[async_trait]
pub trait Response {}

#[async_trait]
pub trait Chat {}
