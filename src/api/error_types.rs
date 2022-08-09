//! Error handling and stuff
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
/// Response returned when there's an error with the Datastore
pub struct DataStoreErrorResponse {
    pub error: String,
    pub message: String,
    pub errorDetails: Vec<DataStoreErrorDetail>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
/// More details of a DataStoreErrorResponse
pub struct DataStoreErrorDetail {
    pub errorDetailType: String,
    pub datastoreErrorCode: DataStoreErrorCode,
}

#[derive(Deserialize, Debug)]
/// All possible error codes regarding the Datastore API
pub enum DataStoreErrorCode {
    ContentLengthRequired,
    InvalidUniverseId,
    InvalidCursor,
    InvalidVersionId,
    ExistingValueNotNumeric,
    IncrementValueTooLarge,
    IncrementValueTooSmall,
    InvalidDataStoreScope,
    InvalidEntryKey,
    InvalidDataStoreName,
    InvalidStartTime,
    InvalidEndTime,
    InvalidAttributes,
    InvalidUserIds,
    ExclusiveCreateAndMatchVersionCannotBeSet,
    ContentTooBig,
    ChecksumMismatch,
    ContentNotJson,
    InvalidSortOrder,
    Forbidden,
    InsufficientScope,
    DatastoreNotFound,
    EntryNotFound,
    VersionNotFound,
    TooManyRequests,
    Unknown,
}

#[derive(Debug)]
/// The enum being utilized when a Datastore, Reqwest or Serde error occurs
pub enum Error {
    /// Error with the Reqwest module
    ReqwestModuleError(reqwest::Error),

    /// JSON Serialization Error
    SerdeModuleJsonError(serde_json::Error),

    /// Roblox Datastore API error
    DataStoreAPIError(DataStoreErrorResponse),
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestModuleError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeModuleJsonError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::ReqwestModuleError(e) => write!(f, "{:?}", e),
            Self::SerdeModuleJsonError(e) => write!(f, "{:?}", e),
            Self::DataStoreAPIError(e) => write!(f, "{:?}", e),
        }
    }
}

impl std::fmt::Display for DataStoreErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let details = self
            .errorDetails
            .iter()
            .map(|item| format!("{:?}", item.datastoreErrorCode))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}] - {}", details, self.message)
    }
}
