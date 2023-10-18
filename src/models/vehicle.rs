use actix_web::web;
use serde::{Serialize, Deserialize};
use mongodb::{
  Collection,
  error::Result as MongoResult,
  results::{InsertOneResult, UpdateResult, DeleteResult},
  bson::{oid::ObjectId, extjson::de::Error, doc}
};
use futures::stream::TryStreamExt;

#[derive(Debug)]
pub enum VehicleType {
  Car,
  Motorcycle
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
  #[serde(rename="_id", skip_serializing_if="Option::is_none")]
  pub id: Option<ObjectId>,
  pub brand: String,
  pub model: String,
  pub color: String,
  wheels: u8,
  doors: u8,
  is_available: bool
}

impl Vehicle {
  pub fn new(
    v_type: VehicleType,
    b: String,
    m: String,
    c: String
  ) -> Vehicle {
    match v_type {
      VehicleType::Car => Vehicle {
        id: None,
        brand: b,
        model: m,
        color: c,
        wheels: 4,
        doors: 4,
        is_available: true
      },
      VehicleType::Motorcycle => Vehicle {
        id: None,
        brand: b,
        model: m,
        color: c,
        wheels: 2,
        doors: 0,
        is_available: true
      }
    }
  }

  pub async fn save(
    db: web::Data<Collection<Vehicle>>,
    v: Vehicle
  ) -> MongoResult<InsertOneResult> {

    Ok(db.insert_one(v, None).await.expect("saved wrong"))
  }

  pub async fn get_all(db: web::Data<Collection<Vehicle>>) -> Result<Vec<Vehicle>, Error> {
    // let uri: String = env::var("DB_URL").unwrap();
    // let client = Client::with_uri_str(&uri).await.expect("could not connect to DB");
    // let db = client.database("vehicle-listing");
    // let col = db.collection::<Vehicle>("vehicles");

    let mut cursors = db
      .find(None, None)
      .await
      .ok()
      .expect("Error getting list of vehicles");

    let mut vehicles: Vec<Vehicle> = Vec::new();
    while let Some(v) = cursors
      .try_next()
      .await
      .ok()
      .expect("Error mapping through cursor")
    {
      vehicles.push(v)
    }
    Ok(vehicles)
  }

  pub async fn get_by_id(
    db: web::Data<Collection<Vehicle>>,
    id: &String
  ) -> Result<Vehicle, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    
    let single_vehicle = db
      .find_one(doc! { "_id": obj_id }, None)
      .await
      .ok()
      .expect("Error fetching vehicle by id");

    Ok(single_vehicle.unwrap())
  }

  pub async fn update_by_id(
    db: web::Data<Collection<Vehicle>>,
    id: &String,
    v: Vehicle
  ) -> Result<UpdateResult, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();

    let updated_obj = db
      .update_one(
        doc! { "_id": obj_id },
        doc! {
          "$set": {
            "brand": v.brand,
            "model": v.model,
            "color": v.color
          }
        },
        None
      ).await
      .ok()
      .expect("Failed to update vehicle");

    Ok(updated_obj)
  }

  pub async fn delete_by_id(
    db: web::Data<Collection<Vehicle>>,
    id: &String
  ) -> Result<DeleteResult, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();

    let deleted_obj = db
      .delete_one(doc! { "_id": obj_id }, None)
      .await
      .ok()
      .expect("Failed to delete vehicle");

    Ok(deleted_obj)
  }

}

// pub trait Vehicle {
//   fn toggle_status(&mut self);
// }