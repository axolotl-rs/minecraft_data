use std::io::{BufRead, Write};
use crate::common::protocol::PacketContent;

pub struct BitField {}
impl PacketContent for BitField {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
        where
            Self: Sized,
    {
        todo!()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
        where
            Self: Sized,
    {
        todo!()
    }
}
