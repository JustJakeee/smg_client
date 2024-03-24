// udp game client
// will have connect and send methods

use std::net::UdpSocket;
use anyhow::Result;
use glam::Vec2;
use serde::{Serialize, Deserialize};
use bincode;
use uuid::Uuid;
use smg_lib::Packet;

pub struct UdpGameClient {
    socket: UdpSocket,
}

impl UdpGameClient {
    pub fn connect(addr: &str, uuid: Uuid) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        let packet = Packet::Connect(uuid);
        socket.send(bincode::serialize(&packet)?.as_slice())?;
        Ok(Self { socket })
    }

    pub fn send(&self, packet: Packet) -> Result<()> {
        let data = bincode::serialize(&packet)?;
        let _len = self.socket.send(data.as_slice())?;
        //let mut buf = [0; 1024];
        //let (amt, _) = self.socket.recv_from(&mut buf)?;
        Ok(())
    }

    pub fn recv(&self) -> Result<Vec<u8>> {
        let mut buf = [0; 1024];
        let (amt, _) = self.socket.recv_from(&mut buf)?;
        let mut vec = Vec::with_capacity(amt);
        vec.extend_from_slice(&buf[..amt]);
        Ok(vec)
    }
}
