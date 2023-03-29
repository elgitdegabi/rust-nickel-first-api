use nickel::{HttpRouter, JsonBody, MediaType, Nickel};
use serde_json::json;

use crate::config::constants::*;
use crate::model::health::Health;
use crate::model::user::User;
use crate::service::user_service::*;

pub fn config_endpoints(mut server: Nickel) -> Nickel {
    server.get("/health",
               middleware! { |_, mut response| {
                   response.set(MediaType::Json);
                   format!("{}", json!(Health {status: String::from(SERVER_RUNNING_STATUS)}))
               }});

    server.get("/users",
               middleware! { |_, mut response| {
                   response.set(MediaType::Json);
                   format!("{}", json!(get_users()))
               }});

    server.get("/user/:userid",
               middleware! { |request, mut response| {
                   let user_id: i32 = get_user_id(request.param("userid"));
                   response.set(MediaType::Json);
                   format!("{}", json!(get_user(user_id)))
               }});

    server.post("/user/add",
                middleware! { |request, mut response| {
                    let user = request.json_as::<User>().unwrap();
                   response.set(MediaType::Json);
                   format!("{}", json!(add_user(user)))
               }});

    server.post("/user/update/:userid",
                middleware! { |request, mut response| {
                   let user_id: i32 = get_user_id(request.param("userid"));
                   let user = request.json_as::<User>().unwrap();
                   response.set(MediaType::Json);
                   format!("{}", json!(update_user(user_id, user)))
               }});

    server.post("/user/delete/:userid",
                middleware! { |request, mut response| {
                   let user_id: i32 = get_user_id(request.param("userid"));
                   response.set(MediaType::Json);
                   format!("{}", json!(Health {status: delete_user(user_id)}))
               }});

    return server;

    fn get_user_id(user_id: Option<&str>) -> i32 {
        return user_id.unwrap().parse::<i32>().unwrap();
    }
}
