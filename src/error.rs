#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("failed to recieve data from event listener: {0}")]
    TryRecvError(#[from] std::sync::mpsc::TryRecvError),
}

