use crossbeam_channel::{RecvError as CrossbeamRecvError, SendError as CrossbeamSendError};
use laminar::{ErrorKind as LaminarError, Packet};
use rmp_serde::decode::Error as RmpDecodeError;
use rmp_serde::encode::Error as RmpEncodeError;

pub enum Error {
    Encoding(RmpEncodeError),
    Transmission(LaminarError),
    Decoding(RmpDecodeError),
    ChannelSend(CrossbeamSendError<Packet>),
    ChannelRecv(CrossbeamRecvError),
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

impl From<CrossbeamSendError<Packet>> for Error {
    fn from(err: CrossbeamSendError<Packet>) -> Self {
        Self::ChannelSend(err)
    }
}

impl From<CrossbeamRecvError> for Error {
    fn from(err: CrossbeamRecvError) -> Self {
        Self::ChannelRecv(err)
    }
}
