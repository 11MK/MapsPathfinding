use minidom::Element;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

// async fn fetch_street_data(l: f64, r: f64, t: f64, b: f64) {
//     let request_url = format!("https://www.openstreetmap.org/api/0.6/map?bbox={l},{b},{r},{t}");
//
//     let resp = match reqwest::get(request_url).await {
//         Ok(resp) => resp.text().await.unwrap(),
//         Err(err) => panic!("Error: {}", err)
//     };
//     // output resp to json file in ./data.json
//     std::fs::write("./data.json", resp).expect("Unable to write file");
// }

// #[tokio::main]
// async fn main() {
//     let left = -111.0542;
//     let bot = 45.6687;
//     let right = -111.0324;
//     let top = 45.6794;
//     fetch_street_data(left, right, top, bot).await;
// }

use actix_web::{
    http::{self, Method, StatusCode},
    middleware::{self, ErrorHandlers},
    web, App, Either, HttpResponse, HttpServer, Responder, Result,
};
use tera::Tera;

mod controller;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = 8091;
    log::info!(" Server started successfully at http://localhost:{}", port);

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        let error_handlers = ErrorHandlers::new()
            .handler( http::StatusCode::INTERNAL_SERVER_ERROR, api::error::internal_server_error,)
            .handler(http::StatusCode::BAD_REQUEST, api::error::bad_request)
            .handler(http::StatusCode::NOT_FOUND, api::error::not_found);
        App::new()
            .app_data(web::Data::new(tera))
            .wrap(error_handlers)
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(api::route::index)))
            .service(web::resource("/login").route(web::get().to(api::route::login)))
            .service(web::resource("/register").route(web::get().to(api::route::register)))
            .service(web::resource("/favicon").route(web::get().to(api::route::favicon)))
            .service(web::resource("/clicked").route(web::get().to(api::route::clicked)))
            .service(web::resource("/test").route(web::get().to(api::route::test)))
        // register favicon
        // .default_service(web::to(default_handler))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
