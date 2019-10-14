use crate::auth::{decode_jwt, PrivateClaim};
use crate::errors::ApiError;
use actix_identity::RequestIdentity;
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use futures::future::{ok, Either, FutureResult};
use futures::Poll;

pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}
pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let identity = req.get_identity().unwrap_or("".into());
        let private_claim: Result<PrivateClaim, ApiError> = decode_jwt(&identity);
        let is_logged_in = private_claim.is_ok();

        if is_logged_in {
            Either::A(self.service.call(req))
        } else {
            if req.path() == "/api/v1/auth/login" {
                Either::A(self.service.call(req))
            } else {
                Either::B(ok(
                    req.into_response(HttpResponse::Unauthorized().finish().into_body())
                ))
            }
        }
    }
}
