use crate::errors::ApiError;
use actix_web::web::Json;
use serde::Serialize;

/// Helper function to reduce boilerplate of an OK/Json response
pub fn respond_json<T>(data: T) -> Result<Json<T>, ApiError>
where
    T: Serialize,
{
    Ok(Json(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    pub struct TestResponse {
        pub first_name: String,
    }

    #[test]
    fn it_responds_json() {
        let response = TestResponse {
            first_name: "Satoshi".into(),
        };
        let result = respond_json(response.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner(), response);
    }
}
