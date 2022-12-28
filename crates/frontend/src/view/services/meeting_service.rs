use thiserror::Error;

use common::model::SearchMeeting;

#[derive(Error, Debug, Clone)]
pub enum MeetingApiError {
    #[error("Http error")]
    HttpError,

    #[error("Json parse error")]
    JsonParseError,
}

pub async fn get_meetings(api: &str) -> Result<Vec<SearchMeeting>, MeetingApiError> {
    let result: Vec<SearchMeeting> = reqwest::get(format!("{api}/meetings"))
        .await
        .map_err(|_| MeetingApiError::HttpError)?
        .json()
        .await
        .map_err(|_| MeetingApiError::JsonParseError)?;

    Ok(result)
}