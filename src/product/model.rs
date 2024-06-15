use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;

#[derive(Debug, Deserialize)]
pub struct CreateProductSchema {
    pub name: String,
    pub barcode: String,
    pub unit: String,
    pub content: BigDecimal,
    pub hpp: BigDecimal,
    pub margin: BigDecimal,
    pub price: BigDecimal,
    pub ppn: BigDecimal,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "variantName",skip_serializing_if = "Option::is_none")]
    pub variant_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    #[serde(rename = "categoryId")]
    pub category_id: i16,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type, Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub barcode: String,
    pub unit: String,
    #[serde(rename = "unitInStock")]
    pub unit_in_stock: BigDecimal,
    pub content: BigDecimal,
    pub hpp: BigDecimal,
    pub margin: BigDecimal,
    pub price: BigDecimal,
    pub ppn: BigDecimal,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "variantName")]
    pub variant_name: Option<String>,
    pub descriptions:  Option<String>,
    #[serde(rename = "categoryId")]
    pub category_id: i16,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Default)]
pub struct PageOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct UpdateCategorySchema {
//     pub name: Option<String>,
//     // pub content: Option<String>,
//     // pub category: Option<String>,
//     // pub published: Option<bool>,
// }