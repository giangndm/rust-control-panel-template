use std::{
    process::{Command, Stdio},
    sync::Arc,
};

use dev_proxy::{proxy, ProxyConfig};
use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};

use crate::prisma;

mod api_admin;
#[allow(unused)]
mod dev_proxy;

pub async fn run_http_server(
    auth0_domain: &str,
    auth0_audien: &str,
    client: Arc<prisma::PrismaClient>,
) -> anyhow::Result<()> {
    let admin = api_admin::build_route(auth0_domain, auth0_audien, client.clone()).await;

    let frontend_app = if cfg!(debug_assertions) {
        let pconfig = ProxyConfig::new("localhost:5173")
            .web_insecure() // Enables proxy-ing web requests, sets the proxy to use http instead of https
            .enable_nesting() // Sets the proxy to support nested routes
            .finish(); // Finishes constructing the configuration

        // Development mode: spawn Vite dev server
        println!("Running in development mode, starting Vite dev server...");
        Command::new("pnpm")
            .current_dir("./admin-panel/")
            .args(&["run", "dev"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start Vite dev server");

        // Proxy frontend requests to Vite
        Route::new().nest("/", proxy.data(pconfig)) // You can add your API here
    } else {
        todo!()
    };

    let route = Route::new()
        .nest("/api/admin-panel/", admin.data(client))
        .nest("/", frontend_app)
        .with(Tracing);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(route)
        .await?;
    Ok(())
}
