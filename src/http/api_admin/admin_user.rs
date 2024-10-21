use crate::prisma::{
    admin_user::{Data as TableData, SetParam, UniqueWhereParam, WhereParam},
    read_filters::StringFilter,
    PrismaClient,
};
use anyhow::anyhow;
use poem::{
    get, handler,
    web::{Data, Json, Path, Query},
    IntoResponse, Route,
};
use serde::Deserialize;
use std::sync::Arc;

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
async fn list(query: Query<ListQuery>, data: Data<&Arc<PrismaClient>>) -> impl IntoResponse {
    async fn process(
        query: Query<ListQuery>,
        data: Data<&Arc<PrismaClient>>,
    ) -> anyhow::Result<(Vec<TableData>, usize)> {
        let users = data
            .admin_user()
            .find_many(vec![])
            .skip(query.start)
            .take(query.end - query.start)
            .exec()
            .await?;
        let users_count = data.admin_user().count(vec![]).exec().await? as usize;
        Ok((users, users_count))
    }
    to_response_list(process(query, data).await)
}

#[handler]
async fn get_one(id: Path<String>, data: Data<&Arc<PrismaClient>>) -> impl IntoResponse {
    async fn process(
        id: Path<String>,
        data: Data<&Arc<PrismaClient>>,
    ) -> anyhow::Result<TableData> {
        Ok(data
            .admin_user()
            .find_first(vec![WhereParam::Id(StringFilter::Equals(id.0))])
            .exec()
            .await?
            .ok_or(anyhow!("not found"))?)
    }

    to_response(process(id, data).await)
}

#[handler]
async fn update_one(
    id: Path<String>,
    body: Json<UpdateParams>,
    data: Data<&Arc<PrismaClient>>,
) -> impl IntoResponse {
    async fn process(
        id: Path<String>,
        body: Json<UpdateParams>,
        data: Data<&Arc<PrismaClient>>,
    ) -> anyhow::Result<TableData> {
        let row = data
            .admin_user()
            .find_first(vec![WhereParam::Id(StringFilter::Equals(id.0.clone()))])
            .exec()
            .await?
            .ok_or(anyhow!("not found"))?;

        let mut updates = vec![];

        if body.active != row.active {
            updates.push(SetParam::SetActive(body.active));
        }

        let res = data
            .admin_user()
            .update(UniqueWhereParam::IdEquals(id.0), updates)
            .exec()
            .await?;
        Ok(res)
    }

    to_response(process(id, body, data).await)
}

#[handler]
async fn create_one(body: Json<CreateParams>, data: Data<&Arc<PrismaClient>>) -> impl IntoResponse {
    async fn process(
        body: Json<CreateParams>,
        data: Data<&Arc<PrismaClient>>,
    ) -> anyhow::Result<TableData> {
        Ok(data
            .admin_user()
            .create(
                body.email.clone(),
                vec![SetParam::SetActive(body.0.active.unwrap_or_default())],
            )
            .exec()
            .await?)
    }

    to_response(process(body, data).await)
}

#[handler]
async fn delete_one(id: Path<String>, data: Data<&Arc<PrismaClient>>) -> impl IntoResponse {
    async fn process(
        id: Path<String>,
        data: Data<&Arc<PrismaClient>>,
    ) -> anyhow::Result<TableData> {
        Ok(data
            .admin_user()
            .delete(UniqueWhereParam::IdEquals(id.0))
            .exec()
            .await?)
    }

    to_response(process(id, data).await)
}

pub fn build_route() -> Route {
    Route::new()
        .at("/", get(list).post(create_one))
        .at("/:id", get(get_one).patch(update_one).delete(delete_one))
}
