use clap::Parser;
use rust_control_panel_template::{
    http::run_http_server,
    schema::{self, AdminUser},
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

    /// Database url
    #[arg(env, long, default_value = "sqlite::memory:")]
    database_url: String,
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

    let client: Arc<dyn welds::Client> = if args.database_url.starts_with("sqlite:") {
        let client = welds::connections::sqlite::connect(&args.database_url).await?;
        schema::up_migrations(&client).await?;
        schema::check_tables(&client).await?;
        Arc::from(client)
    } else if args.database_url.starts_with("mysql:") {
        let client = welds::connections::mysql::connect(&args.database_url).await?;
        schema::up_migrations(&client).await?;
        schema::check_tables(&client).await?;
        Arc::from(client)
    } else if args.database_url.starts_with("postgres:") {
        let client = welds::connections::postgres::connect(&args.database_url).await?;
        schema::up_migrations(&client).await?;
        schema::check_tables(&client).await?;
        Arc::from(client)
    } else {
        anyhow::bail!("Unsupported database url: {}", args.database_url)
    };

    let mut admin_user = AdminUser::new();
    admin_user.email = "giang.ndm@gmail.com".to_owned();
    admin_user.active = true;
    admin_user.save(client.as_ref()).await?;

    run_http_server(&args.auth0_domain, &args.auth0_client_id, client).await
}
