use crate::auth::hash;
use crate::database::Pool;
use crate::errors::ApiError;
use crate::handlers::user::{UserResponse, UsersResponse};
use crate::pagination::{PaginationRequest, PaginationResponse};
use crate::schema::users;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        User {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: hash(&user.password),
        }
    }
}

/// Create a new user
pub async fn create_user(
    pool: Pool<PgConnection>,
    new_user: &User,
) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::users;

    let conn = pool.get()?;
    diesel::insert_into(users).values(new_user).execute(&conn)?;
    Ok(new_user.clone().into())
}

pub async fn get_user(pool: Pool<PgConnection>, user_id: Uuid) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{id, users};

    let conn = pool.get()?;
    let user = users
        .filter(id.eq(user_id.to_string()))
        .first::<User>(&conn)
        .map_err(|_| ApiError::NotFound(format!("User {} not found", user_id)))?;
    Ok(user.into())
}

pub async fn get_users(
    pool: Pool<PgConnection>,
    params: PaginationRequest,
    base: String,
) -> Result<PaginationResponse<UsersResponse>, ApiError> {
    use crate::schema::users::dsl::users;

    crate::paginate!(pool, users, User, params, UsersResponse, base)
}

pub async fn get_user_by_auth(
    pool: Pool<PgConnection>,
    user_email: &str,
    user_password: &str,
) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{email, password, users};

    let conn = pool.get()?;
    let user = users
        .filter(email.eq(user_email))
        .filter(password.eq(user_password))
        .first::<User>(&conn)
        .map_err(|_| ApiError::Unauthorized(format!("Invalid Credentials")))?;
    Ok(user.into())
}

// TODO: convert update and delete methods to Axum
// /// Update a user
// pub fn update(pool: &PoolType, update_user: &UpdateUser) -> Result<UserResponse, ApiError> {
//     use crate::schema::users::dsl::{id, users};

//     let conn = pool.get()?;
//     diesel::update(users)
//         .filter(id.eq(update_user.id.clone()))
//         .set(update_user)
//         .execute(&conn)?;
//     find(&pool, Uuid::parse_str(&update_user.id)?)
// }

// /// Delete a user
// pub fn delete(pool: &PoolType, user_id: Uuid) -> Result<(), ApiError> {
//     use crate::schema::users::dsl::{id, users};

//     let conn = pool.get()?;
//     diesel::delete(users)
//         .filter(id.eq(user_id.to_string()))
//         .execute(&conn)?;
//     Ok(())
// }

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use crate::tests::helpers::tests::{get_pagination_params, get_pool};

//     pub fn get_all_users() -> Result<PaginationResponse<UsersResponse>, ApiError> {
//         let pool = get_pool();
//         let params = get_pagination_params();
//         let base = "http://fake/api/v1/user";
//         get_all(&pool, params, base.into())
//     }

//     pub fn create_user() -> Result<UserResponse, ApiError> {
//         let user_id = Uuid::new_v4();
//         let new_user = NewUser {
//             id: user_id.to_string(),
//             first_name: "Model".to_string(),
//             last_name: "Test".to_string(),
//             email: "model-test@nothing.org".to_string(),
//             password: "123456".to_string(),
//             created_by: user_id.to_string(),
//             updated_by: user_id.to_string(),
//         };
//         let user: User = new_user.into();
//         create(&get_pool(), &user)
//     }

//     #[test]
//     fn it_gets_a_user() {
//         let users = get_all_users();
//         assert!(users.is_ok());
//     }

//     #[test]
//     fn test_find() {
//         let users = get_all_users().unwrap();
//         let user = &users.data.0[0];
//         let found_user = find(&get_pool(), user.id).unwrap();
//         assert_eq!(user, &found_user);
//     }

//     #[test]
//     fn it_doesnt_find_a_user() {
//         let user_id = Uuid::new_v4();
//         let not_found_user = find(&get_pool(), user_id);
//         assert!(not_found_user.is_err());
//     }

//     #[test]
//     fn it_creates_a_user() {
//         let created = create_user();
//         assert!(created.is_ok());
//         let unwrapped = created.unwrap();
//         let found_user = find(&get_pool(), unwrapped.id.clone()).unwrap();
//         assert_eq!(unwrapped, found_user);
//     }

//     #[test]
//     fn it_updates_a_user() {
//         let users = get_all_users().unwrap();
//         let user = &users.data.0[1];
//         let update_user = UpdateUser {
//             id: user.id.to_string(),
//             first_name: "ModelUpdate".to_string(),
//             last_name: "TestUpdate".to_string(),
//             email: "model-update-test@nothing.org".to_string(),
//             updated_by: user.id.to_string(),
//         };
//         let updated = update(&get_pool(), &update_user);
//         assert!(updated.is_ok());
//         let found_user = find(&get_pool(), user.id).unwrap();
//         assert_eq!(updated.unwrap(), found_user);
//     }

//     #[test]
//     fn it_fails_to_update_a_nonexistent_user() {
//         let user_id = Uuid::new_v4();
//         let update_user = UpdateUser {
//             id: user_id.to_string(),
//             first_name: "ModelUpdateFailure".to_string(),
//             last_name: "TestUpdateFailure".to_string(),
//             email: "model-update-failure-test@nothing.org".to_string(),
//             updated_by: user_id.to_string(),
//         };
//         let updated = update(&get_pool(), &update_user);
//         assert!(updated.is_err());
//     }

//     #[test]
//     fn it_deletes_a_user() {
//         let created = create_user();
//         let user_id = created.unwrap().id;
//         let user = find(&get_pool(), user_id);
//         assert!(user.is_ok());
//         delete(&get_pool(), user_id).unwrap();
//         let user = find(&get_pool(), user_id);
//         assert!(user.is_err());
//     }
// }
