use crate::house::{Device, DeviceType, House, NewHouse, NewRoom, Room};
use actix_web::web::Path;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use mongodb::Database;
use serde::Serialize;

pub async fn info(mongo: Data<Database>, house_name: Path<String>) -> HttpResponse {
    let house = House::get_by_name(house_name.into_inner().as_str(), &mongo)
        .await
        .ok()
        .expect("House not found");
    HttpResponse::Ok().json(house)
}

#[derive(Serialize)]
struct Status {
    status: String,
}

pub async fn new_house(mongo: Data<Database>, new_house: Json<NewHouse>) -> HttpResponse {
    match new_house.into_inner().save(&mongo).await {
        Ok(()) => HttpResponse::Ok().json(Status {
            status: "Ok".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn add_room(mongo: Data<Database>, new_room: Json<NewRoom>) -> HttpResponse {
    HttpResponse::Ok().json(())
}
