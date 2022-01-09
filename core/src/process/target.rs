use crate::error::Error;
use crate::protocol::{Message, PointerEvent};
use crossbeam_channel::{select, RecvError, Sender};
use laminar::{Packet, Socket, SocketEvent};
use std::thread;

/// Process an incoming packet in the target process.
fn target_packet<OPEC>(
    msg: Result<SocketEvent, RecvError>,
    v4socket_send: &mut Sender<Packet>,
    v6socket_send: &mut Sender<Packet>,
    on_pointer_event: &mut OPEC,
) -> Result<(), Error>
where
    OPEC: FnMut(PointerEvent),
{
    match msg {
        Ok(SocketEvent::Packet(packet)) => {
            let socket_send = if packet.addr().is_ipv4() {
                v4socket_send
            } else {
                v6socket_send
            };

            match rmp_serde::from_read_ref(packet.payload())? {
                Message::SourceConnectTarget => {
                    //Reflect TargetAcknowledgeSource
                    let response = rmp_serde::to_vec(&Message::TargetAcknowledgeSource)?;

                    socket_send.send(Packet::reliable_ordered(packet.addr(), response, None))?;

                    Ok(())
                }
                Message::SourcePointerEvent(ptr_evt) => {
                    //Process pointer event
                    on_pointer_event(ptr_evt);
                    Ok(())
                }
                Message::TargetAcknowledgeSource => Err(Error::ProtocolRole),
            }
        }

        Ok(_) => Ok(()), // TODO: Report connect/disconnect events

        Err(_) => unimplemented!(), //TODO: Handle unplanned shutdown
    }
}

/// Run event target process.
///
/// This is a process function, it will permenantly take over the calling
/// thread. Spawn it, and hand it callbacks to communicate with your main
/// thread.
pub fn target<OPEC, OEC>(mut on_pointer_event: OPEC, mut on_error: OEC)
where
    OPEC: FnMut(PointerEvent),
    OEC: FnMut(Error),
{
    let v4socket = Socket::bind("0.0.0.0:8192");
    if let Err(e) = v4socket {
        on_error(e.into());
        return;
    }

    let mut v4socket = v4socket.unwrap();

    let v6socket = Socket::bind("[::]:8192");
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

    loop {
        let maybe_error = select! {
            recv(v4socket_recv) -> msg => target_packet(msg, &mut v4socket_send, &mut v6socket_send, &mut on_pointer_event),
            recv(v6socket_recv) -> msg => target_packet(msg, &mut v4socket_send, &mut v6socket_send, &mut on_pointer_event)
        };

        if let Err(e) = maybe_error {
            on_error(e);
        }
    }
}
