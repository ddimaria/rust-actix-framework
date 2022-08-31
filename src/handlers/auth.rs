use axum::extract::Extension;
use axum::{http::StatusCode, response::IntoResponse, Json};
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use validator::Validate;

// use crate::auth::{create_jwt, hash, PrivateClaim};
use crate::auth::hash;
use crate::database::Pool;
use crate::errors::ApiError;
use crate::handlers::user::UserResponse;
use crate::models::user::get_user_by_auth;
use crate::validate::validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginRouteParams {
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

/// Login a user
pub async fn login(
    Json(payload): Json<LoginRouteParams>,
    Extension(pool): Extension<Pool<PgConnection>>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("Log in request for user: {}", payload.email);
    validate(&payload)?;
    // Validate that the email + hashed password matches
    let hashed = hash(&payload.password);
    let user = get_user_by_auth(pool, &payload.email, &hashed).await?;

    // You can save additional session info like this
    // session.insert("user_id", user.id).unwrap();
    Ok((StatusCode::OK, Json::<UserResponse>(user)))
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use crate::tests::helpers::tests::get_data_pool;
//     use actix_identity::Identity;
//     use actix_web::{test, FromRequest};

//     async fn get_identity() -> Identity {
//         let (request, mut payload) =
//             test::TestRequest::with_header("content-type", "application/json").to_http_parts();
//         let identity = Option::<Identity>::from_request(&request, &mut payload)
//             .await
//             .unwrap()
//             .unwrap();
//         identity
//     }

//     async fn login_user() -> Result<Json<UserResponse>, ApiError> {
//         let params = LoginRequest {
//             email: "satoshi@nakamotoinstitute.org".into(),
//             password: "123456".into(),
//         };
//         let identity = get_identity().await;
//         login(identity, get_data_pool(), Json(params)).await
//     }

//     #[actix_rt::test]
//     async fn it_logs_a_user_in() {
//         let response = login_user().await;
//         assert!(response.is_ok());
//     }
// }
