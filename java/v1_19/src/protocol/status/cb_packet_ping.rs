use minecraft_data::protocol::Packet;
use minecraft_data::protocol::PacketContent;
use minecraft_data::protocol::PacketSwitch;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::str::FromStr;

pub struct CbPacketPing;
impl Packet for CbPacketPing {
    type PacketIDType = i32;
    type PacketContent = PacketPingContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        1
    }
}
pub struct PacketPingContent {
    pub time: i64,
}
impl PacketContent for PacketPingContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.time.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let time: i64 = PacketContent::read(reader)?;

        Ok(Self { time })
    }
}
