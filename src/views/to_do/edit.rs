use actix_web::{HttpResponse, web};
use serde_json::Map;
use serde_json::value::Value;

use super::utils::return_state;
use crate::state::read_file;

use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;
use crate::to_do::to_do_factory;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state = read_file(String::from("./state.json"));
    let title_reference = &to_do_item.title.clone();
    let title = to_do_item.title.clone();

    let status: String;
    dbg!("{state}");
    match &state.get(title_reference) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => return HttpResponse::NotFound().json(format!("{title_reference} not in state")),
    }
    if status == to_do_item.status {
        return HttpResponse::Ok().json(return_state());
    }

    match to_do_factory(&status, title) {
        Err(_item) => return HttpResponse::BadRequest().json(format!("{status} not accepted")),
        Ok(item) => process_input(item, String::from("edit"), &state),
    }

    HttpResponse::Ok().json(return_state())
}
