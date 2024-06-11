use crate::a2s::packet::PacketBytes;
use byteorder::{LittleEndian, ReadBytesExt};
use pretty_hex::pretty_hex;
use std::error::Error;
use std::io::{BufRead, Cursor, Read};

#[allow(non_camel_case_types)]
pub struct Packet_Rules {
    bytes: PacketBytes,
    rules: Vec<Rule>,
    mods: Vec<Mod>,
}

pub struct Rule {
    pub name: String,
    pub value: String,
}

pub struct Mod {
    pub id: u32,
    pub name: String,
}

impl Packet_Rules {
    pub fn bytes(&self) -> &PacketBytes {
        &self.bytes
    }

    pub fn rules(&self) -> &Vec<Rule> {
        &self.rules
    }

    pub fn mods(&self) -> &Vec<Mod> {
        &self.mods
    }
}

impl TryFrom<PacketBytes> for Packet_Rules {
    type Error = Box<dyn Error>;

    fn try_from(mut bytes: PacketBytes) -> Result<Self, Self::Error> {
        let rules = vec![];
        let mut mods = vec![];

        let num_rules = bytes.read_u16()?;

        let mut iterating_mod_chunks = true;
        let mut bytes_mods = vec![];

        for i in 0..num_rules {
            let bytes_name = bytes.read_until_zero()?;
            let bytes_value = bytes.read_until_zero()?;

            let i_plus_one = (i + 1) as u8;
            let is_mod_chunk = iterating_mod_chunks
                && bytes_name.len() == 2
                && i_plus_one == bytes_name[0]
                && i_plus_one <= bytes_name[1];

            if !is_mod_chunk && iterating_mod_chunks {
                iterating_mod_chunks = false;
            }

            if is_mod_chunk {
                bytes_mods.extend(bytes_value);
            } else {
                // TODO
            }
        }

        let mut bytes_mods = Cursor::new(unescape(bytes_mods)?);

        bytes_mods.consume(2);
        let is_offset = bytes_mods.read_u8()? > 0;
        bytes_mods.consume(if is_offset { 5 } else { 1 });
        let num_mods = bytes_mods.read_u8()?;

        for _ in 0..num_mods {
            bytes_mods.consume(5);

            let id = bytes_mods.read_u32::<LittleEndian>()?;
            let name_len = bytes_mods.read_u8()?;
            let mut name = vec![];

            for _ in 0..name_len {
                name.push(bytes_mods.read_u8()?);
            }

            // ASCII control chars are not allowed in mod names
            if name.contains(&0x08)
                || name.contains(&0x09)
                || name.contains(&0x0A)
                || name.contains(&0x0C)
                || name.contains(&0x0D)
            {
                continue;
            }

            let name = String::from_utf8(name)?;
            mods.push(Mod { id, name })
        }

        Ok(Self { bytes, rules, mods })
    }
}

fn unescape(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    const ESCAPE_BYTE: u8 = 0x01;

    let size = data.len();
    let mut cursor = Cursor::new(data);
    let mut result = vec![];

    while (cursor.position() as usize) < size {
        let curr = cursor.read_u8()?;
        if curr == ESCAPE_BYTE {
            let next = match cursor.read_u8()? {
                0x01 => 0x01,
                0x02 => 0x00,
                0x03 => 0xFF,
                other => other,
            };
            result.push(next);
        } else {
            result.push(curr);
        }
    }

    Ok(result)
}
