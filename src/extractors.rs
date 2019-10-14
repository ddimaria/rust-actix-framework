use crate::auth::{decode_jwt, PrivateClaim};
use crate::models::user::AuthUser;
use actix_identity::Identity;
use actix_web::{
    dev::Payload,
    web::{HttpRequest, HttpResponse},
    FromRequest,
};

/// Extractor for pulling the identity out of a request.
///
/// Simply add "user: AuthUser" to a handler to invoke this.
impl FromRequest for AuthUser {
    type Error = HttpResponse;
    type Config = ();
    type Future = Result<Self, HttpResponse>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if let Some(identity) = Identity::from_request(req, payload)?.identity() {
            let private_claim: PrivateClaim = decode_jwt(&identity).unwrap();
            return Ok(AuthUser {
                id: private_claim.user_id.to_string(),
                email: private_claim.email,
            });
        }
        Err(HttpResponse::Unauthorized().into())
    }
}
