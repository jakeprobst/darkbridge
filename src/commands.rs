#[allow(dead_code)]

use crate::filters::TargettedPacket;
use crate::proxy::Proxy;
use crate::proxy::GameState;
use crate::packet::{Packet, RawData, PacketData};
use crate::gamecommand::{GameCommand, GameCommandAction, ItemDrop};
use crate::items::*;



use std::convert::TryFrom;


#[derive(Debug)]
pub enum CommandError {
    UnknownCommand(String),
    UnknownTarget(String),
    ItemParseError(ItemParseError),
    HexError(hex::FromHexError),
}

impl From<ItemParseError> for CommandError {
    fn from(err: ItemParseError) -> CommandError {
        CommandError::ItemParseError(err)
    }
}

impl From<hex::FromHexError> for CommandError {
    fn from(err: hex::FromHexError) -> CommandError {
        CommandError::HexError(err)
    }
}


#[derive(Debug)]
pub struct MakeItem {
    //item_hex: u32,
    //item_row1: u32,
    //item_row2: u32,
    //item_row3: u32,
    item: Box<ItemData>,
}

impl MakeItem {
    /*fn parse(item_cmd: Vec<&str>) -> MakeItem {
        let item = None;

        

        if let Ok(weapontype) = WeaponType::try_from(item_cmd[1]) {
            
            MakeItem {
                item: item
            }
        }
        

        
        /*MakeItem {
            //item_row1: 0x009D0000,
            //item_row2: 0x00000364,
            //item_row3: 0x04640564,
            item_row1: 0x00009D00,
            item_row2: 0x64030000,
            item_row3: 0x64046405,
        }*/
    }*/

    fn parse_weapon(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let weapon = WeaponType::try_from(item_cmd[1])?;

        let mut special = None;
        let mut grind = 0;
        let mut attridx = 0;
        let mut attrs = [None, None, None];

        for cmd in item_cmd.iter().skip(2) {
            if let Ok(spec) = WeaponSpecial::try_from(*cmd) {
                special = Some(spec)
            };

            if cmd.chars().nth(0) == Some('+') {
                if let Ok(g) = cmd.chars().skip(1).collect::<String>().parse::<u8>() {
                    grind = g;
                }
            }

            if let Ok(attr) = WeaponAttribute::try_from(*cmd) {
                attrs[attridx] = Some(attr);
                attridx += 1;
            }
        }

        Ok(MakeItem {
            item: Box::new(Weapon {
                weapon: weapon,
                special: special,
                grind: grind,
                attrs: attrs
            })})
    }

    fn parse_tech(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let tech = TechType::try_from(item_cmd[1])?;
        let level = item_cmd[2].parse::<u8>()?;
        
        Ok(MakeItem {
            item: Box::new(Tech {
                tech: tech,
                level: level,
            })
        })
    }
    
    fn as_packet(&self, gamestate: &mut GameState) -> TargettedPacket {
        gamestate.itemdrop_id += 0x10000;
        TargettedPacket::Client(Packet::GameCommand(GameCommand {
            flag: 0,
            client: 0,
            cmd: GameCommandAction::ItemDrop(ItemDrop {
                floor: gamestate.floor,
                x: gamestate.position.x,
                z: gamestate.position.z,
                item_row1: self.item.row1(),
                item_row2: self.item.row2(),
                item_row3: self.item.row3(),
                itemdrop_id: gamestate.itemdrop_id,
                unknown2: 0,
                unknown3: 2,
            })}))
    }
}

#[derive(Debug)]
pub struct RawPacket {
    pkt: TargettedPacket
    //data: Vec<u8>,
}

impl RawPacket {
    fn parse(cmd: Vec<&str>) -> Result<RawPacket, CommandError> {
        let mut data = Vec::new();
        let pkt_cmd = hex::decode(cmd[2])?;
        let flag = hex::decode(cmd[3])?;
        for value in cmd.iter().skip(4) {
            data.extend(hex::decode(value)?);
        }

        let raw = RawData::parse(pkt_cmd[0], flag[0], &data);
            
        
        // TODO: proper index error checking
        Ok(RawPacket {
            pkt: match cmd[1] {
                "client" => TargettedPacket::Client(Packet::RawData(raw)),
                "server" => TargettedPacket::Server(Packet::RawData(raw)),
                _ => return Err(CommandError::UnknownTarget(String::from(cmd[1])))
            }
        })
    }

    fn as_packet(&self) -> TargettedPacket {
        self.pkt.clone()
    }
}


#[derive(Debug)]
pub enum Command {
    ItemCircleStart,
    ItemCircleEnd,
    MakeItem(MakeItem),
    RawPacket(RawPacket),
}

impl Command {
    pub fn parse(data: String) -> Result<Command, CommandError> {
        println!("parse! {:?}", data);
        let split = data.split(" ").collect::<Vec<_>>();

        match split[0] {
            "weapon" => Ok(Command::MakeItem(MakeItem::parse_weapon(split)?)),
            "tech" => Ok(Command::MakeItem(MakeItem::parse_tech(split)?)),
            "raw" => Ok(Command::RawPacket(RawPacket::parse(split)?)),
            _ => Err(CommandError::UnknownCommand(data))
        }
    }
}



#[derive(Debug)]
pub struct CommandRunner {
    item_circle: Option<Vec<Command>>
}




impl CommandRunner {
    pub fn new() -> CommandRunner {
        CommandRunner {
            item_circle: None,
        }
    }
    
    pub fn run(&mut self, cmd: Command, proxy: &mut Proxy) -> Vec<TargettedPacket> {
        let mut result = Vec::new();
        match cmd {
            Command::ItemCircleStart => {
                self.item_circle = Some(Vec::new())
            },
            Command::ItemCircleEnd => {
                // wew all the packets
                self.item_circle = None
            }
            Command::MakeItem(makeitem) => {
                result.push(makeitem.as_packet(&mut proxy.gamestate));
            },
            Command::RawPacket(raw) => {
                result.push(raw.as_packet());
            }
        };
        result
    }
}






#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn weapon() {
        let cmd = Command::parse("weapon df +9 100n 100a 100h".to_string());
        let mut gs = GameState::new();
        gs.floor = 1;
        gs.position.x = 5.0;
        gs.position.z = 7.0;
        dbg!(&cmd);
        if let Ok(Command::MakeItem(ref makeitem)) = cmd {
            dbg!(&makeitem.item);
            println!("{:08X}", makeitem.item.row1());
            println!("{:08X}", makeitem.item.row2());
            println!("{:08X}", makeitem.item.row3());
            dbg!(makeitem.as_packet(&mut gs));
        };
        
        let cmd = Command::parse("weapon notreal 100n 100a 100h".to_string());
        println!("{:?}", cmd);

        let cmd = Command::parse("weapon df 100b 100a 100h".to_string());
        println!("{:?}", cmd);

        let cmd = Command::parse("weapon raygun charge +35 100b 100a 100h".to_string());
        println!("{:?}", cmd);
    }

    fn raw() {
        let cmd = Command::parse("raw client 01 05 0A FF 7F".to_string());
        let mut gs = GameState::new();
    }
}




















