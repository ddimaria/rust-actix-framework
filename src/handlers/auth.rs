use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::user::UserResponse;
use crate::helpers::respond_json;
use crate::models::{auth::hash, user::find_by_auth};
use crate::validate::validate;
use actix_identity::Identity;
use actix_web::web::{Data, Json};
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

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {}

/// Login a user
pub fn login(
    id: Identity,
    pool: Data<PoolType>,
    params: Json<LoginRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    validate(&params)?;

    let hashed = hash(&params.password);
    let user = find_by_auth(&pool, &params.email, &hashed)?;

    id.remember(user.id.to_string());
    respond_json(user.into())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::get_data_pool;
    use actix_web::test;

    // #[test]
    // fn it_logs_in() {
    //     let email = "abc@123.com".to_string();
    //     let password = "123".to_string();
    //     let params = LoginRequest { email, password };
    //     let request =
    //         TestRequest::with_header("content-type", "application/json").to_http_request();
    //     // let identity = RequestIdentity::get_identity("test");
    //     let response =
    //         test::block_on(login(request.into(), get_data_pool(), Json(params))).unwrap();
    //     // assert_eq!(response.into_inner(), *first_user);
    // }
}
