use anyhow::anyhow;
use poem::{
    get, handler,
    web::{Data, Json, Path, Query},
    IntoResponse, Route,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::schema::AdminUser;

use super::{to_response, to_response_list, ListQuery};

#[derive(Debug, Deserialize)]
struct CreateParams {
    email: String,
    active: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct UpdateParams {
    active: bool,
}

#[handler]
async fn list(query: Query<ListQuery>, data: Data<&Arc<dyn welds::Client>>) -> impl IntoResponse {
    async fn process(
        query: Query<ListQuery>,
        data: Data<&Arc<dyn welds::Client>>,
    ) -> anyhow::Result<(Vec<AdminUser>, usize)> {
        let users = AdminUser::all()
            .limit(query.end - query.start)
            .offset(query.start)
            .run(data.as_ref())
            .await?
            .into_iter()
            .map(|user| user.into_inner())
            .collect();
        let users_count = AdminUser::all().count(data.as_ref()).await? as usize;
        Ok((users, users_count))
    }
    to_response_list(process(query, data).await)
}

#[handler]
async fn get_one(id: Path<String>, data: Data<&Arc<dyn welds::Client>>) -> impl IntoResponse {
    async fn process(
        id: Path<String>,
        data: Data<&Arc<dyn welds::Client>>,
    ) -> anyhow::Result<AdminUser> {
        let user = AdminUser::find_by_id(data.as_ref(), id.0)
            .await?
            .ok_or(anyhow!("not found"))?;
        Ok(user.into_inner())
    }

    to_response(process(id, data).await)
}

#[handler]
async fn update_one(
    id: Path<String>,
    body: Json<UpdateParams>,
    data: Data<&Arc<dyn welds::Client>>,
) -> impl IntoResponse {
    async fn process(
        id: Path<String>,
        body: Json<UpdateParams>,
        data: Data<&Arc<dyn welds::Client>>,
    ) -> anyhow::Result<AdminUser> {
        let mut row = AdminUser::find_by_id(data.as_ref(), id.0)
            .await?
            .ok_or(anyhow!("not found"))?;

        if body.active != row.active {
            row.active = body.active;
        }
        row.save(data.as_ref()).await?;
        Ok(row.into_inner())
    }

    to_response(process(id, body, data).await)
}

#[handler]
async fn create_one(
    body: Json<CreateParams>,
    data: Data<&Arc<dyn welds::Client>>,
) -> impl IntoResponse {
    async fn process(
        body: Json<CreateParams>,
        data: Data<&Arc<dyn welds::Client>>,
    ) -> anyhow::Result<AdminUser> {
        let mut user = AdminUser::new();
        user.email = body.email.clone();
        user.active = body.active.unwrap_or_default();
        user.save(data.as_ref()).await?;
        Ok(user.into_inner())
    }

    to_response(process(body, data).await)
}

#[handler]
async fn delete_one(id: Path<String>, data: Data<&Arc<dyn welds::Client>>) -> impl IntoResponse {
    async fn process(id: Path<String>, data: Data<&Arc<dyn welds::Client>>) -> anyhow::Result<()> {
        AdminUser::find_by_id(data.as_ref(), id.0)
            .await?
            .ok_or(anyhow!("not found"))?
            .delete(data.as_ref())
            .await?;
        Ok(())
    }

    to_response(process(id, data).await)
}

pub fn build_route() -> Route {
    Route::new()
        .at("/", get(list).post(create_one))
        .at("/:id", get(get_one).patch(update_one).delete(delete_one))
}
