use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_sessions::async_session::Session;

/// Authenticate middleware is currently unneeded
/// Additional logic could be added here add data to use session for Authorization
#[allow(clippy::missing_errors_doc)]
pub async fn authenticate<B: Send>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    tracing::info!("Middleware: checking if user exists");
    let session = req
        .extensions()
        .get::<Session>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(next.run(req).await)
}
