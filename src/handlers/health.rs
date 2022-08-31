use axum::{http::StatusCode, response::IntoResponse};

/// Handler to get the liveness of the service
pub async fn get_health_endpoint() -> impl IntoResponse {
    StatusCode::OK
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[actix_rt::test]
//     async fn test_get_health() {
//         let response = get_health().await.unwrap();
//         assert_eq!(response.into_inner().status, "ok".to_string());
//     }
// }
