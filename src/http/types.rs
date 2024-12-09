use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::FromRow;
use std::fmt::Formatter;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
pub struct ActivityLogGetRequest {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

#[derive(FromRow, Debug, Serialize)]
pub struct ActivityLogData {
    pub wallet_address: String,
    pub from_token: String,
    pub to_token: String,
    pub percentage: i16,
    pub amount_from: i64,
    pub amount_to: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ActivityLogGetResponse {
    pub transactions: Vec<ActivityLogData>,
    pub next_cursor: Option<String>,
}

#[derive(FromRow, Debug, Serialize)]
pub struct SubscriptionData {
    pub wallet_address: String, 
    pub to_token: String,
    pub is_active: bool
}

#[derive(FromRow, Debug, Serialize)]
pub struct SwapSubscriptionFromTokenData {
    pub wallet_address: String, 
    pub from_token: String,
    pub percentage: i16
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AutoSwapRequest {
    pub from : String,
    pub to : String,
    pub value : i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoSwapResponse {
    pub status: String,
    pub message: String
}

#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub wallet_address: String,
    pub to_token: String,
    pub from_token: Vec<String>,
    pub percentage: Vec<i16>,
}

#[derive(Debug, Serialize)]
pub struct CreateSubscriptionResponse {
    pub wallet_address: String,
}

#[derive(sqlx::Type)]
pub struct TimeStamptz(pub OffsetDateTime);

impl Serialize for TimeStamptz {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0.format(&Rfc3339).map_err(serde::ser::Error::custom)?)
    }
}

impl<'de> Deserialize<'de> for TimeStamptz {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrVisitor;

        impl Visitor<'_> for StrVisitor {
            type Value = TimeStamptz;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.pad("expected string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                OffsetDateTime::parse(v, &Rfc3339)
                    .map(TimeStamptz)
                    .map_err(E::custom)
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}
