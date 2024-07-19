use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use sqlx::{self, types::Json, Row};


// #[serde_as]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Accounts {
    pub id: i16,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<i16>,
    pub normal: i16,
    #[serde(rename = "enName")]
    // #[serde_as(deserialize_as = "DefaultOnNull")]
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde_as(deserialize_as = "DefaultOnNull")]
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
pub struct Account {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i16>,
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


#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "order_enum", rename_all = "snake_case")]
pub enum OrderType {
    Order,
    Stock,
    OrderReturn,
    StockReturn,
    Mutation,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Order
    }
}

#[allow(dead_code)]
impl OrderType {
    pub fn to_str(&self) -> &str {
        match self {
            OrderType::Order => "order",
            OrderType::Stock => "stock",
            OrderType::OrderReturn => "order_return",
            OrderType::StockReturn => "stock_return",
            OrderType::Mutation => "mutation",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "payment_enum", rename_all = "lowercase")]
pub enum PaymentType {
    Cash,
    Pending,
    Loans,
    Lunas,
}

impl Default for PaymentType {
    fn default() -> Self {
        PaymentType::Lunas
    }
}

#[allow(dead_code)]
impl PaymentType {
    pub fn to_str(&self) -> &str {
        match self {
            PaymentType::Cash => "cash",
            PaymentType::Pending => "pending",
            PaymentType::Loans => "loans",
            PaymentType::Lunas => "lunas",
        }
    }
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Order {
    pub id: Uuid,
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    #[serde(rename = "customerId")]
    pub customer_id: Uuid,
    #[serde(rename = "salesId")]
    pub sales_id: Uuid,
    #[serde(rename = "paymentType")]
    pub payment_type: PaymentType,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    pub total: BigDecimal,
    pub dp: BigDecimal,
    pub payment: BigDecimal,
    pub remain: BigDecimal,
    #[serde(rename = "invoiceId")]
    pub invoice_id: Option<String>,
    #[serde(rename = "dueAt")]
    pub due_at: Option<DateTime<Utc>>,
    #[serde(rename = "isProtected")]
    pub is_protected: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct ResponseOrder {
    pub id: Uuid,
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    #[serde(rename = "customerId")]
    pub customer_id: Uuid,
    #[serde(rename = "salesId")]
    pub sales_id: Uuid,
    #[serde(rename = "paymentType")]
    pub payment_type: PaymentType,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    pub total: BigDecimal,
    pub dp: BigDecimal,
    pub payment: BigDecimal,
    pub remain: BigDecimal,
    #[serde(rename = "invoiceId")]
    pub invoice_id: Option<String>,
    #[serde(rename = "dueAt")]
    pub due_at: Option<DateTime<Utc>>,
    #[serde(rename = "isProtected")]
    pub is_protected: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "customerName")]
    pub customer_name: String,
    #[serde(rename = "salesName")]
    pub sales_name: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct OrderDtos {
    #[serde(rename = "orderType")]
    pub order_type: Option<OrderType>,
    #[serde(rename = "customerId")]
    pub customer_id: Uuid,
    #[serde(rename = "salesId")]
    pub sales_id: Uuid,
    #[serde(rename = "paymentType")]
    pub payment_type: Option<PaymentType>,
    #[serde(rename = "updatedBy")]
    #[validate(length(min = 1, message = "USER is required"))]
    pub updated_by: String,
    pub total: BigDecimal,
    pub dp: BigDecimal,
    pub payment: BigDecimal,
    pub remain: BigDecimal,
    #[serde(rename = "invoiceId")]
    pub invoice_id: Option<String>,
    #[serde(rename = "dueRange")]
    pub due_range: Option<u64>,
    #[serde(rename = "dueAt")]
    pub due_at: Option<DateTime<Utc>>,
    #[serde(rename = "isProtected")]
    pub is_protected: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum DetailMark {
    New,
    Update,
    Delete,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateOrderDetailSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<Uuid>,
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    #[serde(rename = "oldProductId")]
    pub old_product_id: Option<Uuid>,
    pub qty: BigDecimal,
    #[serde(rename = "oldQty")]
    pub old_qty: Option<BigDecimal>,
    pub direction: i16,
    #[validate(length(min = 1, message = "UNIT is required"))]
    pub unit: String,
    pub price: BigDecimal,
    pub discount: BigDecimal,
    pub hpp: BigDecimal,
    #[serde(rename = "markAs")]
    pub mark_as: Option<DetailMark>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<BigDecimal>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestQueryOrderDtos {
    pub order: OrderDtos,
    pub details: Vec<CreateOrderDetailSchema>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct OrderDetail {
    #[serde(rename = "orderId")]
    pub order_id: Uuid,
    pub id: Uuid,
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    pub qty: BigDecimal,
    pub direction: i16,
    pub unit: String,
    pub price: BigDecimal,
    pub discount: BigDecimal,
    pub hpp: BigDecimal,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    pub subtotal: BigDecimal,
}

pub struct OrderBuilder {
    pub order_type: OrderType,
    pub customer_id: Uuid,
    pub sales_id: Uuid,
    pub payment_type: PaymentType,
    pub updated_by: String,
    pub total: BigDecimal,
    pub dp: BigDecimal,
    pub payment: BigDecimal,
    pub remain: BigDecimal,
    pub invoice_id: Option<String>,
    pub due_range: Option<u64>,
    pub due_at: Option<DateTime<Utc>>,
    pub is_protected: bool,
    pub created_at: Option<DateTime<Utc>>,
}

impl OrderBuilder {
    pub fn new(
        order_type: OrderType,
        updated_by: String,
        total: BigDecimal,
        payment: BigDecimal,
        is_protected: bool,
        created_at: Option<DateTime<Utc>>,
        invoice_id: Option<String>,
        customer_id: Uuid,
        sales_id: Uuid,
    ) -> OrderBuilder {
        OrderBuilder {
            order_type,
            payment_type: PaymentType::Lunas,
            updated_by,
            total,
            dp: BigDecimal::from(0),
            payment,
            remain: BigDecimal::from(0),
            due_range: Some(7),
            due_at: Some(Utc::now()),
            created_at,
            invoice_id,
            customer_id,
            sales_id,
            is_protected,
        }
    }

    pub fn with_due_range(&mut self, due_range: u64) -> &mut OrderBuilder {
        self.due_range = Some(due_range);
        let now = Some(self.created_at.unwrap_or(Utc::now()));
        self.due_at = match self.payment_type {
            PaymentType::Cash | PaymentType::Lunas => now,
            _ => {
                let date1 = now.unwrap().to_owned();
                let days = chrono::Days::new(self.due_range.unwrap_or(0));
                date1.checked_add_days(days)
            }
        };
        self
    }

    pub fn with_dp(&mut self, dp: BigDecimal) -> &mut OrderBuilder {
        self.dp = dp;
        let total_payment = &self.payment + &self.dp;
        let remain = &self.total - &total_payment;
        let pass = BigDecimal::from_f32(0.0).unwrap();

        let payment_type: PaymentType;

        if total_payment.ge(&self.total) {
            payment_type = PaymentType::Lunas;
        } else if total_payment.lt(&self.total) && total_payment.gt(&pass) {
            payment_type = PaymentType::Pending;
        } else {
            payment_type = PaymentType::Loans;
        }

        self.remain = remain;
        self.payment_type = payment_type;
        self
    }

    pub fn build(&self) -> OrderDtos {
        OrderDtos {
            order_type: Some(self.order_type),
            payment_type: Some(self.payment_type),
            updated_by: self.updated_by.to_owned(),
            total: self.total.to_owned(),
            dp: self.dp.to_owned(),
            payment: self.payment.to_owned(),
            remain: self.remain.to_owned(),
            due_at: self.due_at,
            due_range: self.due_range,
            created_at: self.created_at,
            customer_id: self.customer_id,
            sales_id: self.sales_id,
            invoice_id: self.invoice_id.to_owned(),
            is_protected: self.is_protected,
        }
    }    
}


#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct OrderPayments {
    pub id: Uuid,
    #[serde(rename = "orderId")]
    pub order_id: Uuid,
    pub amount: BigDecimal,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[serde(rename = "viaBy")]
    pub via_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
// #[builder(setter(prefix = "with"), derive(PartialEq))]
pub struct OrderPayment {
    #[serde(rename = "orderId")]
    pub order_id: Uuid,
    pub amount: BigDecimal,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[serde(rename = "viaBy")]
    pub via_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    // #[serde(rename = "createdAt")]
    // pub created_at: Option<DateTime<Utc>>,
    // #[serde(rename = "updatedAt")]
    // pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "relation_enum", rename_all = "lowercase")]
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
    pub relation_type: Vec<RelationType>,
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
    pub relation_type: Vec<RelationType>,
}


#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Moderator,
    User,
}

impl UserRole {
    pub fn to_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Moderator => "moderator",
        }
    }
}

#[derive(Debug, Deserialize, sqlx::FromRow, sqlx::Type, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub photo: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}


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



#[derive(Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct LedgerResult {
    pub id: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
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
    pub details: sqlx::types::Json<Vec<LedgerDetail>>,
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
