mod models;
mod routes;

use routes::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("App running on port 7001");
    
    router::start_server().await
}
