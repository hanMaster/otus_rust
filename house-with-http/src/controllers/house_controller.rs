use crate::{models::house_model::House, repository::house_repo::HouseRepo};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use std::cell::RefCell;
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
    _repo: Data<HouseRepo>,
    name: Path<String>,
) -> HttpResponse {
    let room_name = name.into_inner();

    let mut house = house_data.write().unwrap();
    house.remove_room(&room_name);
    HttpResponse::Ok().json(house.to_owned())
}
