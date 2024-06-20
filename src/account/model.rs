use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Account {
    pub id: i16,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<i16>,
    pub normal: i16,
    #[serde(rename = "enName")]
    pub en_name: Option<String>,
    pub descriptions: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub payable: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,    
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccountSchema {
    pub id: i16,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<i16>,
    pub normal: i16,
    #[serde(rename = "enName",skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub payable: bool,
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAccountSchema {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<i16>,
    pub normal: i16,
    #[serde(rename = "enName",skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub payable: bool,
}
