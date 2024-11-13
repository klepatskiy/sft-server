pub mod command;
pub mod query;

use async_trait::async_trait;
use shaku::Interface;

#[async_trait]
pub trait CommandHandler<C, R>: Interface + Send + Sync{
    async fn handle(&self, command: C) -> Result<R, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait QueryHandler<Q, R>: Interface + Send + Sync{
    async fn handle(&self, query: Q) -> Result<R, Box<dyn std::error::Error>>;
}
