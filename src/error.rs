use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("A Sdl function failed : {0}")]
    SdlError(String),
}