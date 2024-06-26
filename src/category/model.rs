use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow, Row};
// use serde_json::from_value;

#[derive(Debug, Deserialize)]
pub struct CreateCategorySchema {
    pub name: String,
}

// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type, Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: i16,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PageOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCategorySchema {
    pub name: Option<String>,
    // pub content: Option<String>,
    // pub category: Option<String>,
    // pub published: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug, FromRow, sqlx::Type)]
pub struct CategoryProduct {
    pub id: i32,
    pub name: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CategoryWithProduct {
    pub id: i16,
    pub name: String,
    pub products: Option<Json<Vec<CategoryProduct>>>,
}

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for CategoryWithProduct {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let id = row.get("id");
        let name = row.get("name");
        // let products = row
        //     .try_get::<sqlx::types::Json<Vec<CategoryProduct>>, _>("products")
        //     .map(|r| if r.is_empty() {None} else { Some (r.0) })
        //     .unwrap_or(None);
        let products = row
            .try_get::<sqlx::types::Json<Vec<CategoryProduct>>, _>("products")
            .map(|x| if x.is_empty() {None} else { Some (x) })
            .unwrap_or(None); // serde_json::from_value(row.get("products")).unwrap_or(None);

        // let products: Option<Vec<CategoryProduct>> = serde_json::from_value(r.get("products")).unwrap_or(None);

        Ok(Self {id, name, products})
    }
}

// impl<'r> FromRow<'r, PgRow> for CategoryProduct {
//     fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
//         let id = row.get("id");
//         let name = row.get("name");
//         Ok(Self {id, name})
//     }
// }