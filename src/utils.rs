use anyhow::Error;
use rocket::http::Status;

pub mod opt_chrono_datetime_as_bson_datetime {
    use bson::serde_helpers::chrono_datetime_as_bson_datetime;
    use chrono::Utc;
    use mongodb::bson;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    struct Helper(#[serde(with = "chrono_datetime_as_bson_datetime")] chrono::DateTime<Utc>);

    pub fn serialize<S>(
        value: &Option<chrono::DateTime<Utc>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<chrono::DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper: Option<Helper> = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

pub fn handle_error(e: Error) -> Status {
    error_!("{}", e);
    Status::InternalServerError
}
