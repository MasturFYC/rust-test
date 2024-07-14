use bigdecimal::BigDecimal;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::order::{OrderDtos, Order};


#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct OrderDetail {
   #[serde(rename = "orderId")]
   pub order_id: Uuid,
   pub id: Uuid,
   #[serde(rename = "productId")]
   pub product_id: Uuid,
   pub qty: BigDecimal,
   pub direction: i8,
   pub unit: String,
   pub hpp: BigDecimal,
   pub price: BigDecimal,
   pub discount: BigDecimal,
   pub subtotal: BigDecimal,
   #[serde(rename = "createdAt")]
   pub created_at: Option<DateTime<Utc>>,
   #[serde(rename = "updatedAt")]
   pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateOrderDetailSchema {
   #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
   pub order_id: Option<Uuid>,
   #[serde(rename = "productId")]
   pub product_id: i32,
   pub qty: BigDecimal,
   pub direction: i16,
   #[validate(length(min = 1, message = "UNIT is required"))]
   pub unit: String,
   pub hpp: BigDecimal,
   pub price: BigDecimal,
   pub discount: BigDecimal,
   pub subtotal: BigDecimal
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct OrderDetailCreateReturn {
   pub id: Uuid,
   #[serde(rename = "createdAt")]
   pub created_at: Option<DateTime<Utc>>,
   #[serde(rename = "updatedAt")]
   pub updated_at: Option<DateTime<Utc>>, 
}


// type MatchResult = (Vec<Order>, i64);
#[derive(Debug, sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct TrxOrder {
    pub id: Uuid,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

pub type MatchTrxResult = (Option<Order>, Vec<OrderDetail>);
