use crate::{
    models::room_model::{AddRoom, Room},
    repository::{house_repo::HouseRepo, room_repo::RoomRepo},
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/rooms")]
pub async fn create_room(
    house_repo: Data<HouseRepo>,
    room_repo: Data<RoomRepo>,
    add_room: Json<AddRoom>,
) -> HttpResponse {
    let add_room = AddRoom {
        house_name: add_room.house_name.to_owned(),
        room_name: add_room.room_name.to_owned(),
    };
    let house_result = house_repo.get_house(&add_room.house_name).await;
    match house_result {
        Ok(house) => {
            let new_room = Room {
                id: None,
                house_id: house.id.unwrap(),
                name: add_room.room_name,
            };
            let room_detail = room_repo.create_room(new_room).await;
            match room_detail {
                Ok(room) => HttpResponse::Ok().json(room),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// #[get("/house/{name}")]
// pub async fn get_house(db: Data<HouseRepo>, path: Path<String>) -> HttpResponse {
//     let name = path.into_inner();
//     if name.is_empty() {
//         return HttpResponse::BadRequest().body("invalid house name");
//     }
//     let house_detail = db.get_house(&name).await;
//     match house_detail {
//         Ok(house) => HttpResponse::Ok().json(house),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }
