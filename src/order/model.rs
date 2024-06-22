use bigdecimal::BigDecimal;
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
    #[serde(rename = "relationId")]
    pub relation_id: Uuid,
    #[serde(rename = "paymentType")]
    pub payment_type: PaymentType,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    pub total: BigDecimal,
    pub payment: BigDecimal,
    pub remain: BigDecimal,
    #[serde(rename = "invoiceId")]
    pub invoice_id: Option<String>,
    #[serde(rename = "dueAt")]
    pub due_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateOrderSchema {
    #[serde(rename = "orderType")]
    pub order_type: Option<OrderType>,
    #[serde(rename = "relationId")]
    pub relation_id: Uuid,
    #[serde(rename = "paymentType")]
    pub payment_type: Option<PaymentType>,
    #[serde(rename = "updatedBy")]
    #[validate(length(min = 1, message = "USER is required"))]
    pub updated_by: String,
    pub total: BigDecimal,
    pub payment: BigDecimal,
    pub remain: BigDecimal,
    #[serde(rename = "invoiceId")]
    pub invoice_id: Option<String>,
    #[serde(rename = "duedAt")]
    pub due_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
}

impl CreateOrderSchema {

    pub fn set_default(data: &CreateOrderSchema) -> Self {
        // let date1 = data.created_at.unwrap();
        // let days: Days = Days::new(7);
        // date1.checked_add_days(days);
        let t = data.total.to_owned();
        let p = data.payment.to_owned();
        let r = t - p;
        let date = match data.payment_type.unwrap() {
            PaymentType::Cash | PaymentType::Lunas => data.created_at,
            _ => {
                let date1 = data.created_at.unwrap();
                let days: chrono::Days = chrono::Days::new(7);
                date1.checked_add_days(days)
            }
        };

        CreateOrderSchema {
            order_type: data.order_type.to_owned(),
            relation_id: data.relation_id.to_owned(),
            payment_type: data.payment_type.to_owned(),
            updated_by: data.updated_by.to_owned(),
            total: data.total.to_owned(),
            payment: data.payment.to_owned(),
            remain: r.to_owned(),
            invoice_id: data.invoice_id.to_owned(),
            created_at: data.created_at.to_owned(),
            due_at: date
        }

    }
    #[allow(dead_code)]
    pub fn set_defaults(data: &[CreateOrderSchema]) -> Vec<CreateOrderSchema> {
        data.iter().map(CreateOrderSchema::set_default).collect()
    }
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
}
