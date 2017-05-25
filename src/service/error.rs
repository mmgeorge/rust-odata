

#[derive(Debug)]
pub enum Error {
    NoImpl,
    InvalidRoot,
    InvalidModel,
    NotFound(String)
}
