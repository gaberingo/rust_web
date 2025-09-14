use crate::diesel;
use diesel::prelude::*;

use actix_web::{HttpRequest, HttpResponse, web};

use super::utils::return_state;

use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::item::item::Item;
use crate::schema::to_do;

use crate::auth::jwt::JwtToken;

pub async fn delete(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let title_ref = to_do_item.title.clone();

    let token = JwtToken::decode_from_request(req).unwrap();

    let connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(title_ref.as_str()))
        .filter(to_do::columns::user_id.eq(token.user_id.clone()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    let _ = diesel::delete(&items[0]).execute(&connection);
    HttpResponse::Ok().json(return_state(&token.user_id))
}
