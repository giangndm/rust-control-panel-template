use std::sync::Arc;

use auth::{AuthMidleware, AuthMidlewareImpl};
use http::StatusCode;
use poem::{EndpointExt, IntoResponse, Response, Route};
use serde::{Deserialize, Serialize};

use crate::prisma;

mod admin_user;
mod auth;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct ListQuery {
    #[serde(rename = "_start")]
    pub start: i64,
    #[serde(rename = "_end")]
    pub end: i64,
}

pub async fn build_route(
    auth0_domain: &str,
    auth0_audience: &str,
    db: Arc<prisma::PrismaClient>,
) -> AuthMidlewareImpl<Route> {
    Route::new()
        .nest("/admin_user", admin_user::build_route())
        .with(AuthMidleware::new(auth0_domain, auth0_audience, db).await)
}

fn to_response<E: Serialize>(response: anyhow::Result<E>) -> impl IntoResponse {
    match response {
        Ok(res) => Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .body(serde_json::to_vec(&res).expect("should convert to json")),
        Err(err) => Response::builder()
            .header("content-type", "plain-text")
            .status(StatusCode::BAD_REQUEST)
            .body(err.to_string()),
    }
}

fn to_response_list<E: Serialize>(response: anyhow::Result<(Vec<E>, usize)>) -> impl IntoResponse {
    match response {
        Ok((res, total)) => Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .header("X-Total-Count", total)
            .body(serde_json::to_vec(&res).expect("should convert to json")),
        Err(err) => Response::builder()
            .header("content-type", "plain-text")
            .status(StatusCode::BAD_REQUEST)
            .body(err.to_string()),
    }
}
