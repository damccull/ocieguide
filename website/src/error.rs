mod response_error;

pub use response_error::ResponseError;

use http::StatusCode;

type StdError = Box<dyn std::error::Error>;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub fn e400<T>(e: T) -> ResponseError
where
    T: std::fmt::Debug,
    T: std::fmt::Display + 'static,
    T: Into<StdError>,
{
    ResponseError::from(e).set_status(StatusCode::BAD_REQUEST)
}

pub fn e500<T>(e: T) -> ResponseError
where
    T: std::fmt::Debug,
    T: std::fmt::Display + 'static,
    T: Into<StdError>,
{
    // ResponseBadRequestError::from(e)
    ResponseError::from(e)
}
