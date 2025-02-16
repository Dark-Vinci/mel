use async_trait::async_trait;

#[async_trait]
pub trait ShortUrl {}

#[async_trait]
pub trait ShortUrlTrack {}

#[async_trait]
pub trait ChatMedia {}

#[async_trait]
pub trait ProfileMedia {}
