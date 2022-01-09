use crate::error::Error;
use crate::protocol::{Message, PointerEvent};
use crossbeam_channel::{select, Receiver, RecvError, Sender};
use laminar::{Packet, Socket, SocketEvent};
use std::net::SocketAddr;
use std::thread;

/// A command sent to control the source process.
pub enum SourceCommand {
    /// Connect to a new event target.
    Connect(SocketAddr),

    /// Send a pointer event to the target.
    Pointer(PointerEvent),
}

/// Process an incoming command in the source process.
fn source_command(
    msg: Result<SourceCommand, RecvError>,
    v4socket_send: &mut Sender<Packet>,
    v6socket_send: &mut Sender<Packet>,
    socket_target: &mut Option<SocketAddr>,
) -> Result<(), Error> {
    match msg? {
        SourceCommand::Connect(addr) => {
            let payload = rmp_serde::to_vec(&Message::SourceConnectTarget)?;

            *socket_target = Some(addr);

            if addr.is_ipv6() {
                v6socket_send.send(Packet::reliable_ordered(addr, payload, None))?
            } else {
                v4socket_send.send(Packet::reliable_ordered(addr, payload, None))?
            }

            Ok(())
        }
        SourceCommand::Pointer(event) => {
            if let Some(addr) = socket_target {
                let payload = rmp_serde::to_vec(&Message::SourcePointerEvent(event))?;

                if addr.is_ipv6() {
                    v6socket_send.send(Packet::reliable_ordered(*addr, payload, None))?
                } else {
                    v4socket_send.send(Packet::reliable_ordered(*addr, payload, None))?
                }
            }

            Ok(())
        }
    }
}

/// Process an incoming packet in the source process.
fn source_packet(_msg: Result<SocketEvent, RecvError>) -> Result<(), Error> {
    Ok(())
}

/// Event source process.
///
/// Commands from your source application are piped into the receiver you
/// provide. See `SourceCommand` for more info.
pub fn source<OEC>(cmd_source: Receiver<SourceCommand>, mut on_error: OEC)
where
    OEC: FnMut(Error),
{
    let v4socket = Socket::bind("0.0.0.0:0");
    if let Err(e) = v4socket {
        on_error(e.into());
        return;
    }

    let mut v4socket = v4socket.unwrap();

    let v6socket = Socket::bind("[::]:0");
    if let Err(e) = v6socket {
        on_error(e.into());
        return;
    }

    let mut v6socket = v6socket.unwrap();

    let v4socket_recv = v4socket.get_event_receiver();
    let v6socket_recv = v6socket.get_event_receiver();
    let mut v4socket_send = v4socket.get_packet_sender();
    let mut v6socket_send = v6socket.get_packet_sender();

    thread::spawn(move || v4socket.start_polling());
    thread::spawn(move || v6socket.start_polling());

    let mut socket_target = None;

    loop {
        let maybe_error = select! {
            recv(cmd_source) -> msg => source_command(msg, &mut v4socket_send, &mut v6socket_send, &mut socket_target),
            recv(v4socket_recv) -> msg => source_packet(msg),
            recv(v6socket_recv) -> msg => source_packet(msg),
        };

        if let Err(e) = maybe_error {
            on_error(e);
        }
    }
}
