use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::user::{UserResponse, UsersResponse};
use crate::models::auth::hash;
use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_by: String,
    pub updated_by: String,
}

// Get all users
pub fn get_all(pool: &PoolType) -> Result<UsersResponse, ApiError> {
    use crate::schema::users::dsl::users;

    let conn = pool.get()?;
    let all_users = users.load(&conn)?;

    Ok(all_users.into())
}

// Find a user by the user's id or error out
pub fn find(pool: &PoolType, user_id: Uuid) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{id, users};

    let not_found = format!("User {} not found", user_id);
    let conn = pool.get()?;
    let user = users
        .filter(id.eq(user_id.to_string()))
        .first::<User>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(user.into())
}

// Find a user by the user's id or error out
pub fn find_by_auth(
    pool: &PoolType,
    user_email: &str,
    user_password: &str,
) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{email, password, users};

    let conn = pool.get()?;
    let user = users
        .filter(email.eq(user_email.to_string()))
        .filter(password.eq(user_password.to_string()))
        .first::<User>(&conn)
        .map_err(|_| ApiError::Unauthorized)?;

    Ok(user.into())
}

// Create a new user
pub fn create(pool: &PoolType, new_user: &User) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::users;

    let conn = pool.get()?;

    insert_into(users).values(new_user).execute(&conn)?;

    Ok(new_user.clone().into())
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        User {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: hash(&user.password),
            created_by: user.created_by,
            created_at: Utc::now().naive_utc(),
            updated_by: user.updated_by,
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::get_pool;

    pub fn get_all_users() -> Result<UsersResponse, ApiError> {
        let pool = get_pool();
        get_all(&pool)
    }

    #[test]
    fn it_gets_a_user() {
        let users = get_all_users();
        assert!(users.is_ok());
    }

    #[test]
    fn test_find() {
        let users = get_all_users().unwrap();
        let user = &users.0[0];
        let found_user = find(&get_pool(), user.id).unwrap();
        assert_eq!(user, &found_user);
    }

    #[test]
    fn it_doesnt_find_a_user() {
        let user_id = Uuid::new_v4();
        let not_found_user = find(&get_pool(), user_id);
        assert!(not_found_user.is_err());
    }

    #[test]
    fn it_creates_a_user() {
        let user_id = Uuid::new_v4();
        let new_user = NewUser {
            id: user_id.to_string(),
            first_name: "Model".to_string(),
            last_name: "Test".to_string(),
            email: "model-test@nothing.org".to_string(),
            password: "123456".to_string(),
            created_by: user_id.to_string(),
            updated_by: user_id.to_string(),
        };
        let user: User = new_user.into();
        let created = create(&get_pool(), &user);
        assert!(created.is_ok());
        let found_user = find(&get_pool(), user_id).unwrap();
        assert_eq!(created.unwrap(), found_user);
    }
}
