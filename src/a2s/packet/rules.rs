use crate::a2s::packet::PacketBytes;
use pretty_hex::pretty_hex;
use std::error::Error;

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
}

impl TryFrom<PacketBytes> for Packet_Rules {
    type Error = Box<dyn Error>;

    fn try_from(mut bytes: PacketBytes) -> Result<Self, Self::Error> {
        let num_rules = bytes.read_u16()?;

        let mut iterating_mod_list = true;
        let mut bytes_mod_list = vec![];

        for i in 0..num_rules {
            let bytes_name = bytes.read_until_zero()?;
            let bytes_value = bytes.read_until_zero()?;

            let i_plus_one = (i + 1) as u8;
            let is_mod = iterating_mod_list
                && bytes_name.len() == 2
                && i_plus_one == bytes_name[0]
                && i_plus_one <= bytes_name[1];

            if !is_mod && iterating_mod_list {
                iterating_mod_list = false;
            }

            println!();
            println!("Rule ({i}), is_mod={is_mod}");
            println!("{}", pretty_hex(&bytes_name));
            println!("{}", pretty_hex(&bytes_value));

            if is_mod {
                bytes_mod_list.extend(bytes_value);
            } else {
            }
        }

        let bytes_mod_list = mod_unescape(bytes_mod_list);

        println!();
        println!("{}", pretty_hex(&bytes_mod_list));

        let rules = vec![];
        let mods = vec![];

        Ok(Self { bytes, rules, mods })
    }
}

fn mod_unescape(data: Vec<u8>) -> Vec<u8> {
    data
}
