use crate::database::Pool;
use crate::errors::ApiError;
use crate::handlers::user::{UserResponse, UsersResponse};
use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

// Get all users
pub fn get_all(pool: &Pool) -> Result<UsersResponse, ApiError> {
    use crate::schema::users::dsl::users;

    let conn = &pool.get().unwrap();
    let all_users = users.load::<User>(conn)?;

    Ok(all_users.into())
}

// Find a user by the user's id or error out
pub fn find_by_id(user_id: Uuid, pool: &Pool) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{id, users};

    let not_found = format!("User {} not found", user_id);
    let conn = &pool.get().unwrap();
    let user = users
        .filter(id.eq(user_id.to_string()))
        .first::<User>(conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(user.into())
}
