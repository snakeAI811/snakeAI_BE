mod api_error;
mod db_error;
mod request_error;

pub use api_error::*;
pub use db_error::*;
pub use request_error::*;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
