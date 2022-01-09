use laminar::ErrorKind as LaminarError;
use rmp_serde::encode::Error as RmpEncodeError;
use rmp_serde::decode::Error as RmpDecodeError;

pub enum Error {
    Encoding(RmpEncodeError),
    Transmission(LaminarError),
    Decoding(RmpDecodeError),
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

impl From<RmpDecodeError> for Error {
    fn from(err: RmpDecodeError) -> Self {
        Self::Decoding(err)
    }
}
