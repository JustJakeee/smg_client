// udp game client
// will have connect and send methods

use std::net::UdpSocket;
use anyhow::Result;
use glam::Vec2;
use serde::{Serialize, Deserialize};
use bincode;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Packet {
    Connect(Uuid),
    Disconnect(Uuid),
    Message(String),
    Player(PlayerState),
    List(),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PlayerState {
    pub uuid: Uuid,
    pub x: f32,
    pub y: f32,
}

impl PlayerState {
    // new take Vec2
    pub fn new(uuid: Uuid, pos: Vec2) -> Self {
        Self {
            uuid,
            x: pos.x,
            y: pos.y,
        }
    }
}

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
