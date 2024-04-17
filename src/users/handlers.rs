use actix_web::{get, post, HttpResponse, Responder};
use crate::users::services::users_list;


#[get("/")]
async fn get_users() -> impl Responder {
    let users = users_list(); // Call users_list function to get the list of users
    HttpResponse::Ok().json(users)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
