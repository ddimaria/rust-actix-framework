use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::models::user::{find_by_id, get_data};
use crate::validate::validate;
use actix_web::web::{Json, Path};
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UsersResponse(Vec<UserResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(
        min = 3,
        message = "first_name is required and must be at least 3 characters"
    ))]
    pub first_name: String,

    #[validate(length(
        min = 3,
        message = "last_name is required and must be at least 3 characters"
    ))]
    pub last_name: String,

    #[validate(email(message = "email must be a valid email"))]
    pub email: String,
}

/// Get a user
pub fn get_user(user_id: Path<(Uuid)>) -> Result<Json<UserResponse>, ApiError> {
    find_by_id(*user_id)
}

/// Get all users
pub fn get_users() -> Result<Json<UsersResponse>, ApiError> {
    respond_json(UsersResponse(get_data()))
}

/// Create a user
pub fn create_user(params: Json<CreateUserRequest>) -> Result<Json<UserResponse>, ApiError> {
    validate(&params)?;
    respond_json::<UserResponse>(params.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    #[test]
    fn test_get_user() {
        let user_id: Path<(Uuid)> = get_data()[0].id.into();
        let response = test::block_on(get_user(user_id)).unwrap();
        assert_eq!(response.into_inner(), get_data()[0]);
    }

    #[test]
    fn test_get_user_not_found() {
        let uuid = Uuid::new_v4();
        let user_id: Path<(Uuid)> = uuid.into();
        let response = test::block_on(get_user(user_id));
        let expected_error = ApiError::NotFound(format!("User {} not found", uuid.to_string()));
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);
    }

    #[test]
    fn test_get_users() {
        let response = test::block_on(get_users()).unwrap();
        assert_eq!(response.into_inner(), UsersResponse(get_data()));
    }

    #[test]
    fn test_create_users() {
        let params = Json(CreateUserRequest {
            first_name: "Satoshi".into(),
            last_name: "Nakamoto".into(),
            email: "satoshi@nakamotoinstitute.org".into(),
        });
        let response = test::block_on(create_user(Json(params.clone()))).unwrap();
        assert_eq!(response.into_inner().first_name, params.first_name);
    }
}
