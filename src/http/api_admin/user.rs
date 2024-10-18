use std::sync::Arc;

use http::StatusCode;
use poem::{
    get, handler, post,
    web::{Data, Json, Query},
    IntoResponse, Response, Route,
};
use serde::Deserialize;

use crate::prisma::PrismaClient;

use super::ListQuery;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Fields {
    Id,
    DisplayName,
}

#[handler]
async fn list(
    query: Query<ListQuery<Fields>>,
    data: Data<&Arc<PrismaClient>>,
) -> impl IntoResponse {
    let users = data
        .user()
        .find_many(vec![])
        .skip(query.start)
        .take(query.end - query.start)
        .exec()
        .await
        .unwrap();
    let users_count = data.user().count(vec![]).exec().await.unwrap();
    let vec = serde_json::to_vec(&users).expect("should convert to json");
    Response::builder()
        .header("X-Total-Count", users_count)
        .header("content-type", "application/json; charset=utf-8")
        .body(vec)
}

pub fn build_route() -> Route {
    Route::new().at("/", get(list))
}
