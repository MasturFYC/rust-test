use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ledger {
    pub id: Uuid,
    #[serde(rename = "relationId")]
    pub relation_id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateLedgerSchema {
    #[serde(rename = "relationId")]
    pub relation_id: Uuid,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub descriptions: Option<String>,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
}

pub type MatchResult = (Vec<Ledger>, i64);
