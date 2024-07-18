use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// #[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
// #[serde(rename_all = "snake_case")]
#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "ledger_enum", rename_all = "snake_case")]
pub enum LedgerType {
    Order,
    Stock,
    OrderReturn,
    StockReturn,
    Loan,
    OrderPayment,
    StockPaymnent,
}

impl Default for LedgerType {
    fn default() -> Self {
        LedgerType::Order
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

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct LedgerSchema {
    /// customer, supplier, employee
    #[serde(rename = "relationId")]
    pub relation_id: Uuid,
    #[serde(rename = "LedgerType")]
    pub ledger_type: LedgerType,
    #[serde(rename = "isValid")]
    pub is_valid: bool,

    /// active user login
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LedgerDetail {
    #[serde(rename = "ledgerId")]
    pub ledger_id: Uuid,

    pub id: i16,

    #[serde(rename = "accountId")]
    pub account_id: i16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,

    /// nominal transaksi
    pub amount: BigDecimal,

    pub direction: i16,
    /// diambil dari no id transaksi lain
    #[serde(skip_serializing_if = "Option::is_none", rename = "refId")]
    pub ref_id: Option<Uuid>,
}

// #[derive(Debug, Clone, Copy)]
// pub enum MixedInts {
//     Fromi16(i16),
//     Fromi32(i32),
// }

// impl Into<i16> for MixedInts {
//     fn into(self) -> i16 {
//         match self {
//             MixedInts::Fromi16(value) => value,
//             MixedInts::Fromi32(value) => value as i16
//         }
//     }
// }

// impl From<i32> for MixedInts {
//     fn from(value: i32) -> MixedInts {
//         MixedInts::Fromi32(value)
//     }
// }

// impl From<i16> for MixedInts {
//     fn from(value: i16) -> MixedInts {
//         MixedInts::Fromi16(value)
//     }
// }

pub trait Direction {
    fn into_i16(&self) -> i16;
}

impl Direction for i32 {
    fn into_i16(&self) -> i16 {
        *self as i16
    }
}

impl Direction for i16 {
    fn into_i16(&self) -> i16 {
        *self
    }
}

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

impl From<Coa> for i16 {
    fn from(val: Coa) -> Self {
        val as i16
    }
}
/*
impl Into<i16> for Coa {
    fn into(self) -> i16 {
        self as i16
    }
}
*/

#[derive(Clone, Default, Debug, PartialEq)]
pub struct LedgerDetailBuilder {
    pub ledger_id: Option<Uuid>,
    pub id: Option<i16>,
    pub account_id: Option<i16>,
    pub descriptions: Option<String>,
    pub amount: Option<BigDecimal>,
    pub direction: Option<i16>,
    pub ref_id: Option<Uuid>,
}


impl LedgerDetailBuilder {
    pub fn with_ledger_id<T: Into<Uuid>>(mut self, value: T) -> LedgerDetailBuilder {
       self.ledger_id = Some(value.into());
       self
   }
   pub fn with_id<T>(mut self, value: T) -> LedgerDetailBuilder 
   where 
       T: Direction
   {
       // let mi: MixedInts = Into::into(value);
       self.id = Some(value.into_i16()); 
       self
   }
   pub fn with_account_id<T: Into<i16>>(mut self, value: T) -> LedgerDetailBuilder {
       self.account_id = Some(value.into());
       self
   }
   pub fn with_descriptions<T: Into<String>>(mut self, value: T) -> LedgerDetailBuilder {
       self.descriptions = Some(value.into());
       self
   }
   pub fn with_amount<T: Into<BigDecimal>>(mut self, value: T) -> LedgerDetailBuilder {
       self.amount = Some(value.into());
       self
   }

   pub fn with_direction<T>(mut self, value: T) -> LedgerDetailBuilder
   where
       T: Direction,
   {
       // let v = value.into().into();

       self.direction = Some(value.into_i16());
       self
   }
   pub fn with_ref_id<T: Into<Uuid>>(mut self, value: T) -> LedgerDetailBuilder {
       self.ref_id = Some(value.into());
       self
   }
   pub fn build(&self) -> LedgerDetail {
       LedgerDetail {
           ledger_id: self.ledger_id.expect("ledger_id not define"),
           id: self.id.expect("id not define"),
           account_id: self.account_id.expect("account_id not define"),
           descriptions: self.descriptions.to_owned(),
           amount: self
               .amount
               .to_owned()
               .unwrap_or(bigdecimal::BigDecimal::from(0)), //.to_owned(),// .expect("amount not define"),
           direction: self.direction.expect("direction must be 1 o r -1"),
           ref_id: self.ref_id,
       }
   }
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
           relation_id: self.relation_id.expect("relation_id not initialize"),
           ledger_type: self.ledger_type.expect("ledger_type not define"),
           is_valid: self.is_valid.unwrap_or(false),
           updated_by: self.updated_by.to_owned().expect("updater not define"),
           descriptions: self.descriptions.to_owned(),
       }
   }
}

pub struct LedgerUtil {}
impl LedgerUtil {
    /// ## Ledger untuk transaksi order
    /// `(-) 106 - Persediaan barang` -> **hpp**
    ///
    /// `(+) 521 - Biaya beli barang` -> **hpp**
    ///
    /// `(-) 421 - Penjualan barang` -> **total**
    ///
    /// Tidak digunakan jika tidak ada pembayaran sama sekali dan diganti dengan
    /// *`111 - Piutang barang`*.
    ///
    /// `(+) 101 - Kas` -> **dp**
    ///
    /// Jika pembayaran tidak lunas atau tidak ada pembayaran sama sekali.
    /// Catat sisa pembayaran ke dalam piutang,
    /// data diambil dari total order dikurangi dp.
    /// Tidak digunakan jika *`dp`* lebih besar atau sama dengan *`total`*
    ///
    /// `(+) 111 - Piutang barang` -> (**total - dp**)
    ///
    /// **`ref_id`** order id
    ///
    /// **`ledger_id`** biasanya sama dengan order id
    ///    
     pub fn from_order(
        total: &BigDecimal,
        dp: &BigDecimal,
        hpp: &BigDecimal,
        ref_id: Uuid,
        ledger_id: Uuid,
    ) -> (Vec<LedgerDetail>, usize) {
        let mut details: Vec<LedgerDetail> = Vec::new();
        let mut i: i16 = 1;
        let remain = total - dp;
        let pass = bigdecimal::BigDecimal::from(0);

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Revenue)
            .with_direction(-1)
            .with_amount(total.to_owned())
            .with_descriptions("Penjualan barang")
            .build();

        details.push(detail);

        if remain.le(&pass) {
            i += 1;

            let detail = LedgerDetailBuilder::default()
                .with_ref_id(ref_id)
                .with_ledger_id(ledger_id)
                .with_id(i)
                .with_account_id(Coa::Cash)
                .with_direction(1)
                .with_amount(total.to_owned())
                .with_descriptions("Cash payment")
                .build();

            details.push(detail);
        } else {
            // sisa pembayaran
            i += 1;
            let detail = LedgerDetailBuilder::default()
                .with_ref_id(ref_id)
                .with_ledger_id(ledger_id)
                .with_id(i)
                .with_account_id(Coa::Loan)
                .with_direction(1)
                .with_amount(remain)
                .with_descriptions("Piutang barang")
                .build();

            details.push(detail);

            // jika ada pembayaran
            if dp.gt(&pass) {
                i += 1;
                let detail = LedgerDetailBuilder::default()
                    .with_ref_id(ref_id)
                    .with_ledger_id(ledger_id)
                    .with_id(i)
                    .with_account_id(Coa::Cash)
                    .with_direction(1)
                    .with_amount(dp.to_owned())
                    .with_descriptions("Cash DP")
                    .build();

                details.push(detail);
            }
        }

        i += 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Inventory)
            .with_direction(-1)
            .with_amount(hpp.to_owned())
            .with_descriptions("Persediaan barang")
            .build();

        details.push(detail);

        i += 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::GoodCost)
            .with_direction(1)
            .with_amount(hpp.to_owned())
            .with_descriptions("Biaya Beli Barang")
            .build();

        details.push(detail);

        (details, i as usize)
    }

    /// ## Ledger untuk transaksi pembayaran piutang
    ///
    /// `(+) 101 - Kas` -> **amount**
    ///
    /// `(-) 111 - Piutang barang` -> **amount**
    ///
    /// **`ref_id`** payment id
    ///
    /// **`ledger_id`** ledger id biasanya sama dengan payment id
    ///

    pub fn from_order_payment(
        amount: &BigDecimal,
        ref_id: Uuid,
        ledger_id: Uuid,
    ) -> (Vec<LedgerDetail>, usize) {
        let mut details: Vec<LedgerDetail> = Vec::new();
        let mut i: i16 = 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Cash)
            .with_direction(1)
            .with_amount(amount.to_owned())
            .with_descriptions("Titip bayar")
            .build();

        details.push(detail);

        i += 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Loan)
            .with_direction(-1)
            .with_amount(amount.to_owned())
            .with_descriptions("Piutang penjualan")
            .build();

        details.push(detail);

        (details, i as usize)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     pub fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
