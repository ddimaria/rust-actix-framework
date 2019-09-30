use crate::database::Pool;
use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::models::user::{find_by_id, get_all, User};
use crate::validate::validate;
use actix_web::web::{Data, Json, Path};
use rayon::prelude::*;
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
pub struct UsersResponse(pub Vec<UserResponse>);

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
pub fn get_user(user_id: Path<(Uuid)>, pool: Data<Pool>) -> Result<Json<UserResponse>, ApiError> {
    respond_json(find_by_id(*user_id, &pool)?)
}

/// Get all users
pub fn get_users(pool: Data<Pool>) -> Result<Json<UsersResponse>, ApiError> {
    respond_json(get_all(&pool)?)
}

/// Create a user
pub fn create_user(params: Json<CreateUserRequest>) -> Result<Json<UserResponse>, ApiError> {
    validate(&params)?;
    respond_json::<UserResponse>(params.into())
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: Uuid::parse_str(&user.id).unwrap(),
            first_name: user.first_name.to_string(),
            last_name: user.last_name.to_string(),
            email: user.email.to_string(),
        }
    }
}

impl From<Vec<User>> for UsersResponse {
    fn from(users: Vec<User>) -> Self {
        UsersResponse(users.into_par_iter().map(|user| user.into()).collect())
    }
}

// Quick way to convert a CreateUserRequest to a new UserResponse
// For demonstration purposes only
impl From<Json<CreateUserRequest>> for UserResponse {
    fn from(request: Json<CreateUserRequest>) -> Self {
        UserResponse {
            id: Uuid::new_v4(),
            first_name: request.first_name.to_string(),
            last_name: request.last_name.to_string(),
            email: request.email.to_string(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::database::init_pool;
    use crate::tests::helpers::tests::get_data_pool;
    use actix_web::test;

    pub fn get_all_users() -> UsersResponse {
        let pool = init_pool().unwrap();
        get_all(&pool).unwrap()
    }

    pub fn get_first_users_id() -> Uuid {
        get_all_users().0[0].id
    }

    #[test]
    fn test_get_user() {
        let first_user = &get_all_users().0[0];
        let user_id: Path<(Uuid)> = get_first_users_id().into();
        let response = test::block_on(get_user(user_id, get_data_pool())).unwrap();
        assert_eq!(response.into_inner(), *first_user);
    }

    #[test]
    fn test_get_user_not_found() {
        let uuid = Uuid::new_v4();
        let user_id: Path<(Uuid)> = uuid.into();
        let response = test::block_on(get_user(user_id, get_data_pool()));
        let expected_error = ApiError::NotFound(format!("User {} not found", uuid.to_string()));
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);
    }

    #[test]
    fn test_get_users() {
        let response = test::block_on(get_users(get_data_pool())).unwrap();
        assert_eq!(response.into_inner(), get_all_users());
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
