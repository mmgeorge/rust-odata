//! Provides the wrapper around hyper that is used for creating an OData
//! service. 

mod service;
mod handler;
mod error;

pub use self::service::Service;
pub use self::service::ServiceBuilder;
pub use self::error::Error;
pub use self::handler::Res;
