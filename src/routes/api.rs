use actix_web::{
    web,
    // Responder,
    HttpResponse
};
use mongodb::Collection;
use serde::{Serialize, Deserialize};

use crate::models::vehicle::{Vehicle, VehicleType};

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleInput {
    v_type: String,
    brand: String,
    model: String,
    color: String,
    price: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleUpdateInput {
    id: String,
    brand: String,
    model: String,
    color: String,
    price: String
}

// pub async fn test() -> impl Responder {
//     "The app is working fine!"
// }

pub async fn get_vehicles(db: web::Data<Collection<Vehicle>>) -> HttpResponse {
    let res = Vehicle::get_all(db).await;

    HttpResponse::Ok().json(res.unwrap())
}

pub async fn get_vehicle_by_id(
    db: web::Data<Collection<Vehicle>>,
    id: web::Path<String>
) -> HttpResponse {
    let res = Vehicle::get_by_id(db, &id).await.unwrap();

    HttpResponse::Ok().json(res)
}

pub async fn create_vehicle(
    db: web::Data<Collection<Vehicle>>,
    v: web::Json<VehicleInput>
) -> HttpResponse {
    let input_type = v.v_type.to_owned();
    
    let vehicle_type: VehicleType = match input_type.as_str() {
        "car" => VehicleType::Car,
        "motorcycle" => VehicleType::Motorcycle,
        _ => VehicleType::Car
    };

    let new_vehicle = Vehicle::new(
        vehicle_type,
        String::from(v.brand.to_owned()),
        String::from(v.model.to_owned()),
        String::from(v.color.to_owned()),
        String::from(v.price.to_owned())
    );

    let _ = Vehicle::save(db, new_vehicle).await;

    HttpResponse::Ok().json(String::from("Vehicle created successfully!"))
}

pub async fn update_vehicle(
    db: web::Data<Collection<Vehicle>>,
    v: web::Json<VehicleUpdateInput>
) -> HttpResponse {
    let old_vehicle = Vehicle::new(
        VehicleType::Car,
        String::from(v.brand.to_owned()),
        String::from(v.model.to_owned()),
        String::from(v.color.to_owned()),
        String::from(v.price.to_owned()),
    );

    let _ = Vehicle::update_by_id(db, &v.id, old_vehicle).await;

    HttpResponse::Ok().json(String::from("Vehicle updated successfully"))

}

pub async fn delete_vehicle(
    db: web::Data<Collection<Vehicle>>,
    id: web::Path<String>
) -> HttpResponse {
    let _ = Vehicle::delete_by_id(db, &id).await;

    HttpResponse::Ok().json(String::from("Vehicle deleted successfully"))
}