use std::io::Cursor;
use std::io::{Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian, BigEndian};



#[derive(Debug, Clone)]
pub struct Redirect {
    pub ip: [u8; 4],
    pub port: u16,
}




#[derive(Debug, Clone)]
pub struct RawData {
    pub cmd: u8,
    pub flag: u8,
    pub len: u16,
    pub data: Vec<u8>
}


#[derive(Debug, Clone)]
pub struct AllowDenyAccess{
    pub allow: u8
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

#[derive(Debug, Clone)]
pub enum Packet {
    Redirect(Redirect),
    EncryptionKeys(EncryptionKeys),
    AllowDenyAccess(AllowDenyAccess),

    RawData(RawData)
}


impl Packet {
    pub fn parse(cmd: u8, flag: u8, len: u16, data: &Vec<u8>) -> Packet {
        let mut cur = Cursor::new(data.clone());
        if cmd == 0x19 {  // Redirect
            cur.set_position(0x04);
            let port = cur.read_u16::<LittleEndian>().unwrap();
            Packet::Redirect(Redirect {
                ip: [data[0x00], data[0x01], data[0x02], data[0x03]],
                port: port,
            })
        }
        else if cmd == 0x17 || cmd == 0x02 { // welcome msg (enc keys!)
            //cur.set_position(0x40);
            let mut msg = vec![0u8; 0x40];
            cur.read(&mut msg).unwrap();
            let sseed = cur.read_u32::<LittleEndian>().unwrap();
            let cseed = cur.read_u32::<LittleEndian>().unwrap();
            let mut secret_msg = Vec::new();
            cur.read_to_end(&mut secret_msg).unwrap();
            Packet::EncryptionKeys(EncryptionKeys {
                cmd: cmd,
                welcome_msg: msg,
                client_seed: cseed,
                server_seed: sseed,
                secret_msg: secret_msg,
            })
        }
        else if cmd == 0x9A {
            Packet::AllowDenyAccess(AllowDenyAccess {
                allow: flag,
            })
        }
        else {
            Packet::RawData(RawData {
                cmd: cmd,
                flag: flag,
                len: len,
                data: cur.into_inner(),
            })
        }
    }


    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        match *self {
            Packet::Redirect(ref redirect) => {
                buf.write_u16::<LittleEndian>(0x19).unwrap();
                buf.write_u16::<LittleEndian>(0x0C).unwrap();
                buf.write_u8(redirect.ip[0]).unwrap();
                buf.write_u8(redirect.ip[1]).unwrap();
                buf.write_u8(redirect.ip[2]).unwrap();
                buf.write_u8(redirect.ip[3]).unwrap();
                buf.write_u16::<LittleEndian>(redirect.port).unwrap();
                buf.write_u16::<LittleEndian>(0x00).unwrap();
            },
            Packet::EncryptionKeys(ref enc_keys) => {
                buf.write_u8(enc_keys.cmd).unwrap();
                buf.write_u8(0).unwrap();
                buf.write_u16::<LittleEndian>(0x4C + enc_keys.secret_msg.len() as u16).unwrap();
                //buf.write_u16::<LittleEndian>(0x4C).unwrap();
                buf.write(&enc_keys.welcome_msg).unwrap();
                buf.write_u32::<LittleEndian>(enc_keys.server_seed).unwrap();
                buf.write_u32::<LittleEndian>(enc_keys.client_seed).unwrap();
                buf.write(&enc_keys.secret_msg).unwrap();
                //buf.write_u8(0).unwrap();
            }
            Packet::AllowDenyAccess(ref allowdeny) => {
                buf.write_u8(0x9A).unwrap();
                buf.write_u8(allowdeny.allow).unwrap();
                buf.write_u16::<LittleEndian>(0x04).unwrap();
                
            }
            Packet::RawData(ref data) => {
                buf.write_u8(data.cmd).unwrap();
                buf.write_u8(data.flag).unwrap();
                buf.write_u16::<LittleEndian>(data.len).unwrap();
                buf.extend(&data.data);
            },
        }
        buf
    }
}
