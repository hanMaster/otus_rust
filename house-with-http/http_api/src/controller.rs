use crate::db::house_repo::HouseRepo;
use crate::house::House;
use actix_web::web::Json;
use actix_web::{
    get, post,
    web::{Data, Path},
    HttpResponse,
};
use models::device::AddDevice;
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
    {
        let mut house = house_data.write().unwrap();
        if let Err(err) = house.add_room(&room_name) {
            return HttpResponse::UnprocessableEntity().body(err);
        };
    }
    let house = house_data.read().unwrap().to_owned();
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

    {
        let mut house = house_data.write().unwrap();
        house.remove_room(&room_name);
    }
    let house = house_data.read().unwrap().to_owned();
    repo.persist_house(&house).await;
    HttpResponse::Ok().json(house.to_owned())
}

#[post("/house/add-device")]
pub async fn add_device(
    house_data: Data<Arc<RwLock<House>>>,
    repo: Data<HouseRepo>,
    add_device: Json<AddDevice>,
) -> HttpResponse {
    {
        let mut house = house_data.write().unwrap();

        if let Err(err) = house.add_device_in_room(
            &add_device.room_name,
            &add_device.device_name,
            add_device.device_type,
        ) {
            return HttpResponse::UnprocessableEntity().body(err);
        };
    }
    let house = house_data.read().unwrap().to_owned();
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
    {
        let mut house = house_data.write().unwrap();
        house.remove_device_from_room(&room_name, &device_name);
    }
    let house = house_data.read().unwrap().to_owned();
    repo.persist_house(&house).await;
    HttpResponse::Ok().json(house.to_owned())
}

#[get("/house/devices/{room_name}")]
pub async fn device_list_by_room(
    house_data: Data<Arc<RwLock<House>>>,
    name: Path<String>,
) -> HttpResponse {
    let room_name = name.into_inner();
    let house = house_data.read().unwrap().to_owned();
    let list = house.device_list_by_room(room_name);
    HttpResponse::Ok().json(list)
}

#[get("/house/rooms")]
pub async fn rooms_list(house_data: Data<Arc<RwLock<House>>>) -> HttpResponse {
    let house = house_data.read().unwrap().to_owned();
    let list = house.rooms_list();
    HttpResponse::Ok().json(list)
}
