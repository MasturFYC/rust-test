use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "order_enum", rename_all = "lowercase")]
pub enum OrderType {
    Order,
    Stock,
    OrderReturn,
    StockReturn,
    Mutation,
}

#[allow(dead_code)]
impl OrderType {
    pub fn to_str(&self) -> &str {
        match self {
            OrderType::Order => "order",
            OrderType::Stock => "stock",
            OrderType::OrderReturn => "orderreturn",
            OrderType::StockReturn => "stockreturn",
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

impl OrderDtos {
    
    pub fn set_total(&mut self, total: &BigDecimal) {
        self.total = total.to_owned();
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
        self.payment_type = Some(payment_type);
    }

    pub fn set_due_date(&mut self) {
        let now = Some(self.created_at.unwrap_or(Utc::now()));
        self.due_at = match self.payment_type.unwrap() {
            PaymentType::Cash | PaymentType::Lunas => now,
            _ => {
                let date1 = now.unwrap().to_owned();
                let days = chrono::Days::new(self.due_range.unwrap_or(0));
                date1.checked_add_days(days)
            }
        };
    }
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
   pub details: Vec<CreateOrderDetailSchema>
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

pub type MatchTrxResult = (Option<Order>, Vec<OrderDetail>);
pub type MatchResult = (Vec<ResponseOrder>, i64);

    // #[allow(dead_code)]
    // pub fn set_default(data: &OrderDtos) -> Self {
    //     // let date1 = data.created_at.unwrap();
    //     // let days: Days = Days::new(7);
    //     // date1.checked_add_days(days);
    //     let t = data.total.to_owned();
    //     let p = data.payment.to_owned();
    //     let r = t - p;
    //     let now = Some(data.created_at.unwrap_or(Utc::now()));
    //     let date = match data.payment_type.unwrap() {
    //         PaymentType::Cash | PaymentType::Lunas => now,
    //         _ => {
    //             let date1 = now.unwrap().to_owned();
    //             let days = chrono::Days::new(data.due_range.unwrap_or(0));
    //             date1.checked_add_days(days)
    //         }
    //     };

    //     OrderDtos {
    //         // order_type: data.order_type.to_owned(),
    //         // relation_id: data.relation_id.to_owned(),
    //         // payment_type: data.payment_type.to_owned(),
    //         // updated_by: data.updated_by.to_owned(),
    //         // total: data.total.to_owned(),
    //         // payment: data.payment.to_owned(),
    //         remain: r.to_owned(),
    //         // invoice_id: data.invoice_id.to_owned(),
    //         created_at: now.to_owned(),
    //         due_at: date.to_owned(),
    //         // due_range: data.due_range
    //         ..data.clone()
    //     }
    // }

    // #[allow(dead_code)]
    // pub fn set_defaults(mut data: Vec<OrderDtos>) { // &[OrderDtos]) { //-> Vec<OrderDtos> {

    //     for e in data.iter_mut() {
    //         e.set_remain();
    //     }
    //     // data.iter()
    //         // .map(|&mut e| -> e.set_remain()); // OrderDtos::set_default)
    //         //.collect()
    //     // data.iter().map(|d| CreateOrderSchema::set_default(d)).collect()
    // }
    // fn get_due_at(&mut self) -> Option<DateTime<Utc>> {
    //     // let test = self.payment_type.unwrap();

    //     match self.payment_type.unwrap() {
    //         PaymentType::Cash => self.created_at,
    //         _ => {
    //             let date1 = self.created_at.unwrap();
    //             let days: Days = Days::new(7);
    //             date1.checked_add_days(days);
    //             Some(date1)
    //         }
    //     }

    //     // if test == PaymentType::Pending
    //     //     || test == PaymentType::Loans
    //     // {
    //     //     let date1 = self.created_at.unwrap();
    //     //     let days: Days = Days::new(7);
    //     //     date1.checked_add_days(days);
    //     //     Some(date1)
    //     // } else {
    //     //     self.created_at
    //     // }
    // }

    // fn get_remain (&mut self) -> BigDecimal {
    //     &self.total - &self.payment
    // }
    //}