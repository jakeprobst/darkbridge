use std::io::Cursor;
use std::io::{Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian, BigEndian};


pub trait GameCommandData {
    fn parse(cmd: u8, data: &Vec<u8>) -> Self where Self: Sized;
    fn as_bytes(&self) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct PlayerWalk {
}

#[derive(Debug, Clone)]
pub struct PlayerRun {
}

#[derive(Debug, Clone)]
pub struct PlayerStop {
    unknown1: u32,
    unknown2: u32,
    x: f32,
    y: f32,
    z: f32,
}

impl GameCommandData for PlayerStop {
    fn parse(gcmd: u8, data: &Vec<u8>) -> PlayerStop {
        let mut cur = Cursor::new(data);
        let unknown1 = cur.read_u32::<LittleEndian>().unwrap();
        let unknown2 = cur.read_u32::<LittleEndian>().unwrap();
        let x = cur.read_f32::<LittleEndian>().unwrap();
        let y = cur.read_f32::<LittleEndian>().unwrap();
        let z = cur.read_f32::<LittleEndian>().unwrap();

        PlayerStop {
            unknown1: unknown1,
            unknown2: unknown2,
            x: x,
            y: y,
            z: z,
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.write_u8(0x3E).unwrap();
        buf.write_u8(6).unwrap();
        buf.write_u8(0).unwrap();
        buf.write_u8(0).unwrap();
        buf.write_u32::<LittleEndian>(self.unknown1).unwrap();
        buf.write_u32::<LittleEndian>(self.unknown2).unwrap();
        buf.write_f32::<LittleEndian>(self.x).unwrap();
        buf.write_f32::<LittleEndian>(self.y).unwrap();
        buf.write_f32::<LittleEndian>(self.z).unwrap();
        buf
    }
}

#[derive(Debug, Clone)]
pub struct RawGameCommand {
    gcmd: u8,
    data: Vec<u8>,
}

impl GameCommandData for RawGameCommand {
    fn parse(gcmd: u8, data: &Vec<u8>) -> RawGameCommand {
        //let mut cur = Cursor::new(data.clone());
        //cur.set_position(0x04);
        //let mut command_data = Vec::new();
        //cur.read_to_end(&mut command_data).unwrap();
        RawGameCommand {
            gcmd: gcmd,
            //client: data[0x02],
            //size: data[0x02],
            data: data.clone(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // TODO: self.size is based on 32 bit items, `assert len(self.data) % 4 == 0`?
        let mut buf: Vec<u8> = Vec::new();
        buf.write_u8(self.gcmd).unwrap();
        buf.write_u8((1 + self.data.len() / 4) as u8).unwrap();
        buf.write_u8(0).unwrap();
        buf.write_u8(0).unwrap();
        buf.write(&self.data).unwrap();
        buf
    }
}

#[derive(Debug, Clone)]
pub enum GameCommandAction {
    //PlayerWalk(PlayerWalk), // 64
    //PlayerRun(PlayerRun), // 66
    PlayerStop(PlayerStop), // 62

    RawGameCommand(RawGameCommand),
}

#[derive(Debug, Clone)]
pub struct GameCommand {
    pub flag: u8,
    pub client: u8,
    pub cmd: GameCommandAction,
}
