use axum::extract::Extension;
use axum::{
    body::Body, extract::Path, extract::Query, extract::RequestParts, http::Request,
    http::StatusCode, response::IntoResponse, Json,
};
use diesel::pg::PgConnection;
use http::header::HeaderValue;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::{Config, CONFIG};
use crate::database::Pool;
use crate::errors::ApiError;
use crate::models::user::{create_user, get_user, get_users, NewUser, User};
use crate::pagination::{PaginationRequest, PaginationResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRouteParams {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UsersResponse(pub Vec<UserResponse>);

impl From<Vec<User>> for UsersResponse {
    fn from(users: Vec<User>) -> Self {
        UsersResponse(users.into_par_iter().map(|user| user.into()).collect())
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: Uuid::parse_str(&user.id).unwrap(),
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        }
    }
}

pub async fn create_user_endpoint(
    Json(payload): Json<CreateUserRouteParams>,
    Extension(pool): Extension<Pool<PgConnection>>,
) -> Result<impl IntoResponse, ApiError> {
    // From impl hashes password
    let new_user: User = NewUser {
        id: Uuid::new_v4().to_string(),
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        password: payload.password,
    }
    .into();

    let insert_response = create_user(pool, &new_user).await?;
    Ok((StatusCode::CREATED, Json(insert_response)))
}

pub async fn get_user_endpoint(
    Path(user_id): Path<Uuid>,
    Extension(pool): Extension<Pool<PgConnection>>,
) -> Result<impl IntoResponse, ApiError> {
    let user = get_user(pool, user_id).await?;
    Ok((StatusCode::OK, Json::<UserResponse>(user)))
}

pub async fn get_users_endpoint(
    Query(params): Query<PaginationRequest>,
    // req: Request<Body>,
    Extension(pool): Extension<Pool<PgConnection>>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: fix get_base method
    // get_base is not currently working
    // let base = get_base(RequestParts::new(req));
    let base = "base".to_string();
    let paginated_users = get_users(pool, params, base).await?;
    Ok((
        StatusCode::OK,
        Json::<PaginationResponse<UsersResponse>>(paginated_users),
    ))
}

/// This method is currently failing to properly extract the RequestParts
/// scheme://hostpath
/// ex: http:127.0.0.1:8000/path
fn get_base(req: RequestParts<Body>) -> String {
    let scheme = req.uri().scheme_str().unwrap_or("http");
    let mut host = "";
    let default_host = HeaderValue::from_static(&CONFIG.server);
    if let Some(headers) = req.headers() {
        host = &headers
            .get("host")
            .unwrap_or(&default_host)
            .to_str()
            .unwrap_or("");
    };

    let path = req.uri().path();

    format!("{}://{}{}", scheme, host, path)
}

// TODO: convert update_user and delete_user to Axum

// /// Update a user
// pub async fn update_user(
//     user_id: Path<Uuid>,
//     pool: Data<PoolType>,
//     params: Json<UpdateUserRequest>,
// ) -> Result<Json<UserResponse>, ApiError> {
//     validate(&params)?;

//     // temporarily use the user's id for updated_at
//     // update when auth is added
//     let update_user = UpdateUser {
//         id: user_id.to_string(),
//         first_name: params.first_name.to_string(),
//         last_name: params.last_name.to_string(),
//         email: params.email.to_string(),
//         updated_by: user_id.to_string(),
//     };
//     let user = block(move || update(&pool, &update_user)).await?;
//     respond_json(user.into())
// }

// /// Delete a user
// pub async fn delete_user(
//     user_id: Path<Uuid>,
//     pool: Data<PoolType>,
// ) -> Result<HttpResponse, ApiError> {
//     block(move || delete(&pool, *user_id)).await?;
//     respond_ok()
// }

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use crate::models::user::tests::create_user as model_create_user;
//     use crate::tests::helpers::tests::{
//         get_data_pool, get_pagination_params, get_pool, get_query_pagination_params,
//         mock_get_request,
//     };

//     pub fn get_all_users() -> PaginationResponse<UsersResponse> {
//         let pool = get_pool();
//         let params = get_pagination_params();
//         let base = "http://fake/api/v1/user";
//         get_all(&pool, params, base.into()).unwrap()
//     }

//     pub fn get_first_users_id() -> Uuid {
//         get_all_users().data.0[0].id
//     }

//     #[actix_rt::test]
//     async fn it_gets_a_user() {
//         let first_user = &get_all_users().data.0[0];
//         let user_id: Path<Uuid> = get_first_users_id().into();
//         let response = get_user(user_id, get_data_pool()).await.unwrap();
//         assert_eq!(response.into_inner(), *first_user);
//     }

//     #[actix_rt::test]
//     async fn it_doesnt_find_a_user() {
//         let uuid = Uuid::new_v4();
//         let user_id: Path<Uuid> = uuid.into();
//         let response = get_user(user_id, get_data_pool()).await;
//         let expected_error = ApiError::NotFound(format!("User {} not found", uuid.to_string()));
//         assert!(response.is_err());
//         assert_eq!(response.unwrap_err(), expected_error);
//     }

//     #[actix_rt::test]
//     async fn it_gets_all_users() {
//         let request = mock_get_request("/api/v1/user");
//         let response = get_users(request, get_query_pagination_params(), get_data_pool()).await;
//         assert!(response.is_ok());
//         assert_eq!(
//             response.unwrap().into_inner().data.0[0],
//             get_all_users().data.0[0]
//         );
//     }

//     #[actix_rt::test]
//     async fn it_creates_a_user() {
//         let params = Json(CreateUserRequest {
//             first_name: "Satoshi".into(),
//             last_name: "Nakamoto".into(),
//             email: "satoshi@nakamotoinstitute.org".into(),
//             password: "123456".into(),
//         });
//         let response = create_user(get_data_pool(), Json(params.clone()))
//             .await
//             .unwrap();
//         assert_eq!(response.into_inner().first_name, params.first_name);
//     }

//     #[actix_rt::test]
//     async fn it_updates_a_user() {
//         let first_user = &get_all_users().data.0[0];
//         let user_id: Path<Uuid> = get_first_users_id().into();
//         let params = Json(UpdateUserRequest {
//             first_name: first_user.first_name.clone(),
//             last_name: first_user.last_name.clone(),
//             email: first_user.email.clone(),
//         });
//         let response = update_user(user_id, get_data_pool(), Json(params.clone()))
//             .await
//             .unwrap();
//         assert_eq!(response.into_inner().first_name, params.first_name);
//     }

//     #[actix_rt::test]
//     async fn it_deletes_a_user() {
//         let created = model_create_user();
//         let user_id = created.unwrap().id;
//         let user_id_path: Path<Uuid> = user_id.into();
//         let user = find(&get_pool(), user_id);
//         assert!(user.is_ok());
//         delete_user(user_id_path, get_data_pool()).await.unwrap();
//         let user = find(&get_pool(), user_id);
//         assert!(user.is_err());
//     }
// }
