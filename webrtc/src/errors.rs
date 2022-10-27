use thiserror::Error;

/// All camera error.
#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug, Clone)]
pub enum MediaStreamError {
    #[error("Unitialized Camera. Call `init()` first!")]
    UnitializedError,
    #[error("Could not initialize")]
    InitializeError,
    #[error("Could not shutdown")]
    ShutdownError,
    #[error("Error: {0}")]
    GeneralError(String),
    #[error("Could not generate required structure {structure}: {error}")]
    StructureError { structure: String, error: String },
    #[error("Could not open device {0}: {1}")]
    OpenDeviceError(String, String),
    #[error("Could not get device property {property}: {error}")]
    GetPropertyError { property: String, error: String },
    #[error("Could not set device property {property} with value {value}: {error}")]
    SetPropertyError {
        property: String,
        value: String,
        error: String,
    },
    #[error("Could not open device stream: {0}")]
    OpenStreamError(String),
    #[error("Could not capture frame: {0}")]
    ReadFrameError(String),
    #[error("Could not stop stream: {0}")]
    StreamShutdownError(String),
    #[error("This operation is not supported.")]
    UnsupportedOperationError,
    #[error("This operation is not implemented yet: {0}")]
    NotImplementedError(String),
}
