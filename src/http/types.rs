use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::FromRow;
use std::fmt::Formatter;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
pub struct ActivityLogGetRequest {
    pub wallet_address: Option<String>,
    pub from_token: Option<String>,
    pub to_token: Option<String>,
    pub amount_to: Option<i64>,
    pub cursor: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct AutoSwapRequest {
    pub from: String,
    pub to: String,
    pub value: i64,
}

#[derive(Debug, Serialize)]
pub struct AutoSwapResponse {
    pub message: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Encode, Decode)]
pub struct PoolKey {
    token0: Felt,
    token1: Felt,
    fee: u128,
    tick_spacing: u128,
    extension: Felt,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct RouteNode {
    pub pool_key: PoolKey,
    pub sqrt_ratio_limit: U256,
    pub skip_ahead: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct TokenAmount {
    pub token: Felt,
    pub amount: i129,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct Swap {
    pub route: Vec<RouteNode>,
    pub token_amount: TokenAmount,
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

#[derive(Debug, Deserialize)]
pub struct UpdatePercentageRequest {
    pub wallet_address: String,
    pub from_token: String,
    pub percentage: i16,
}

#[derive(Debug, Serialize)]
pub struct UpdatePercentageResponse {
    pub message: String,
}
