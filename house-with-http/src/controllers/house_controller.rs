use crate::{models::house_model::House, repository::house_repo::HouseRepo};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/house")]
pub async fn create_house(db: Data<HouseRepo>, new_house: Json<House>) -> HttpResponse {
    let data = House {
        id: None,
        name: new_house.name.to_owned(),
    };
    let house_detail = db.create_house(data).await;
    match house_detail {
        Ok(house) => HttpResponse::Ok().json(house),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/house/{name}")]
pub async fn get_house(db: Data<HouseRepo>, path: Path<String>) -> HttpResponse {
    let name = path.into_inner();
    if name.is_empty() {
        return HttpResponse::BadRequest().body("invalid house name");
    }
    let house_detail = db.get_house(&name).await;
    match house_detail {
        Ok(house) => HttpResponse::Ok().json(house),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
