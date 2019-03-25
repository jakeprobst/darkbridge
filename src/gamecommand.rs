use std::io::Cursor;
use std::io::{Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian, BigEndian};

use crate::packet::PacketData;

trait GameCommandData {
    fn parse(cmd: u8, data: &Vec<u8>) -> Self where Self: Sized;
    fn as_bytes(&self) -> Vec<u8>;
}


struct GameCommandBytes {
    cmd: u8,
    buffer: Vec<u8>,
}

impl GameCommandBytes {
    fn new() -> GameCommandBytes {
        GameCommandBytes {
            cmd: 0,
            buffer: Vec::new(),
        }
    }

    fn cmd(mut self, c: u8) -> GameCommandBytes {
        self.cmd = c;
        self
    }

    fn _u32(mut self, item: u32) -> GameCommandBytes {
        self.buffer.write_u32::<LittleEndian>(item).unwrap();
        self
    }

    fn _u32_be(mut self, item: u32) -> GameCommandBytes {
        self.buffer.write_u32::<BigEndian>(item).unwrap();
        self
    }

    fn _f32(mut self, item: f32) -> GameCommandBytes {
        self.buffer.write_f32::<LittleEndian>(item).unwrap();
        self
    }

    fn build(self) -> Vec<u8> {
        // TODO: assert size divisible by 4 bytes?
        let mut buf = Vec::new();
        buf.write_u8(self.cmd).unwrap();
        buf.write_u8(self.buffer.len() as u8 / 4 + 1).unwrap();
        buf.write_u8(0).unwrap();
        buf.write_u8(0).unwrap();
        buf.extend(self.buffer);
        buf
    }
}


struct GameCommandParser {
    cursor: Cursor<Vec<u8>>,
}

impl GameCommandParser {
    fn new(data: Vec<u8>) -> GameCommandParser {
        GameCommandParser {
            cursor: Cursor::new(data)
        }
    }

    fn _u32(&mut self) -> u32 {
        self.cursor.read_u32::<LittleEndian>().unwrap()
    }

    fn _f32(&mut self) -> f32 {
        self.cursor.read_f32::<LittleEndian>().unwrap()
    }
}


#[derive(Debug, Clone)]
pub struct PlayerArea {
    pub floor: u32,
}

impl GameCommandData for PlayerArea {
    fn parse(_gcmd: u8, data: &Vec<u8>) -> PlayerArea {
        let mut parser = GameCommandParser::new(data.clone());
        PlayerArea {
            floor: parser._u32(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        GameCommandBytes::new()
            .cmd(0x1F)
            ._u32(self.floor)
            .build()
    }
}

#[derive(Debug, Clone)]
pub struct PlayerWalk {
    pub x: f32,
    pub z: f32,
    unknown: f32,
}

impl GameCommandData for PlayerWalk {
    fn parse(_gcmd: u8, data: &Vec<u8>) -> PlayerWalk {
        let mut parser = GameCommandParser::new(data.clone());
        PlayerWalk {
            x: parser._f32(),
            z: parser._f32(),
            unknown: parser._f32(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        GameCommandBytes::new()
            .cmd(0x40)
            ._f32(self.x)
            ._f32(self.z)
            ._f32(self.unknown)
            .build()
    }
}

#[derive(Debug, Clone)]
pub struct PlayerRun {
    pub x: f32,
    pub z: f32,
}

impl GameCommandData for PlayerRun {
    fn parse(_gcmd: u8, data: &Vec<u8>) -> PlayerRun {
        let mut parser = GameCommandParser::new(data.clone());
        PlayerRun {
            x: parser._f32(),
            z: parser._f32(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        GameCommandBytes::new()
            .cmd(0x40)
            ._f32(self.x)
            ._f32(self.z)
            .build()
    }
}

#[derive(Debug, Clone)]
pub struct PlayerStop {
    unknown1: u32,
    unknown2: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl GameCommandData for PlayerStop {
    fn parse(_gcmd: u8, data: &Vec<u8>) -> PlayerStop {
        let mut parser = GameCommandParser::new(data.clone());
        PlayerStop {
            unknown1: parser._u32(),
            unknown2: parser._u32(),
            x: parser._f32(),
            y: parser._f32(),
            z: parser._f32(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        GameCommandBytes::new()
            .cmd(0x3E)
            ._u32(self.unknown1)
            ._u32(self.unknown2)
            ._f32(self.x)
            ._f32(self.y)
            ._f32(self.z)
            .build()
    }
}

#[derive(Debug, Clone)]
pub struct RawGameCommand {
    gcmd: u8,
    data: Vec<u8>,
}

impl GameCommandData for RawGameCommand {
    fn parse(gcmd: u8, data: &Vec<u8>) -> RawGameCommand {
        RawGameCommand {
            gcmd: gcmd,
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
pub struct ItemDrop {
    pub floor: u32,
    pub x: f32,
    pub z: f32,
    pub item_row1: u32,
    pub item_row2: u32,
    pub item_row3: u32,
    pub itemdrop_id: u32,
    pub item_row4: u32,
    pub unknown: u32,
}

impl /*GameCommandData for*/ ItemDrop {
    /*fn parse(gcmd: u8, data: &Vec<u8>) -> ItemDrop {
        ItemDrop {
        }
}*/

    //fn make_wep()
    //fn make_armor()
    //fn make_shield()
    //fn make_tech()
    //fn make_tool()
    //fn make_mag()
    //fn make_meseta()

    fn as_bytes(&self) -> Vec<u8> {
        GameCommandBytes::new()
            .cmd(0x5D)
            ._u32(self.floor)
            ._f32(self.x)
            ._f32(self.z)
            ._u32_be(self.item_row1)
            ._u32_be(self.item_row2)
            ._u32_be(self.item_row3)
            ._u32(self.itemdrop_id)
            ._u32_be(self.item_row4)
            ._u32(self.unknown)
            .build()
        // TODO: self.size is based on 32 bit items, `assert len(self.data) % 4 == 0`?
        /*let mut buf: Vec<u8> = Vec::new();
        buf.write_u8(0x5D).unwrap();
        buf.write_u8(0x0A).unwrap();
        buf.write_u8(0).unwrap();
        buf.write_u8(0).unwrap();
        buf*/
    }
}


#[derive(Debug, Clone)]
pub enum GameCommandAction {
    PlayerWalk(PlayerWalk),
    PlayerRun(PlayerRun),
    PlayerStop(PlayerStop),
    PlayerArea(PlayerArea),
    ItemDrop(ItemDrop),

    RawGameCommand(RawGameCommand),
}

#[derive(Debug, Clone)]
pub struct GameCommand {
    pub flag: u8,
    pub client: u8,
    pub unknown: u8,
    pub cmd: GameCommandAction,
}

impl PacketData for GameCommand {
    fn parse(_cmd: u8, flag: u8, data: &Vec<u8>) -> GameCommand {
        let mut cur = Cursor::new(data);
        let gcmd = cur.read_u8().unwrap();
        let _size = cur.read_u8().unwrap();
        let client = cur.read_u8().unwrap();
        let unknown = cur.read_u8().unwrap();

        let mut cmd_data = Vec::new();
        cur.read_to_end(&mut cmd_data).unwrap();

        GameCommand {
            flag: flag,
            client: client,
            unknown: unknown,
            cmd: match gcmd {
                0x1F => GameCommandAction::PlayerArea(PlayerArea::parse(gcmd, &cmd_data)),
                0x3E => GameCommandAction::PlayerStop(PlayerStop::parse(gcmd, &cmd_data)),
                0x40 => GameCommandAction::PlayerWalk(PlayerWalk::parse(gcmd, &cmd_data)),
                0x42 => GameCommandAction::PlayerRun(PlayerRun::parse(gcmd, &cmd_data)),
                _ => GameCommandAction::RawGameCommand(RawGameCommand::parse(gcmd, &cmd_data)),
            }
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut data = match &self.cmd {
            GameCommandAction::PlayerArea(cmd) => cmd.as_bytes(),
            GameCommandAction::PlayerStop(cmd) => cmd.as_bytes(),
            GameCommandAction::PlayerWalk(cmd) => cmd.as_bytes(),
            GameCommandAction::PlayerRun(cmd) => cmd.as_bytes(),
            GameCommandAction::ItemDrop(cmd) => cmd.as_bytes(),
            GameCommandAction::RawGameCommand(cmd) => cmd.as_bytes(),
        };

        data[0x02] = self.client;
        data[0x03] = self.unknown;

        let mut buf: Vec<u8> = Vec::new();
        buf.write_u8(0x60).unwrap();
        buf.write_u8(self.flag).unwrap();
        buf.write_u16::<LittleEndian>(0x04 + data.len() as u16).unwrap();
        buf.write(&data).unwrap();
        buf
    }
}
