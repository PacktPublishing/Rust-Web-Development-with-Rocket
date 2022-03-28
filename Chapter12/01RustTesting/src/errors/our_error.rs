use rocket::http::Status;
use rocket::serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use sqlx::Error as sqlxError;
use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use uuid::Error as uuidError;

#[derive(Debug)]
pub struct OurError {
    pub status: Status,
    pub message: String,
    debug: Option<Box<dyn Error>>,
}

impl fmt::Display for OurError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for OurError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if self.debug.is_some() {
            self.debug.as_ref().unwrap().source();
        }
        None
    }
}

impl Serialize for OurError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OurError", 2)?;
        state.serialize_field("status", &self.status.code)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

impl OurError {
    fn new_error_with_status(
        status: Status,
        message: String,
        debug: Option<Box<dyn Error>>,
    ) -> Self {
        if debug.is_some() {
            log::error!("Error: {:?}", &debug);
        }
        OurError {
            status,
            message,
            debug,
        }
    }
    pub fn new_bad_request_error(message: String, debug: Option<Box<dyn Error>>) -> Self {
        Self::new_error_with_status(Status::BadRequest, message, debug)
    }

    pub fn new_not_found_error(message: String, debug: Option<Box<dyn Error>>) -> Self {
        Self::new_error_with_status(Status::NotFound, message, debug)
    }

    pub fn new_internal_server_error(message: String, debug: Option<Box<dyn Error>>) -> Self {
        Self::new_error_with_status(Status::InternalServerError, message, debug)
    }

    pub fn new_unauthorized_error(debug: Option<Box<dyn Error>>) -> Self {
        Self::new_error_with_status(Status::Unauthorized, String::from("unauthorized"), debug)
    }

    pub fn from_sqlx_error(e: sqlxError) -> Self {
        match e {
            sqlxError::RowNotFound => {
                OurError::new_not_found_error(String::from("Not found"), Some(Box::new(e)))
            }
            sqlxError::Database(db) => {
                if db.code().unwrap_or(Cow::Borrowed("2300")).starts_with("23") {
                    return OurError::new_bad_request_error(
                        String::from("Cannot create or update resource"),
                        Some(Box::new(db)),
                    );
                }
                OurError::new_internal_server_error(
                    String::from("Something went wrong"),
                    Some(Box::new(db)),
                )
            }
            _ => OurError::new_internal_server_error(
                String::from("Something went wrong"),
                Some(Box::new(e)),
            ),
        }
    }

    pub fn from_uuid_error(e: uuidError) -> Self {
        OurError::new_bad_request_error(String::from("Something went wrong"), Some(Box::new(e)))
    }
}
