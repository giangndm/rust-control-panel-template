use std::sync::Arc;

use auth::{AuthMidleware, AuthMidlewareImpl};
use http::StatusCode;
use poem::{EndpointExt, IntoResponse, Response, Route};
use serde::{Deserialize, Serialize};

use crate::prisma;

mod admin_user;
mod auth;
mod emdedded_files;

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
    auth0_client_id: &str,
    db: Arc<prisma::PrismaClient>,
) -> AuthMidlewareImpl<Route> {
    Route::new()
        .nest("/admin_user", admin_user::build_route())
        .with(AuthMidleware::new(auth0_domain, auth0_client_id, db).await)
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

/// only include in release build
#[cfg(not(debug_assertions))]
#[derive(rust_embed::Embed)]
#[folder = "admin-panel/dist"]
pub struct AdminPanelFiles;

pub fn frontend_app() -> Route {
    #[cfg(debug_assertions)]
    {
        let pconfig = super::dev_proxy::ProxyConfig::new("localhost:5173")
            .web_insecure() // Enables proxy-ing web requests, sets the proxy to use http instead of https
            .enable_nesting() // Sets the proxy to support nested routes
            .finish(); // Finishes constructing the configuration

        // Development mode: spawn Vite dev server
        println!("Running in development mode, starting Vite dev server...");
        std::process::Command::new("pnpm")
            .current_dir("./admin-panel/")
            .args(&["run", "dev"])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("Failed to start Vite dev server");

        // Proxy frontend requests to Vite
        Route::new().nest("/", super::dev_proxy::proxy.data(pconfig)) // You can add your API here
    }

    #[cfg(not(debug_assertions))]
    {
        // Production mode: serve static files
        Route::new()
            .at(
                "/",
                emdedded_files::EmbeddedFileEndpoint::<AdminPanelFiles>::new("index.html"),
            )
            .nest(
                "/",
                emdedded_files::EmbeddedFilesEndpoint::<AdminPanelFiles>::new(),
            )
    }
}
