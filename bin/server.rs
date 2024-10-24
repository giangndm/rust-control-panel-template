use clap::Parser;
use rust_control_panel_template::{
    http::run_http_server,
    prisma::{admin_user, PrismaClient},
};
use std::sync::Arc;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Demo control panel server with Rust and Refine.dev
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Auth0 domain
    #[arg(env, long)]
    auth0_domain: String,

    /// Auth0 client_id
    #[arg(env, long)]
    auth0_client_id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    let args = Args::parse();
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

    run_http_server(&args.auth0_domain, &args.auth0_client_id, client).await
}
