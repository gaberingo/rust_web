use actix_web::{HttpRequest, Responder};

use super::utils::return_state;
use crate::auth::jwt::JwtToken;

/// This view gets all of the saved to do items that are stored in the state.json file.
///
/// # Arguments
/// None
///
/// # Returns
/// * (web::Json): all of the stored to do items
pub async fn get(req: HttpRequest) -> impl Responder {
    let token = JwtToken::decode_from_request(req).unwrap();
    return_state(&token.user_id)
}
