
/// Error types for handler
#[derive(Debug)]
pub enum Error {
    NoImpl,
    InvalidRoot,
    InvalidModel,
    InvalidParameter,
    NotFound(String)
}
