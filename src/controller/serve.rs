use std::fs;

use actix_files::NamedFile;
use actix_web::{
    dev, http::header, middleware::ErrorHandlerResponse, web, HttpResponse, Responder, Result,
};

/// Index Page
pub async fn index() -> Result<impl Responder> {
    Ok(NamedFile::open("./src/view/templates/index.html")?)
}

/// CSS Styles Handler
pub async fn styles() -> Result<impl Responder> {
    match fs::read_to_string("./src/view/static/output.css") {
        Ok(css_content) => Ok(HttpResponse::Ok()
            .append_header((header::CACHE_CONTROL, "max-age=7200"))
            .content_type("text/css; charset=utf-8")
            .body(css_content)),
        Err(e) => {
            log::error!("{}", e);
            Ok(HttpResponse::InternalServerError().body("Couldn't locate asset"))
        }
    }
}

/// Favicon Handler
pub async fn fav_svg() -> Result<impl Responder> {
    match fs::read_to_string("./src/assets/favicon.svg") {
        Ok(css_content) => Ok(HttpResponse::Ok()
            .append_header((header::CACHE_CONTROL, "max-age=7200"))
            .content_type("image/svg+xml; charset=utf-8")
            .body(css_content)),
        Err(e) => {
            log::error!("{}", e);
            Ok(HttpResponse::InternalServerError().body("Couldn't locate asset"))
        }
    }
}

/// Favicon Handler
pub async fn fav_png() -> Result<impl Responder> {
    match fs::read_to_string("./src/assets/favicon.png") {
        Ok(css_content) => Ok(HttpResponse::Ok()
            .append_header((header::CACHE_CONTROL, "max-age=7200"))
            .content_type("image/png; charset=utf-8")
            .body(css_content)),
        Err(e) => {
            log::error!("{}", e);
            Ok(HttpResponse::InternalServerError().body("Couldn't locate asset"))
        }
    }
}

/// Google Maps API Script
pub async fn custom_map() -> Result<impl Responder> {
    match fs::read_to_string("./src/scripts/map.js") {
        Ok(css_content) => Ok(HttpResponse::Ok()
            .append_header((header::CACHE_CONTROL, "max-age=7200"))
            .content_type("application/javascript; charset=utf-8")
            .body(css_content)),
        Err(e) => {
            log::error!("{}", e);
            Ok(HttpResponse::InternalServerError().body("Couldn't locate asset"))
        }
    }
}

/// Google Maps API Script
pub async fn google_api() -> Result<impl Responder> {
    match fs::read_to_string("./src/scripts/importMapsAPI.js") {
        Ok(css_content) => Ok(HttpResponse::Ok()
            .append_header((header::CACHE_CONTROL, "max-age=7200"))
            .content_type("application/javascript; charset=utf-8")
            .body(css_content)),
        Err(e) => {
            log::error!("{}", e);
            Ok(HttpResponse::InternalServerError().body("Couldn't locate asset"))
        }
    }
}

pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("./src/view/static/errors/400.html")?
        .customize()
        .with_status(res.status())
        .respond_to(res.request())
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("./src/view/static/errors/404.html")?
        .customize()
        .with_status(res.status())
        .respond_to(res.request())
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("./src/view/static/errors/500.html")?
        .customize()
        .with_status(res.status())
        .respond_to(res.request())
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}
