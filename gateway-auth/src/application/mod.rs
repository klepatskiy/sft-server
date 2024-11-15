pub mod command;
pub mod query;
pub mod error;

use async_trait::async_trait;
use shaku::Interface;
use crate::application::error::AppError;

#[async_trait]
pub trait CommandHandler<C, R>: Interface + Send + Sync {
    async fn handle(&self, command: C) -> Result<R, AppError>;
}

#[async_trait]
pub trait QueryHandler<Q, R>: Interface + Send + Sync {
    async fn handle(&self, query: Q) -> Result<R, AppError>;
}
