use std::sync::Arc;

use http::StatusCode;
use poem::{Endpoint, Error, IntoResponse, Middleware, Request, Response, Result};
use serde::Deserialize;

use crate::schema::AdminUser;

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct JwtParams {
    sub: String,
    nickname: Option<String>,
    email: Option<String>,
    email_verified: Option<bool>,
}

pub struct AuthMidleware {
    db: Arc<dyn welds::Client>,
    jwks: Arc<jwks::Jwks>,
    auth0_client_id: String,
}

impl AuthMidleware {
    pub async fn new(
        auth0_domain: &str,
        auth0_client_id: &str,
        db: Arc<dyn welds::Client>,
    ) -> Self {
        let jwks =
            jwks::Jwks::from_jwks_url(format!("https://{auth0_domain}/.well-known/jwks.json"))
                .await
                .expect("should get jwks");

        Self {
            db,
            jwks: jwks.into(),
            auth0_client_id: auth0_client_id.to_owned(),
        }
    }
}

impl<E: Endpoint> Middleware<E> for AuthMidleware {
    type Output = AuthMidlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthMidlewareImpl {
            endpoint: ep,
            db: self.db.clone(),
            jwks: self.jwks.clone(),
            auth0_client_id: self.auth0_client_id.clone(),
        }
    }
}

pub struct AuthMidlewareImpl<E> {
    endpoint: E,
    db: Arc<dyn welds::Client>,
    jwks: Arc<jwks::Jwks>,
    auth0_client_id: String,
}

impl<E: Endpoint> Endpoint for AuthMidlewareImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let token = req
            .headers()
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                Error::from_string("Missing authorization header", StatusCode::BAD_REQUEST)
            })?;

        token
            .starts_with("Bearer ")
            .then_some(())
            .ok_or(Error::from_string(
                "Not Bearer token",
                StatusCode::BAD_REQUEST,
            ))?;
        let token = &token[7..];

        log::info!("[AuthMidlewareImpl] got token {token}");

        let header = jsonwebtoken::decode_header(token)
            .map_err(|e| Error::from_string(e.to_string(), StatusCode::BAD_REQUEST))?;

        log::info!("[AuthMidlewareImpl] got token header {header:?}");

        let kid = header.kid.as_ref().ok_or(Error::from_string(
            "Token header missing kid",
            StatusCode::BAD_REQUEST,
        ))?;

        let jwk = self.jwks.keys.get(kid).ok_or(Error::from_string(
            "Kid not found in Jwks",
            StatusCode::BAD_REQUEST,
        ))?;

        let mut validation = jsonwebtoken::Validation::new(header.alg);
        validation.set_audience(&[&self.auth0_client_id]);

        let decoded_token =
            jsonwebtoken::decode::<JwtParams>(token, &jwk.decoding_key, &validation)
                .map_err(|e| Error::from_string(e.to_string(), StatusCode::BAD_REQUEST))?;

        let email = decoded_token.claims.email.ok_or(Error::from_string(
            "Missing Email".to_string(),
            StatusCode::BAD_REQUEST,
        ))?;

        let users = AdminUser::where_col(|col| col.email.equal(email.clone()))
            .run(self.db.as_ref())
            .await
            .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

        if users.is_empty() {
            return Err(Error::from_string(
                "user not found",
                StatusCode::BAD_REQUEST,
            ));
        }

        let user = users.first().expect("should have at least one user");

        if !user.active {
            return Err(Error::from_string(
                "user not actived",
                StatusCode::BAD_REQUEST,
            ));
        }

        let res = self.endpoint.call(req).await;
        match res {
            Ok(resp) => {
                let resp = resp.into_response();
                Ok(resp)
            }
            Err(err) => Err(err),
        }
    }
}
