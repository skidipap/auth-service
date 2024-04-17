mod users;

use actix_web::{App, HttpServer};

use users::handlers::{get_users, echo};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_users)
            .service(echo)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}