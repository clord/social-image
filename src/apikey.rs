use crate::AppConfig;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
    Failure,
}

/// true if `key` is a valid API key string.
fn is_valid(from_config: &str, key: &str) -> bool {
    key == from_config
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let app_key = req
            .rocket()
            .state::<AppConfig>()
            .map(|my_config| &my_config.key);
        match app_key {
            Some(api_key) => match req.headers().get_one("x-api-key") {
                None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
                Some(key) if is_valid(api_key, key) => Outcome::Success(ApiKey(key)),
                Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            },
            None => {
                error!("Failed to get config");
                Outcome::Failure((Status::BadRequest, ApiKeyError::Failure))
            }
        }
    }
}
