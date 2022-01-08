use crate::error::Error;
use crate::protocol::{Message, PointerEvent};
use laminar::{Packet, Socket, SocketEvent};
use std::thread;

/// Run event target process.
///
/// This is a process function, it will permenantly take over the calling
/// thread. Spawn it, and hand it callbacks to communicate with your main
/// thread.
pub fn target<OPEC, OEC>(mut on_pointer_event: OPEC, mut on_error: OEC) -> Result<(), Error>
where
    OPEC: FnMut(PointerEvent),
    OEC: FnMut(Error),
{
    //TODO: IPv6
    //TODO: bind all

    let mut v4socket = Socket::bind("0.0.0.0:8192")?;
    let mut v6socket = Socket::bind("[::]:8192")?;

    let v4socket_recv = v4socket.get_event_receiver();
    let v6socket_recv = v6socket.get_event_receiver();
    let mut v4socket_send = v4socket.get_packet_sender();
    let mut v6socket_send = v6socket.get_packet_sender();

    thread::spawn(move || v4socket.start_polling());
    thread::spawn(move || v6socket.start_polling());

    loop {
        match v4socket_recv
            .try_recv()
            .or_else(|_| v6socket_recv.try_recv())
        {
            Ok(SocketEvent::Packet(packet)) => {
                let socket_send = if packet.addr().is_ipv4() {
                    &mut v4socket_send
                } else {
                    &mut v6socket_send
                };

                match rmp_serde::from_read_ref(packet.payload()) {
                    Ok(Message::SourceConnectTarget) => {
                        //Reflect TargetAcknowledgeSource
                        let response = rmp_serde::to_vec(&Message::TargetAcknowledgeSource);
                        if let Err(e) = response {
                            on_error(e.into());
                            continue;
                        }

                        let response = response.unwrap();

                        //TODO: Can the sender actually be shut down?
                        socket_send
                            .send(Packet::reliable_ordered(packet.addr(), response, None))
                            .unwrap()
                    }
                    Ok(Message::SourcePointerEvent(ptr_evt)) => {
                        //Process pointer event
                        on_pointer_event(ptr_evt);
                    }
                    Ok(Message::TargetAcknowledgeSource) => {
                        on_error(Error::ProtocolRole);
                    }
                    Err(_) => {}
                }
            }

            // Connect/disconnect/timeout
            Ok(_) => {}
            // Both channels are empty
            Err(_) => {}
        }
    }
}
