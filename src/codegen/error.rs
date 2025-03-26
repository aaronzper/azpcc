use thiserror::Error;

#[derive(Error)]
pub enum CodegenError {
    #[error("Out of scratch registers")]
    OutOfScratch,
}
