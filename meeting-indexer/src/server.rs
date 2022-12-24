use std::net::IpAddr;

use actix_cors::Cors;
use actix_web::{
    App, get, HttpResponse, HttpServer, middleware::Logger, Responder, ResponseError,
    web,
};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::index::*;
use crate::meeting;

#[derive(Serialize)]
struct ApiError {
    message: String,
}

#[derive(Deserialize, IntoParams)]
struct SearchQuery {
    /// The longitude of the user.
    longitude: Option<f64>,

    /// The latitude of the user.
    latitude: Option<f64>,

    /// The maximum distance in kilometers.
    distance: Option<f64>,
}

impl Into<SearchOptions> for SearchQuery {
    fn into(self) -> SearchOptions {
        SearchOptions {
            distance: match (self.longitude, self.latitude, self.distance) {
                (Some(longitude), Some(latitude), Some(distance)) => Some(DistanceSearch {
                    latitude,
                    longitude,
                    distance,
                }),
                _ => None,
            },
        }
    }
}

impl ResponseError for IndexError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ApiError {
            message: self.to_string(),
        })
    }
}

#[utoipa::path(
    params(SearchQuery),
    responses(
        (status = 200, description = "Retrieve a list of meetings", body = [SearchMeeting]))
    )
]
#[get("/meetings")]
async fn index(
    meeting_index: web::Data<MeetingIndex>,
    query: web::Query<SearchQuery>,
) -> Result<impl Responder, IndexError> {
    let query = query.into_inner();

    let meetings = meeting_index.search(&query.into()).await?;
    Ok(web::Json(meetings))
}

#[derive(OpenApi)]
#[openapi(
    paths(index),
    components(schemas(
        SearchMeeting,
        meeting::Meeting,
        meeting::OnlineOptions,
        meeting::MeetingTime,
        meeting::WeekDay,
        meeting::Contact,
        meeting::Location,
        meeting::Position,
        meeting::Organization
    ))
)]
struct ApiDoc;

pub async fn start_server(
    meeting_index: MeetingIndex,
    address: IpAddr,
    port: u16,
) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let openapi = ApiDoc::openapi();

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

            .service(
                SwaggerUi::new("/{_:.*}").url("openapi.json", openapi.clone()),
            )
    })
        .bind((address, port))?
        .run()
        .await
}
