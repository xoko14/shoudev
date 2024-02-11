use std::{error::Error, fmt::Display};

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::templates;

const FALLBACK_PAGE: &str = include_str!("../templates/error_pages/fallback.html");

#[derive(Debug)]
pub struct ShoudevError {
    pub error_type: ShoudevErrorType,
    pub status_code: StatusCode,
}

#[derive(Debug)]
pub enum ShoudevErrorType {
    // INTERNAL
    HighlightingError,
    IntendedError,

    // EXTERNAL
    GenericNotFound,
    PostNotFound,

    // CRITICAL
    ErrorTemplateNotFound,
}

impl ShoudevError {
    pub fn new(error_type: ShoudevErrorType, status_code: StatusCode) -> Self {
        Self {
            error_type: error_type,
            status_code: status_code,
        }
    }

    pub fn err() -> Self {
        Self {
            error_type: ShoudevErrorType::IntendedError,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Display for ShoudevError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ERROR {:?}] {}", self.error_type, self.status_code)
    }
}

impl Error for ShoudevError {}

impl From<tree_sitter_highlight::Error> for ShoudevError {
    fn from(_: tree_sitter_highlight::Error) -> Self {
        Self::new(
            ShoudevErrorType::HighlightingError,
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

impl From<tera::Error> for ShoudevError {
    fn from(value: tera::Error) -> Self {
        tracing::info!("Tera error: {:?}", value.kind);
        Self::new(
            ShoudevErrorType::ErrorTemplateNotFound,
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

impl From<std::io::Error> for ShoudevError {
    fn from(value: std::io::Error) -> Self {
        Self::new(ShoudevErrorType::GenericNotFound, StatusCode::NOT_FOUND)
    }
}

impl IntoResponse for ShoudevError {
    fn into_response(self) -> axum::response::Response {
        match self.status_code {
            StatusCode::NOT_FOUND => match self.error_type {
                _ => templates::error_404(),
            },
            _ => templates::error_500(),
        }
        .map_err(map_fallback)
        .into_response()
    }
}

fn map_fallback(e: ShoudevError) -> (StatusCode, Html<String>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Html(FALLBACK_PAGE.replace("{errtype}", &e.to_string())),
    )
}
