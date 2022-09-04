mod controllers;
mod models;
mod repository;

use std::sync::Arc;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use controllers::house_controller::{create_house, get_house};
use crate::controllers::room_controller::create_room;
use crate::models::room_model::Room;
use crate::repository::house_repo::HouseRepo;
use crate::repository::mongo::Mongo;
use crate::repository::room_repo::RoomRepo;

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
    let room_repo = RoomRepo::init(mongo.get_db()).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(house_repo.clone()))
            .app_data(Data::new(room_repo.clone()))
            .service(hello)
            .service(create_house)
            .service(get_house)
            .service(create_room)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
