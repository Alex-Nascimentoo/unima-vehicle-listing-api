use std::env;
use actix_web::{web, App, HttpServer};
use mongodb::Client;

use crate::{routes::api, models::vehicle::Vehicle};

pub async fn start_server() -> std::io::Result<()> {
  let uri: String = env::var("DB_URL").unwrap();
  let client = Client::with_uri_str(uri).await.expect("Failed to connect with DB");

  let col = client.database("vehicle-listing").collection::<Vehicle>("vehicles");

  let router = HttpServer::new(move || {
  App::new()
    .app_data(web::Data::new(col.clone()))
    // API handlers
    // .route("/api", web::get().to(api::home))
    .route("/api/vehicle", web::get().to(api::get_vehicles))
    .route("/api/vehicle/{id}", web::get().to(api::get_vehicle_by_id))
    .route("/api/vehicle", web::post().to(api::create_vehicle))
    .route("/api/vehicle/update", web::put().to(api::update_vehicle))
    .route("/api/vehicle/{id}", web::delete().to(api::delete_vehicle))
  })
  .bind(("127.0.0.1", 7001))?
  .run()
  .await;

  router
}