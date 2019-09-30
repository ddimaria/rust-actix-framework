#[cfg(test)]
mod tests {
    use crate::handlers::user::{tests::get_first_users_id, CreateUserRequest};
    use crate::tests::helpers::tests::{assert_get, assert_post};
    use actix_web::web::Path;
    use uuid::Uuid;

    const PATH: &str = "/api/v1/user";

    #[test]
    fn test_get_user() {
        let user_id: Path<(Uuid)> = get_first_users_id().into();
        let url = format!("{}/{}", PATH, user_id);
        assert_get(&url);
    }

    #[test]
    fn test_get_users() {
        assert_get(PATH);
    }

    #[test]
    fn test_create_user() {
        let params = CreateUserRequest {
            first_name: "Satoshi".into(),
            last_name: "Nakamoto".into(),
            email: "satoshi@nakamotoinstitute.org".into(),
        };
        assert_post(PATH, params);
    }
}
