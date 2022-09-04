mod controllers;
mod models;
mod repository;

use crate::controllers::house_controller::{
    add_device, add_room, get_house, remove_device, remove_room,
};
use crate::models::{device_model::DeviceType, house_model::House, room_model::Room};
use crate::repository::{house_repo::HouseRepo, mongo::Mongo};
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, RwLock};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from smart-house http server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mongo = Mongo::new().await;
    let house_repo = HouseRepo::init(mongo.get_db()).await;
    let house = Arc::new(RwLock::new(House::with_name("myHouse", &house_repo).await));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Arc::clone(&house)))
            .app_data(Data::new(house_repo.clone()))
            .service(hello)
            .service(get_house)
            .service(add_room)
            .service(remove_room)
            .service(add_device)
            .service(remove_device)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
