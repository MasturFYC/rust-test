use bigdecimal::BigDecimal;
use chrono::prelude::*;
//use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct OrderPayments {
    pub id: Uuid,
    #[serde(rename = "orderId")]
    pub order_id: Uuid,
    pub amount: BigDecimal,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[serde(rename = "viaBy")]
    pub via_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
// #[builder(setter(prefix = "with"), derive(PartialEq))]
pub struct OrderPayment {
    #[serde(rename = "orderId")]
    pub order_id: Uuid,
    pub amount: BigDecimal,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[serde(rename = "viaBy")]
    pub via_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    // #[serde(rename = "createdAt")]
    // pub created_at: Option<DateTime<Utc>>,
    // #[serde(rename = "updatedAt")]
    // pub updated_at: Option<DateTime<Utc>>,
}