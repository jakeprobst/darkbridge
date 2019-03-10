use std::io::Cursor;
use std::io::{Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian, BigEndian};
use crate::gamecommand::*;

pub trait PacketData {
    fn parse(cmd: u8, flag: u8, data: &Vec<u8>) -> Self where Self: Sized;
    fn as_bytes(&self) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct Redirect {
    pub ip: [u8; 4],
    pub port: u16,
}

impl PacketData for Redirect {
    fn parse(cmd: u8, flag: u8, data: &Vec<u8>) -> Redirect {
        let mut cur = Cursor::new(data.clone());
        cur.set_position(0x04);
        let port = cur.read_u16::<LittleEndian>().unwrap();
        Redirect {
            ip: [data[0x00], data[0x01], data[0x02], data[0x03]],
            port: port,
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.write_u16::<LittleEndian>(0x19).unwrap();
        buf.write_u16::<LittleEndian>(0x0C).unwrap();
        buf.write_u8(self.ip[0]).unwrap();
        buf.write_u8(self.ip[1]).unwrap();
        buf.write_u8(self.ip[2]).unwrap();
        buf.write_u8(self.ip[3]).unwrap();
        buf.write_u16::<LittleEndian>(self.port).unwrap();
        buf.write_u16::<LittleEndian>(0x00).unwrap();
        buf
    }
}

#[derive(Debug, Clone)]
pub struct RawData {
    pub cmd: u8,
    pub flag: u8,
    pub len: u16,
    pub data: Vec<u8>
}

impl PacketData for RawData {
    fn parse(cmd: u8, flag: u8, data: &Vec<u8>) -> RawData {
        RawData {
            cmd: cmd,
            flag: flag,
            len: (data.len() + 4) as u16,
            data: data.clone()
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.write_u8(self.cmd).unwrap();
        buf.write_u8(self.flag).unwrap();
        buf.write_u16::<LittleEndian>(self.len).unwrap();
        buf.extend(&self.data);
        buf
    }
}

#[derive(Debug, Clone)]
pub struct AllowDenyAccess{
    pub allow: u8
}

impl PacketData for AllowDenyAccess {
    fn parse(cmd: u8, flag: u8, data: &Vec<u8>) -> AllowDenyAccess {
        AllowDenyAccess {
            allow: flag,
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.write_u8(0x9A).unwrap();
        buf.write_u8(self.allow).unwrap();
        buf.write_u16::<LittleEndian>(0x04).unwrap();
        buf
    }
}

#[derive(Debug, Clone)]
pub struct EncryptionKeys {
    //welcome_msg: [u8; 0x40],
    pub cmd: u8,
    pub welcome_msg: Vec<u8>,
    pub client_seed: u32,
    pub server_seed: u32,
    pub secret_msg: Vec<u8>,
}

impl PacketData for EncryptionKeys {
    fn parse(cmd: u8, flag: u8, data: &Vec<u8>) -> EncryptionKeys {
        let mut cur = Cursor::new(data.clone());
        let mut msg = vec![0u8; 0x40];
        cur.read(&mut msg).unwrap();
        let sseed = cur.read_u32::<LittleEndian>().unwrap();
        let cseed = cur.read_u32::<LittleEndian>().unwrap();
        let mut secret_msg = Vec::new();
        cur.read_to_end(&mut secret_msg).unwrap();
        EncryptionKeys {
            cmd: cmd,
            welcome_msg: msg,
            client_seed: cseed,
            server_seed: sseed,
            secret_msg: secret_msg,
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.write_u8(self.cmd).unwrap();
        buf.write_u8(0).unwrap();
        buf.write_u16::<LittleEndian>(0x4C + self.secret_msg.len() as u16).unwrap();
        buf.write(&self.welcome_msg).unwrap();
        buf.write_u32::<LittleEndian>(self.server_seed).unwrap();
        buf.write_u32::<LittleEndian>(self.client_seed).unwrap();
        buf.write(&self.secret_msg).unwrap();
        buf
    }
}




#[derive(Debug, Clone)]
pub struct ChatMessage {

}

#[derive(Debug, Clone)]
pub enum Packet {
    Redirect(Redirect),
    EncryptionKeys(EncryptionKeys),
    AllowDenyAccess(AllowDenyAccess),

    GameCommand(GameCommand),
    //ChatMessage(ChatMessage),

    RawData(RawData)
}



impl Packet {
    pub fn parse(cmd: u8, flag: u8, len: u16, data: &Vec<u8>) -> Packet {
        match cmd {
            0x60 => Packet::GameCommand(GameCommand::parse(cmd, flag, data)),
            0x19 => Packet::Redirect(Redirect::parse(cmd, flag, data)),
            0x17 | 0x02 => Packet::EncryptionKeys(EncryptionKeys::parse(cmd, flag, data)),
            0x9A => Packet::AllowDenyAccess(AllowDenyAccess::parse(cmd, flag, data)),
            _ => Packet::RawData(RawData::parse(cmd, flag, data))
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Packet::Redirect(pkt) => pkt.as_bytes(),
            Packet::EncryptionKeys(pkt) => pkt.as_bytes(),
            Packet::AllowDenyAccess(pkt) => pkt.as_bytes(),
            Packet::RawData(pkt) => pkt.as_bytes(),
            Packet::GameCommand(pkt) => pkt.as_bytes(),
        }
    }
}
