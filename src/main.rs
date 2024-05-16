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

    let port: u16 = 6969;
    log::info!(" Server started successfully at http://localhost:{}", port);

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/view/templates/**/*")).unwrap();
        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                controller::serve::internal_server_error,
            )
            .handler(
                http::StatusCode::BAD_REQUEST,
                controller::serve::bad_request,
            )
            .handler(http::StatusCode::NOT_FOUND, controller::serve::not_found);
        App::new()
            .app_data(web::Data::new(tera))
            .wrap(error_handlers)
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(controller::serve::index)))
            // .service(web::resource("/login").route(web::get().to(api::route::login)))
            // .service(web::resource("/register").route(web::get().to(api::route::register)))
            .service(web::resource("/output").route(web::get().to(controller::serve::styles)))
            .service(web::resource("/favicon.svg").route(web::get().to(controller::serve::fav_svg)))
            .service(web::resource("/favicon.png").route(web::get().to(controller::serve::fav_png)))
            .service(web::resource("/mapsLibrary").route(web::get().to(controller::serve::google_api)))
            .service(web::resource("/mapsConfig").route(web::get().to(controller::serve::custom_map)))
        // .service(web::resource("/clicked").route(web::get().to(api::route::clicked)))
        // .service(web::resource("/test").route(web::get().to(api::route::test)))
        // register favicon
        // .default_service(web::to(default_handler))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
