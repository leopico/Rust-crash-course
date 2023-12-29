use actix_web::{
    post,
    web::{self, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct JsonReq {
    name: String,
    age: i32,
}

#[derive(Serialize, Deserialize)]
struct JsonStruct {
    msg: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .default_service(web::to(default_route))
            .route("/{num}", web::get().to(route_fn)) //same level
            .service(route_sevice) //same level
    })
    .bind(("127.0.0.1", 8090))
    .unwrap()
    .run()
    .await
}

async fn default_route() -> impl Responder {
    "Hello Rusty!"
}

// #[get("/{name}")]
// async fn route_sevice(name: Path<String>) -> impl Responder {
//     name.to_owned()
// }

// #[get("/json/{name}")]
// async fn route_sevice(name: Path<String>) -> HttpResponse {
//     let msg = format!("Hello, {name}");
//     HttpResponse::Ok().json(JsonStruct { msg })
// }

#[post("/json/req")]
async fn route_sevice(user: Json<JsonReq>) -> HttpResponse {
    let msg = format!("Hello, {}! You are {} years old", user.name, user.age);
    HttpResponse::Ok().json(JsonStruct { msg })
}

async fn route_fn(num: Path<i32>) -> impl Responder {
    format!("{}", num.to_owned() + 1)
}
