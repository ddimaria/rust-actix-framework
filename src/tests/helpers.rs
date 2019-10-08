#[cfg(test)]
pub mod tests {
    use crate::config::CONFIG;
    use crate::database::{add_pool, init_pool, Pool};
    use crate::routes::routes;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::{test, web::Data, App};
    use diesel::{mysql::MysqlConnection};
    use serde::Serialize;

    /// Helper for HTTP GET integration tests
    pub fn test_get(route: &str) -> ServiceResponse {
        let mut app = test::init_service(
            App::new()
                .configure(add_pool)
                .configure(routes),
        );
        let request = test::TestRequest::get().uri(route).to_request();
        test::block_on(app.call(request)).unwrap()
    }

    /// Helper for HTTP POST integration tests
    pub fn test_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
       let mut app = test::init_service(
            App::new()
                .configure(add_pool)
                .configure(routes),
        );
        let request = test::TestRequest::post()
            .set_json(&params)
            .uri(route)
            .to_request();
        test::block_on(app.call(request)).unwrap()
    }

    /// Assert that a route is successful for HTTP GET requests
    pub fn assert_get(route: &str) -> ServiceResponse {
        let response = test_get(route);
        assert!(response.status().is_success());
        response
    }

    /// Assert that a route is successful for HTTP POST requests
    pub fn assert_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
        let response = test_post(route, params);
        assert!(response.status().is_success());
        response
    }

    pub fn get_pool() -> Pool<MysqlConnection> {
        init_pool::<MysqlConnection>(CONFIG.clone()).unwrap()
    }

    pub fn get_data_pool() -> Data<Pool<MysqlConnection>> {
        Data::new(get_pool())
    }
}
