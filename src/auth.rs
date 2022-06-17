use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let api_key_validator = request
            .rocket()
            .state::<ApiKeyValidator>()
            .expect("ApiKeyValidator not initialized");

        match request.headers().get_one("X-API-KEY") {
            Some(key) => {
                if api_key_validator.is_valid(key) {
                    Outcome::Success(ApiKey(key))
                } else {
                    Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid))
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
        }
    }
}

pub struct ApiKeyValidator {
    keys: Vec<String>,
}

impl ApiKeyValidator {
    pub fn from_string(str: &str) -> Self {
        ApiKeyValidator {
            keys: str.split(',').map(|a| a.to_owned()).collect(),
        }
    }

    pub fn is_valid(&self, api_key: &str) -> bool {
        self.keys.iter().any(|k| k.as_str() == api_key)
    }
}
