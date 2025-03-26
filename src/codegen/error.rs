use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("Out of scratch registers")]
    OutOfScratch,
}
