use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// Import our local libraries
use lib_core;
use lib_auth;

#[get("/")]
async fn index() -> impl Responder {
    // Use logic from the libraries
    let app_name = lib_core::get_app_name();
    let auth_status = lib_auth::check_health();

    let response = format!("{} | Status: {}", app_name, auth_status);
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Rusteze Workspace Server running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}