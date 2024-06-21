use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "relation_type", rename_all = "lowercase")]
pub enum RelationType {
    Customer,
    Employee,
    Member,
    Supplier,
}

impl RelationType {
    pub fn to_str(&self) -> &str {
        match self {
            RelationType::Customer => "customer",
            RelationType::Employee => "employee",
            RelationType::Member => "member",
            RelationType::Supplier => "supplier"
        }
    }
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Relation {
    pub id: uuid::Uuid,
    pub name: String,
    pub city: String,
    pub street: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isSpecial")]
    pub is_special: bool,
    #[serde(rename = "relationType")]
    pub rel_type: Option<Vec<RelationType>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateRelationSchema {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "City is required"))]
    pub city: String,
    pub street: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isSpecial")]
    pub is_special: bool,
    #[serde(rename = "relationType")]
    #[validate(length(min = 1, message = "Type is required"))]
    pub rel_type: Option<Vec<RelationType>>,
}
