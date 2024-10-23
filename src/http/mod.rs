use std::sync::Arc;

use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};

mod api_admin;
#[allow(unused)]
mod dev_proxy;

pub async fn run_http_server(
    auth0_domain: &str,
    auth0_client_id: &str,
    client: Arc<dyn welds::Client>,
) -> anyhow::Result<()> {
    let admin = api_admin::build_route(auth0_domain, auth0_client_id, client.clone()).await;

    let route = Route::new()
        .nest("/api/admin-panel/", admin.data(client))
        .nest("/", api_admin::frontend_app())
        .with(Tracing);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(route)
        .await?;
    Ok(())
}
