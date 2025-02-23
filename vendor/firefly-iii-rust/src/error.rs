use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    HTTPErr(#[from] ureq::Error),
    #[error(transparent)]
    IOErr(#[from] std::io::Error),
}

impl From<&ureq::Error> for Error {
    fn from(err: &ureq::Error) -> Self {
        err.into()
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        err.into()
    }
}

impl From<u16> for Error {
    fn from(err: u16) -> Self {
        err.into()
    }
}
