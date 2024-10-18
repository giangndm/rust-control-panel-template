use std::sync::Arc;

use poem::{EndpointExt, Route};
use serde::Deserialize;

use crate::prisma;

mod user;

#[derive(Debug, Deserialize)]
enum ListOrder {
    #[serde(rename = "ASC")]
    Asc,
    #[serde(rename = "DESC")]
    Desc,
}

#[derive(Debug, Deserialize)]
struct ListQuery<Fields> {
    #[serde(rename = "_start")]
    pub start: i64,
    #[serde(rename = "_end")]
    pub end: i64,
    #[serde(rename = "_order")]
    pub order: ListOrder,
    #[serde(rename = "_sort")]
    pub sort: Fields,
}

pub fn build_route() -> Route {
    Route::new().nest("/users", user::build_route())
}
