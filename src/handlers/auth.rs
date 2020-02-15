use crate::auth::{create_jwt, hash, PrivateClaim};
use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::user::UserResponse;
use crate::helpers::{respond_json, respond_ok};
use crate::models::user::find_by_auth;
use crate::validate::validate;
use actix_identity::Identity;
use actix_web::web::{Data, HttpResponse, Json};
use serde::Serialize;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

/// Login a user
/// Create and remember their JWT
pub async fn login(
    id: Identity,
    pool: Data<PoolType>,
    params: Json<LoginRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    validate(&params)?;

    // Validate that the email + hashed password matches
    let hashed = hash(&params.password);
    let user = find_by_auth(&pool, &params.email, &hashed)?;

    // Create a JWT
    let private_claim = PrivateClaim::new(user.id, user.email.clone());
    let jwt = create_jwt(private_claim)?;

    // Remember the token
    id.remember(jwt);
    respond_json(user.into())
}

/// Logout a user
/// Forget their user_id
pub async fn logout(id: Identity) -> Result<HttpResponse, ApiError> {
    id.forget();
    respond_ok()
}

#[cfg(test)]
pub mod tests {
    // use super::*;
    // use crate::tests::helpers::tests::get_data_pool;
    // use actix_identity::RequestIdentity;
    // use actix_web::test;

    // #[test]
    // fn it_logs_in() {
    //     let email = "abc@123.com".to_string();
    //     let password = "123".to_string();
    //     let params = LoginRequest { email, password };
    //     let request = test::TestRequest::with_header("content-type", "application/json").to_http_request();
    //     let identity = RequestIdentity::get_identity(&request);
    //     let response =
    //         test::block_on(login(NEED_IDENTITY_HERE, get_data_pool(), Json(params))).unwrap();
    //     // assert_eq!(response.into_inner(), *first_user);
    // }
}
