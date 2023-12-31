use std::io::{Read as _, Write as _};

use bevy::{
    ecs::event::Event,
    math::{Quat, Vec2, Vec3},
};
use bevy_matchbox::matchbox_socket::PeerId;
use serde::{Deserialize, Serialize};

const MAX_UNCOMPRESSED_SIZE: usize = 256;

#[derive(Debug, Event)]
pub struct Connected {
    pub peer_id: PeerId,
}

#[derive(Debug, Event)]
pub struct Disconnected {
    pub peer_id: PeerId,
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct PlayerState {
    pub id: PeerId,
    pub position: Vec3,
    pub rotation: Quat,
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct EnemyState {
    pub id: u16,
    pub position: Vec2,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct BulletState {
    pub id: u32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub enum NetworkEvent {
    PlayerState(PlayerState),
    EnemyState(EnemyState),
    BulletState(BulletState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetPacket(pub Vec<NetworkEvent>);

// compress with flate if >256 bytes
pub fn net_packet_to_bytes(data: &NetPacket) -> Box<[u8]> {
    let mut data = bincode::serialize(data).unwrap();
    let compressed = data.len() > MAX_UNCOMPRESSED_SIZE;
    if compressed {
        let mut encoder =
            flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&data).unwrap();
        data = encoder.finish().unwrap();
        data.push(1);
    } else {
        data.push(0);
    }

    data.into_boxed_slice()
}

pub fn bytes_to_net_packet(data: &[u8]) -> Option<NetPacket> {
    let compressed = data.last().unwrap() == &1;
    let data = if compressed {
        let mut decoder = flate2::read::ZlibDecoder::new(&data[..data.len() - 1]);
        let mut data = Vec::new();
        decoder.read_to_end(&mut data).unwrap();
        data
    } else {
        data[..data.len() - 1].to_vec()
    };

    bincode::deserialize(&data).ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_net_packet() {
        // let data = to_net_packet(&"hello");
        // let data = from_net_packet::<String>(&data);
        // assert_eq!(data.unwrap(), "hello");
    }
}
