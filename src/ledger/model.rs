use bigdecimal::BigDecimal;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use sqlx::{types::Json, FromRow, Row};


#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct LedgerDetail {
    #[serde(rename = "ledgerId")]
    pub ledger_id: Uuid,
    pub id: i16,
    #[serde(rename = "accountId")]
    pub account_id: i16,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    pub amount: BigDecimal,
    pub direction: i16,
    #[serde(skip_serializing_if = "Option::is_none", rename = "refId")]
    pub ref_id: Option<Uuid>,
}



#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct LedgerWithDetails {
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
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Json<Vec<LedgerDetail>>,
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
    // pub details: Option<Json<Vec<LedgerDetail>>>,
}


pub type MatchResult = (Vec<Ledger>, i64);


impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for LedgerWithDetails {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let id = row.get("id");
        let relation_id = row.get("relation_id");
        let name = row.get("name");
        let descriptions = row.get("descriptions");
        let is_valid = row.get("is_valid");
        let updated_by = row.get("updated_by");
        let created_at = row.get("created_at");
        let updated_at = row.get("updated_at");
        
        let details = row
            .try_get::<Json<Vec<LedgerDetail>>, _>("details")
            .unwrap();
            //.map(|x| if x.is_empty() {None} else { Some (x) })
            // .unwrap_or(None);

        Ok(Self {id, relation_id, name, descriptions, is_valid, updated_by, created_at, updated_at, details})
    }
}