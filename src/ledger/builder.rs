use bigdecimal::BigDecimal;
use uuid::Uuid;

use super::{LedgerDetail, LedgerSchema, LedgerType};

// /// 106 - persediaan barang
// const ACC_INVENTORY: i16 = 0x6A;
// /// 521 - biaya beli barang
// const ACC_GOODS_COST: i16 = 0x209;
// /// 421 - penjualan barang
// // const ACC_REVENUE: i16 = 0x1A5;
// /// 111 - piutang penjualan
// const ACC_PIUTANG: i16 = 0x6F;
// /// 111 - kas
// const ACC_KAS: i16 = 0x65;

#[derive(Copy, Clone)]
pub enum Coa {
    /// 106 - persediaan barang
    Inventory = 0x6A,
    /// 521 - biaya beli barang
    GoodCost = 0x209,
    /// 421 - penjualan barang
    Revenue = 0x1A5,
    /// 111 - piutang penjualan
    Loan = 0x6F,
    /// 111 - kas
    Cash = 0x65,
}

impl Into<i16> for Coa {
    fn into(self) -> i16 {
        self as i16
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct LedgerDetailBuilder {
    ledger_id: Option<Uuid>,
    id: Option<i16>,
    account_id: Option<i16>,
    descriptions: Option<String>,
    amount: Option<BigDecimal>,
    direction: Option<i16>,
    ref_id: Option<Uuid>,
}
impl LedgerDetailBuilder {
    pub fn with_ledger_id<T: Into<Uuid>>(mut self, value: T) -> LedgerDetailBuilder {
        self.ledger_id = Some(value.into());
        self
    }
    pub fn with_id<T: Into<i16>>(mut self, value: T) -> LedgerDetailBuilder {
        self.id = Some(value.into());
        self
    }
    pub fn with_account_id<T: Into<i16>>(mut self, value: T) ->  LedgerDetailBuilder {
        self.account_id = Some(value.into());
        self
    }
    pub fn with_descriptions<T: Into<String>>(mut self, value: T) ->  LedgerDetailBuilder {
        self.descriptions = Some(value.into());
        self
    }
    pub fn with_amount<T: Into<BigDecimal>>(mut self, value: T) ->  LedgerDetailBuilder {
        self.amount = Some(value.into());
        self
    }
    pub fn with_direction<T: Into<i16>>(mut self, value: T) ->  LedgerDetailBuilder {
        self.direction = Some(value.into());
        self
    }
    pub fn with_ref_id<T: Into<Uuid>>(mut self, value: T) ->  LedgerDetailBuilder {
        self.ref_id = Some(value.into());
        self
    }
    pub fn build(&self) -> LedgerDetail {
        LedgerDetail {
            ledger_id: self.ledger_id.expect("ledger_id not define"),
            id: self.id.expect("id not define"),
            account_id: self.account_id.expect("account_id not define"),
            descriptions: self.descriptions.to_owned(),
            amount: self.amount.to_owned().unwrap(), //.unwrap_or(bigdecimal::BigDecimal::from(0)), //.to_owned(),// .expect("amount not define"),
            direction: self.direction.expect("direction must be 1 o r -1"),
            ref_id: self.ref_id,
        }
    }
}

#[derive(Clone, Default)]
pub struct LedgerBuilder {
    pub relation_id: Option<Uuid>,
    pub ledger_type: Option<LedgerType>,
    pub is_valid: Option<bool>,
    pub updated_by: Option<String>,
    pub descriptions: Option<String>,
}

impl LedgerBuilder {
    pub fn relation_id<T: Into<Uuid>>(mut self, value: T) -> LedgerBuilder {
        self.relation_id = Some(value.into());
        self
    }
    pub fn ledger_type<T: Into<LedgerType>>(mut self, value: T) -> LedgerBuilder {
        self.ledger_type = Some(value.into());
        self
    }
    pub fn is_valid<T: Into<bool>>(mut self, value: T) -> LedgerBuilder {
        self.is_valid = Some(value.into());
        self
    }
    pub fn updated_by<T: Into<String>>(mut self, value: T) -> LedgerBuilder {
        self.updated_by = Some(value.into());
        self
    }
    pub fn descriptions<T: Into<String>>(mut self, value: T) -> LedgerBuilder {
        self.descriptions = Some(value.into());
        self
    }

    pub fn build(&self) -> LedgerSchema {
        LedgerSchema {
            relation_id: self.relation_id.expect("relation_id not define"),
            ledger_type: self.ledger_type.expect("ledger_type not define"),
            is_valid: self.is_valid.unwrap_or(false),
            updated_by: self.updated_by.to_owned().expect("updater not define"),
            descriptions: self.descriptions.to_owned(),
        }
    }
}
