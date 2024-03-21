use std::fmt::Formatter;
use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParams,
    ColorSwapError,
}

#[derive(Debug)]
pub enum LoginError {
    InvalidCredentials,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse {}", err)
            }
            Error::MissingParams => {
                write!(f, "Missing parameters")
            }
            Error::ColorSwapError => {
                write!(f, "Cannot swap colors")
            }
        }
    }
}


impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            LoginError::InvalidCredentials => {
                write!(f, "Invalid Credentials")
            }
        }
    }
}

impl Reject for Error {}

impl Reject for LoginError {}