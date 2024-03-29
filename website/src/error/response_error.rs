use std::fmt::{Debug, Display};

use axum::response::IntoResponse;
use http::StatusCode;

use super::{error_chain_fmt, StdError};

pub struct ResponseError {
    status_code: StatusCode,
    internal_error: StdError,
}

impl ResponseError {
    pub fn new(status_code: StatusCode, internal_error: StdError) -> Self {
        Self {
            status_code,
            internal_error,
        }
    }

    pub fn set_status(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{:?}", self);
        (self.status_code, self.internal_error.to_string()).into_response()
    }
}

impl<E> From<E> for ResponseError
where
    E: Into<StdError>,
{
    fn from(value: E) -> Self {
        let internal_error: StdError = value.into();
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            internal_error,
        }
    }
}

impl Debug for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(&self.internal_error.as_ref(), f)
    }
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.internal_error.to_string())
    }
}
