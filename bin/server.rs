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

    /// Auth0 audience
    #[arg(env, long)]
    auth0_audience: String,

    /// Database url
    #[arg(env, long, default_value = "sqlite::memory:")]
    database_url: String,

    /// Default admin-user email
    #[arg(env, long)]
    default_admin_email: String,
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

    if AdminUser::all().count(client.as_ref()).await? == 0 {
        let mut admin_user = AdminUser::new();
        admin_user.email = args.default_admin_email;
        admin_user.active = true;
        admin_user.save(client.as_ref()).await?;
    }

    run_http_server(
        &args.auth0_domain,
        &args.auth0_client_id,
        &args.auth0_audience,
        client,
    )
    .await
}
