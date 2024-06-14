use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::models::Category;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}


#[derive(Serialize, Deserialize, Validate, IntoParams)]
// Request query dto
pub struct RequestCategoryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryResponseDto {
    pub status: String,
    pub data: Category,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryListResponseDto {
    pub status: String,
    pub categories: Vec<Category>,
    pub results: usize,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}
