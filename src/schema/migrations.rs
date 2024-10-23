use welds::check::Level;
use welds::errors::Result;
use welds::migrations::prelude::*;

use super::AdminUser;

pub async fn check_tables(client: &dyn welds::Client) -> anyhow::Result<()> {
    let mut has_error = false;
    let diff = welds::check::schema::<AdminUser>(client).await?;
    for d in diff {
        if d.level == Level::Critical {
            log::warn!("{}", d);
            has_error = true;
        }
    }
    if has_error {
        anyhow::bail!("Schema mismatch");
    }
    Ok(())
}

pub async fn up_migrations(client: &dyn welds::TransactStart) -> anyhow::Result<()> {
    up(client, &[m20241020_create_admin_user]).await?;
    Ok(())
}

fn m20241020_create_admin_user(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("admin_user")
        .id(|c| c("id", Type::Uuid))
        .column(|c| c("email", Type::String))
        .column(|c| c("active", Type::Bool))
        .column(|c| c("created_at", Type::IntBig))
        .column(|c| c("updated_at", Type::IntBig));
    Ok(MigrationStep::new("create admin_user", m))
}
