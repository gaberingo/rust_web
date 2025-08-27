use actix_web::{HttpResponse, web};
use serde_json::Map;
use serde_json::value::Value;

use super::utils::return_state;
use crate::state::read_file;

use crate::json_serialization::to_do_items::ToDoItem;
use crate::processes::process_input;
use crate::to_do::to_do_factory;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file(String::from("./state.json"));
    let title_reference = &to_do_item.title.clone();
    let title = to_do_item.title.clone();

    let status: String;

    match &state.get(title_reference) {
        Some(res) => {
            status = res.to_string().replace('\"', "");
        }
        None => return HttpResponse::NotFound().json(format!("{} not in state", title_reference)),
    }

    if &status == &to_do_item.status {
        HttpResponse::Ok().json(return_state())
    } else {
        HttpResponse::NotFound().json(format!("{} not in state", title_reference))
    }
}
