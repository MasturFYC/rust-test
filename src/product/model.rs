use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateProductSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(rename = "supplierId")]
    pub supplier_id: Uuid,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "Barcode is required"))]
    pub barcode: String,
    #[validate(length(min = 1, message = "Unit is required"))]
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
    // #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    // pub created_at: Option<DateTime<Utc>>,
    // #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    // pub updated_at: Option<DateTime<Utc>>,
}

// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type, Clone)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Product {
    pub id: Uuid,
    #[serde(rename = "supplierId")]
    pub supplier_id: Uuid,
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
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

// #[derive(Debug, Deserialize, Default)]
// pub struct PageOptions {
//     pub page: Option<usize>,
//     pub limit: Option<usize>,
// }

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Products {
    pub id: Uuid,
    #[serde(rename = "supplierId")]
    pub supplier_id: Uuid,
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
    #[serde(rename = "createdAt",skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt",skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "categoryName")]
    pub category_name: String,
    #[serde(rename = "supplierName")]
    pub supplier_name: String,
} 

// #[derive(Serialize, Deserialize, Debug)]
// pub struct UpdateCategorySchema {
//     pub name: Option<String>,
//     // pub content: Option<String>,
//     // pub category: Option<String>,
//     // pub published: Option<bool>,
// }

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductResponseDto {
    pub status: String,
    pub data: Products,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateResponseDto {
    pub status: String,
    pub data: Product,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteResponseDto {
    pub status: String,
    pub data: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductListResponseDto {
    pub status: String,
    pub data: Vec<Products>,
    pub count: usize,
}