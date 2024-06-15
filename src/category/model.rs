use serde::{Deserialize, Serialize};

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