use serde::{Deserialize, Serialize};
use welds::{errors::Result, WeldsModel};

#[derive(Debug, WeldsModel, Serialize, Deserialize)]
#[welds(table = "admin_user")]
#[welds(db(Sqlite, MySql, Postgres))]
#[welds(BeforeCreate(fn_to_call_before_create))]
#[welds(BeforeUpdate(fn_to_call_before_update))]
pub struct AdminUser {
    #[welds(primary_key)]
    pub id: String,
    #[welds(unique)]
    pub email: String,
    pub active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

fn fn_to_call_before_create(state: &mut AdminUser) -> Result<()> {
    state.id = uuid::Uuid::new_v4().to_string();
    state.created_at = chrono::Utc::now().timestamp_millis();
    state.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}

fn fn_to_call_before_update(state: &mut AdminUser) -> Result<()> {
    state.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}
