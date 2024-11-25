use std::sync::Arc;

use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, web::Data, EndpointExt, IntoResponse,
    Response, Route, Server,
};
use serde::Serialize;

mod api_admin;
#[allow(unused)]
mod dev_proxy;

#[derive(Debug, Clone, Serialize)]
struct Auth0Config {
    domain: String,
    client_id: String,
    audience: String,
}

#[handler]
async fn auth0_config(Data(data): Data<&Auth0Config>) -> impl IntoResponse {
    Response::builder()
        .header("content-type", "application/json; charset=utf-8")
        .body(serde_json::to_vec(data).expect("should convert to json"))
}

pub async fn run_http_server(
    auth0_domain: &str,
    auth0_client_id: &str,
    auth0_audience: &str,
    client: Arc<dyn welds::Client>,
) -> anyhow::Result<()> {
    let admin = api_admin::build_route(auth0_domain, auth0_client_id, client.clone()).await;

    let route = Route::new()
        .at(
            "/api/auth0-config",
            get(auth0_config).data(Auth0Config {
                domain: auth0_domain.to_string(),
                client_id: auth0_client_id.to_string(),
                audience: auth0_audience.to_string(),
            }),
        )
        .nest("/api/admin-panel/", admin.data(client))
        .nest("/", api_admin::frontend_app())
        .with(Tracing);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(route)
        .await?;
    Ok(())
}
