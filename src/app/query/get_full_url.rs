// use crate::app::error::AppError;
// use crate::domain::url::url_entity::Url;
// use async_trait::async_trait;
//
// #[async_trait]
// pub trait LoginUserRepository {
//     async fn get_full_url(&self, short_url: String) -> Result<Url, AppError>;
// }
//
// pub struct GetFullUrlQuery<R>
// where
//     R: LoginUserRepository,
// {
//     repo: R,
// }
//
// impl<R> GetFullUrlQuery<R>
// where
//     R: LoginUserRepository,
// {
//     pub fn new(repo: R) -> Self {
//         Self { repo }
//     }
//
//     pub async fn execute(&self, id: String) -> Result<Url, AppError> {
//         self.repo.get_full_url(id).await
//     }
// }
