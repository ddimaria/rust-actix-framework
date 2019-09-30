use crate::errors::ApiError;
use crate::handlers::user::{CreateUserRequest, UserResponse};
use crate::helpers::respond_json;
use actix_web::web::Json;
use uuid::Uuid;

// A vector of users
// For illustration purposes only
// Should be replaced with some data fetch (DB)
pub fn get_data() -> Vec<UserResponse> {
  vec![
    UserResponse {
      id: Uuid::parse_str("a421a56e-8652-4da6-90ee-59dfebb9d1b4").unwrap(),
      first_name: "Satoshi".into(),
      last_name: "Nakamoto".into(),
      email: "satoshi@nakamotoinstitute.org".into(),
    },
    UserResponse {
      id: Uuid::parse_str("c63d285b-7794-4419-bfb7-86d7bb3ff17d").unwrap(),
      first_name: "Barbara".into(),
      last_name: "Liskov".into(),
      email: "bliskov@substitution.org".into(),
    },
  ]
}

// Find a user by the user's id or error out
pub fn find_by_id(user_id: Uuid) -> Result<Json<UserResponse>, ApiError> {
  let not_found = format!("User {} not found", user_id);

  match get_data().into_iter().find(|user| user.id == user_id) {
    Some(user) => respond_json(user),
    None => Err(ApiError::NotFound(not_found)),
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
