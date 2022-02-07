use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::Deserialize;
use rocket::Config;

pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
    Failure,
}

#[derive(Deserialize)]
struct AppConfig {
    key: String,
}

/// true if `key` is a valid API key string.
fn is_valid(from_config: &str, key: &str) -> bool {
    key == from_config
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match Config::figment().extract::<AppConfig>() {
            Ok(app_config) => match req.headers().get_one("x-api-key") {
                None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
                Some(key) if is_valid(&app_config.key, key) => Outcome::Success(ApiKey(key)),
                Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            },
            Err(e) => {
                error!("Failed to get config: {e:?}");
                Outcome::Failure((Status::InternalServerError, ApiKeyError::Failure))
            }
        }
    }
}
