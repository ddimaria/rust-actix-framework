use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::user::{UserResponse, UsersResponse};
use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Identifiable, Insertable)]
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
pub fn get_all(pool: &PoolType) -> Result<UsersResponse, ApiError> {
    use crate::schema::users::dsl::users;

    let conn = pool.get()?;
    let all_users = users.load(&conn)?;

    Ok(all_users.into())
}

// Find a user by the user's id or error out
pub fn find_by_id(user_id: Uuid, pool: &PoolType) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{id, users};

    let not_found = format!("User {} not found", user_id);
    let conn = pool.get()?;
    let user = users
        .filter(id.eq(user_id.to_string()))
        .first::<User>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(user.into())
}

// Create a new user
pub fn create(new_user: &User, pool: &PoolType) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::users;

    let conn = pool.get()?;

    insert_into(users).values(new_user).execute(&conn)?;

    Ok(new_user.clone().into())
}
