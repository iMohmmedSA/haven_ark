pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Keychain error: {0}")]
    KeyChain(#[from] keyring::Error),

    #[error("App data error: {0}")]
    AppData(#[from] etcetera::HomeDirError),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Plex client error: {0}")]
    PlexClient(#[from] plex_client::error::Error),
}
