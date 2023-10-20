use std::env;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use mongodb::Client;

use crate::{routes::api, models::vehicle::Vehicle};

pub async fn start_server() -> std::io::Result<()> {
  let uri: String = env::var("DB_URL").unwrap();
  let client = Client::with_uri_str(uri).await.expect("Failed to connect with DB");

  let col = client.database("vehicle-listing").collection::<Vehicle>("vehicles");

  let router = HttpServer::new(move || {
    let cors = Cors::permissive();

    App::new()
    .app_data(web::Data::new(col.clone()))
    .wrap(cors)
      // API handlers
      // .route("/api", web::get().to(api::home))
      .route("/api/vehicle", web::get().to(api::get_vehicles))
      .route("/api/vehicle/{id}", web::get().to(api::get_vehicle_by_id))
      .route("/api/vehicle", web::post().to(api::create_vehicle))
      .route("/api/vehicle/update", web::put().to(api::update_vehicle))
      .route("/api/vehicle/{id}", web::delete().to(api::delete_vehicle))
  })
  .bind(("localhost", 7001))?
  .run()
  .await;

  router
}