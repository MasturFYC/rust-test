use bigdecimal::BigDecimal;
use chrono::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow, Row};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "ledger_enum", rename_all = "lowercase")]
pub enum LedgerType {
    Order,
    Stock,
    OrderReturn,
    StockReturn,
    Loan,
}

#[allow(dead_code)]
impl LedgerType {
    pub fn to_str(&self) -> &str {
        match self {
            LedgerType::Order => "order",
            LedgerType::Stock => "stock",
            LedgerType::OrderReturn => "orderreturn",
            LedgerType::StockReturn => "stockreturn",
            LedgerType::Loan => "loan",
        }
    }
}

impl Default for LedgerType {
    fn default() -> Self {
        LedgerType::Order
    }
}

#[derive(Builder, Debug, Deserialize, Serialize, FromRow, Clone)]
#[builder(derive(PartialEq))]
pub struct LedgerDetail {
    
    #[builder(setter(prefix = "with"))]
    #[serde(rename = "ledgerId")]
    pub ledger_id: Uuid,
    
    #[builder(setter(prefix = "with", into))]
    pub id: i16,
    
    #[serde(rename = "accountId")]
    #[builder(setter(prefix = "with", into))]
    pub account_id: i16,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(prefix = "with", into, strip_option))]
    pub descriptions: Option<String>,    
    
    /// nominal
    #[builder(setter(prefix = "with"))]
    pub amount: BigDecimal,

    #[builder(setter(prefix = "with"))]
    pub direction: i16,
    
    #[serde(skip_serializing_if = "Option::is_none", rename = "refId")]
    #[builder(setter(prefix = "with"))]
    pub ref_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct LedgerResult {
    pub id: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct Ledger {
    pub id: Uuid,
    #[serde(rename = "relationId")]
    pub relation_id: Uuid,
    #[serde(rename = "ledgerType")]
    pub ledger_type: LedgerType,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
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
    #[serde(rename = "ledgerType")]
    pub ledger_type: LedgerType,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Json<Vec<LedgerDetail>>,
}

#[derive(Builder, Debug, Validate, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct LedgerSchema {
    #[serde(rename = "relationId")]
    pub relation_id: Uuid,
    #[serde(rename = "LedgerType")]
    pub ledger_type: LedgerType,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
}

pub type MatchResult = (Vec<Ledger>, i64);

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for LedgerWithDetails {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let id = row.get("id");
        let relation_id = row.get("relation_id");
        let ledger_type = row.get("ledger_type");
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

        Ok(Self {
            id,
            relation_id,
            ledger_type,
            descriptions,
            is_valid,
            updated_by,
            created_at,
            updated_at,
            details,
        })
    }
}
