use std::error::Error;
use std::net::IpAddr;
use std::ops::Deref;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, middleware::Logger, Responder, ResponseError, web};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::index::{DistanceSearch, IndexError, MeetingIndex, SearchOptions};

#[derive(Serialize)]
struct ApiError {
    message: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    longitude: Option<f64>,
    latitude: Option<f64>,
    distance: Option<f64>,
}

impl Into<SearchOptions> for SearchQuery {
    fn into(self) -> SearchOptions {
        SearchOptions {
            distance: match (self.longitude, self.latitude, self.distance) {
                (Some(longitude), Some(latitude), Some(distance)) => Some(DistanceSearch { latitude, longitude, distance }),
                _ => None
            }
        }
    }
}

impl ResponseError for IndexError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(ApiError {
                message: self.to_string(),
            })
    }
}

#[get("/")]
async fn index(meeting_index: web::Data<MeetingIndex>, query: web::Query<SearchQuery>) -> Result<impl Responder, IndexError> {
    let query = query.into_inner();

    let meetings = meeting_index.search(&query.into()).await?;
    Ok(web::Json(meetings))
}

pub async fn start_server(meeting_index: MeetingIndex, address: IpAddr, port: u16) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"])
            .block_on_origin_mismatch(false)
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(meeting_index.clone()))
            .service(index)
    })
        .bind((address, port))?
        .run()
        .await
}
