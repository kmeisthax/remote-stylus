use laminar::ErrorKind as LaminarError;
use rmp_serde::encode::Error as RmpEncodeError;

pub enum Error {
    Encoding(RmpEncodeError),
    Transmission(LaminarError),
    ProtocolRole,
}

impl From<LaminarError> for Error {
    fn from(err: LaminarError) -> Self {
        Self::Transmission(err)
    }
}

impl From<RmpEncodeError> for Error {
    fn from(err: RmpEncodeError) -> Self {
        Self::Encoding(err)
    }
}
