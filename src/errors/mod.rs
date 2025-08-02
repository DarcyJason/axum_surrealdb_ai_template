pub mod api;
pub mod auth;
pub mod db;
pub mod jwt;
pub mod response;

#[derive(Debug)]
pub enum Error {}

impl Error {
    pub fn error_code(self) -> String {
        match self {}
    }
}

pub type Result<T> = std::result::Result<T, Error>;
