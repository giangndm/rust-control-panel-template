use std::sync::Arc;

use rust_control_panel_template::{
    http::run_http_server,
    prisma::{admin_user, PrismaClient},
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = PrismaClient::_builder().build().await?;
    let client = Arc::new(client);

    let _ = client
        .admin_user()
        .create(
            "giang.ndm@gmail.com".to_owned(),
            vec![admin_user::SetParam::SetActive(true)],
        )
        .exec()
        .await;

    run_http_server(
        "dev-tnxbxx784nmjqzaz.us.auth0.com",
        "Rt8hHIFCgUHug4KGXOAR2EPi6fvu1cuM",
        client,
    )
    .await
}
