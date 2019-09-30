use crate::errors::ApiError;
use crate::helpers::respond_json;
use actix_web::web::Json;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Handler to get the liveness of the service
pub fn get_health() -> Result<Json<HealthResponse>, ApiError> {
    respond_json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    #[test]
    fn test_get_health() {
        let response = test::block_on(get_health()).unwrap();
        assert_eq!(response.into_inner().status, "ok".to_string());
    }
}
