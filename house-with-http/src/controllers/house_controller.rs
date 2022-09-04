use crate::models::device_model::AddDevice;
use crate::{models::house_model::House, repository::house_repo::HouseRepo};
use actix_web::web::Json;
use actix_web::{
    get, post,
    web::{Data, Path},
    HttpResponse,
};
use std::sync::{Arc, RwLock};

#[get("/house")]
pub async fn get_house(house_data: Data<Arc<RwLock<House>>>) -> HttpResponse {
    let house = house_data.read().unwrap().to_owned();
    HttpResponse::Ok().json(house)
}

#[get("/house/add-room/{room_name}")]
pub async fn add_room(
    house_data: Data<Arc<RwLock<House>>>,
    repo: Data<HouseRepo>,
    name: Path<String>,
) -> HttpResponse {
    let room_name = name.into_inner();

    let mut house = house_data.write().unwrap();
    house.add_room(&room_name);
    repo.persist_house(&house).await;
    HttpResponse::Ok().json(house.to_owned())
}

#[get("/house/remove-room/{room_name}")]
pub async fn remove_room(
    house_data: Data<Arc<RwLock<House>>>,
    repo: Data<HouseRepo>,
    name: Path<String>,
) -> HttpResponse {
    let room_name = name.into_inner();

    let mut house = house_data.write().unwrap();
    house.remove_room(&room_name);
    repo.persist_house(&house).await;
    HttpResponse::Ok().json(house.to_owned())
}

#[post("/house/add-device")]
pub async fn add_device(
    house_data: Data<Arc<RwLock<House>>>,
    repo: Data<HouseRepo>,
    add_device: Json<AddDevice>,
) -> HttpResponse {
    let mut house = house_data.write().unwrap();
    house.add_device_in_room(
        &add_device.room_name,
        &add_device.device_name,
        add_device.device_type,
    );
    repo.persist_house(&house).await;
    HttpResponse::Ok().json(house.to_owned())
}

#[get("/house/remove-device/{room_name}/{device_name}")]
pub async fn remove_device(
    house_data: Data<Arc<RwLock<House>>>,
    repo: Data<HouseRepo>,
    names: Path<(String, String)>,
) -> HttpResponse {
    let (room_name, device_name) = names.into_inner();
    let mut house = house_data.write().unwrap();
    house.remove_device_from_room(&room_name, &device_name);
    repo.persist_house(&house).await;
    HttpResponse::Ok().json(house.to_owned())
}
