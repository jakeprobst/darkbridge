use std::convert::TryFrom;

use crate::filters::TargettedPacket;
use crate::proxy::Proxy;
use crate::proxy::{GameState, Position};
use crate::packet::{Packet, RawData, PacketData};
use crate::gamecommand::{GameCommand, GameCommandAction, ItemDrop};
use crate::items::*;


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
    item: Box<dyn ItemData>,
}

impl MakeItem {
    fn parse_weapon(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let weapon = WeaponType::try_from(*item_cmd.get(1).ok_or(ItemParseError::MissingParameter)?)?;

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

    fn parse_esweapon(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let weapon = ESWeaponType::try_from(*item_cmd.get(1).ok_or(ItemParseError::MissingParameter)?)?;
        let (special, name, grind) = item_cmd.iter().skip(2)
            .fold((None, None, None), |(mut special, mut name, mut grind), cmd| {
                if special.is_none() {
                    if let Ok(spec) = ESWeaponSpecial::try_from(*cmd) {
                        special = Some(spec);
                        return (special, name, grind);
                    }
                }

                if grind.is_none() {
                    if let Ok(gr) = cmd.parse::<u8>() {
                        grind = Some(gr);
                        return (special, name, grind);
                    }
                }

                if name.is_none() {
                    let mut out = [0u8; 8];
                    for (i, k) in cmd.to_ascii_uppercase().as_bytes().into_iter().enumerate().take(8) {
                        out[i] = *k;
                    }

                    name = Some(out);
                    return (special, name, grind);
                }

                (special, name, grind)
            });

        Ok(MakeItem {
            item: Box::new(ESWeapon {
                weapon,
                special,
                grind: grind.unwrap_or(0),
                name: name.unwrap_or([0; 8]),
            })})
    }

    fn parse_armor(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let armor = ArmorType::try_from(item_cmd[1])?;

        let (dfp, evp, slots) = item_cmd.iter().skip(2)
            .try_fold((0, 0, 0), |(mut dfp, mut evp, mut slots), cmd| {
                if cmd.ends_with("d") {
                    dfp = cmd[..cmd.len()-1].parse::<u8>()?;
                }
                if cmd.ends_with("e") {
                    evp = cmd[..cmd.len()-1].parse::<u8>()?;
                }
                if cmd.ends_with("s") {
                    slots = cmd[..cmd.len()-1].parse::<u8>()?;
                }
                Ok::<_, ItemParseError>((dfp, evp, slots))
            })?;

        Ok(MakeItem {
            item: Box::new(Armor {
                armor,
                dfp,
                evp,
                slots
            })
        })
    }

    fn parse_shield(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let shield = ShieldType::try_from(item_cmd[1])?;
        let (dfp, evp) = item_cmd.iter().skip(2)
            .try_fold((0, 0), |(mut dfp, mut evp), cmd| {
                if cmd.ends_with("d") {
                    dfp = cmd[..cmd.len()-1].parse::<u8>()?;
                }
                if cmd.ends_with("e") {
                    evp = cmd[..cmd.len()-1].parse::<u8>()?;
                }
                Ok::<_, ItemParseError>((dfp, evp))
            })?;

        Ok(MakeItem {
            item: Box::new(Shield {
                shield,
                dfp,
                evp,
            })
        })
    }

    fn parse_unit(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let unit = UnitType::try_from(item_cmd[1])?;

        let umod = item_cmd.get(2).and_then(|cmd| {
            match *cmd {
                "++" => Some(UnitModifier::PlusPlus),
                "+" => Some(UnitModifier::Plus),
                "-" => Some(UnitModifier::Minus),
                "--" => Some(UnitModifier::MinusMinus),
                _ => None,
            }
        });

        Ok(MakeItem {
            item: Box::new(Unit {
                unit,
                umod,
            })
        })
    }


    fn parse_tech(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let tech = TechType::try_from(item_cmd[1])?;
        let level = item_cmd.get(2).and_then(|cmd| {
            cmd.parse::<u8>().ok()
        }).unwrap_or(1) - 1;

        Ok(MakeItem {
            item: Box::new(Tech {
                tech: tech,
                level: level,
            })
        })
    }

    fn parse_tool(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let tool = ToolType::try_from(item_cmd[1])?;
        let stack = match item_cmd.get(2) {
            Some(s) => s.parse::<u8>()?,
            None => 0
        };

        Ok(MakeItem {
            item: Box::new(Tool {
                tool: tool,
                stack: stack,
            })
        })
    }

    fn parse_mag(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let mag = MagType::try_from(*item_cmd.get(1).ok_or(ItemParseError::MissingParameter)?)?;

        let stats = item_cmd.get(2).ok_or(ItemParseError::MissingParameter)?.split("/").collect::<Vec<_>>();
        let def = stats.get(0).ok_or(ItemParseError::MissingParameter)?.parse::<u16>()?;
        let pow = stats.get(1).ok_or(ItemParseError::MissingParameter)?.parse::<u16>()?;
        let dex = stats.get(2).ok_or(ItemParseError::MissingParameter)?.parse::<u16>()?;
        let mnd = stats.get(3).ok_or(ItemParseError::MissingParameter)?.parse::<u16>()?;

        let mut pb_idx = 0;
        let mut pbs: [Option<PhotonBlast>; 3] = [None, None, None];
        for pb_cmd in item_cmd.iter().skip(3) {
            if let Ok(pb) = PhotonBlast::try_from(*pb_cmd) {
                pbs[pb_idx] = Some(pb);
                pb_idx += 1;
            }
        }

        Ok(MakeItem {
            item: Box::new(Mag {
                mag: mag,
                iq: 200,
                sync: 120,
                def: def,
                pow: pow,
                dex: dex,
                mnd: mnd,
                pbs: pbs,
                color: MagColor::Null,
            })
        })
    }

    fn parse_meseta(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let amount = item_cmd[1].parse::<u32>()?;

        Ok(MakeItem {
            item: Box::new(Meseta {
                amount
            })
        })
    }

    fn parse_raw(item_cmd: Vec<&str>) -> Result<MakeItem, ItemParseError> {
        let mut data = Vec::new();
        for value in item_cmd.iter().skip(1) {
            data.extend(hex::decode(value)?);
        }

        Ok(MakeItem {
            item: Box::new(RawItemData {
                data: data,
            })
        })
    }

    fn as_packet(&self, floor: u32, position: Position, item_id: u32) -> Packet {
        Packet::GameCommand(GameCommand {
            flag: 0,
            client: 0,
            unknown: 0,
            cmd: GameCommandAction::ItemDrop(ItemDrop {
                floor: floor,
                x: position.x,
                z: position.z,
                y: position.y,
                item_row1: self.item.row1(),
                item_row2: self.item.row2(),
                item_row3: self.item.row3(),
                itemdrop_id: item_id,
                item_row4: self.item.row4(),
                unknown: 2,
            })})
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


fn restore_parse(cmd: Vec<&str>) -> Result<Vec<ToolType>, CommandError> {
    Ok(cmd
        .iter()
        .flat_map(|cmd| {
            cmd.split(",")
        })
        .filter_map(|cmd| {
            match cmd {
                "mm" => Some(ToolType::Monomate),
                "dm" => Some(ToolType::Dimate),
                "tm" => Some(ToolType::Trimate),
                "mf" => Some(ToolType::Monofluid),
                "df" => Some(ToolType::Difluid),
                "tf" => Some(ToolType::Trifluid),
                "sa" => Some(ToolType::SolAtomizer),
                "ma" => Some(ToolType::MoonAtomizer),
                "sd" => Some(ToolType::ScapeDoll),
                _ => None
            }
        })
       .collect())

}


#[derive(Debug)]
pub enum Command {
    MakeItem(MakeItem),
    Restore(Vec<ToolType>),
    RawPacket(RawPacket),
}

impl Command {
    pub fn parse(data: String) -> Result<Command, CommandError> {
        println!("parse! {:?}", data);
        let split = data.split(" ").collect::<Vec<_>>();

        match split[0] {
            "weapon" => Ok(Command::MakeItem(MakeItem::parse_weapon(split)?)),
            "esweapon" => Ok(Command::MakeItem(MakeItem::parse_esweapon(split)?)),
            "tech" => Ok(Command::MakeItem(MakeItem::parse_tech(split)?)),
            "armor" => Ok(Command::MakeItem(MakeItem::parse_armor(split)?)),
            "shield" => Ok(Command::MakeItem(MakeItem::parse_shield(split)?)),
            "unit" => Ok(Command::MakeItem(MakeItem::parse_unit(split)?)),
            "mag" => Ok(Command::MakeItem(MakeItem::parse_mag(split)?)),
            "tool" => Ok(Command::MakeItem(MakeItem::parse_tool(split)?)),
            "meseta" => Ok(Command::MakeItem(MakeItem::parse_meseta(split)?)),
            "rawitem" => Ok(Command::MakeItem(MakeItem::parse_raw(split)?)),
            "raw" => Ok(Command::RawPacket(RawPacket::parse(split)?)),
            "restore" => Ok(Command::Restore(restore_parse(split)?)),
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
        match cmd {
            Command::MakeItem(makeitem) => {
                let pkt = makeitem.as_packet(proxy.gamestate.floor, proxy.gamestate.position, proxy.gamestate.item_id());
                let mut result = Vec::new();
                result.push(TargettedPacket::Client(pkt.clone()));
                result.push(TargettedPacket::Server(pkt));
                result
            },
            Command::Restore(restore) => {
                let restore_items = restore
                    .into_iter()
                    .filter_map(|tool_type| {
                        let amount_in_inventory = proxy.gamestate.inventory
                            .iter()
                            .filter_map(|item| {
                                match item {
                                    Item::Tool(tool, amount) if *tool == tool_type => Some(*amount),
                                    _ => None
                                }
                            })
                            .next()
                            .unwrap_or(0);

                        let amount = tool_type.max_stack() - amount_in_inventory;
                        if amount > 0 {
                            Some(MakeItem {item: Box::new(Tool {tool: tool_type, stack: amount })})
                        }
                        else {
                            None
                        }
                    })
                    .chain(std::iter::once(MakeItem {item: Box::new(Meseta {amount: 999999})}))
                    .collect::<Vec<_>>();

                let total_size = restore_items.len() as f32;
                restore_items
                    .into_iter()
                    .enumerate()
                    .flat_map(|(i, makeitem)| {
                        let mut position = proxy.gamestate.position;

                        position.x += (2.0*std::f32::consts::PI*((i as f32)/total_size)).sin() * 12.0;
                        position.z += (2.0*std::f32::consts::PI*((i as f32)/total_size)).cos() * 12.0;

                        let pkt = makeitem.as_packet(proxy.gamestate.floor, position, proxy.gamestate.item_id());
                        vec![TargettedPacket::Client(pkt.clone()), TargettedPacket::Server(pkt)]
                    })
                    .collect()
            }
            Command::RawPacket(raw) => {
                vec![raw.as_packet()]
            }
        }
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
            dbg!(makeitem.as_packet(gs.floor, gs.position, gs.item_id()));
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

    #[test]
    fn mags() {
        let cmd = Command::parse("mag sato 5/145/50/0 leilla pilla twins".to_string());
        dbg!(&cmd);

        let mut gs = GameState::new();
        gs.floor = 1;
        gs.position.x = 5.0;
        gs.position.z = 7.0;

        if let Ok(Command::MakeItem(ref makeitem)) = cmd {
            dbg!(&makeitem.item);
            println!("{:08X}", makeitem.item.row1());
            println!("{:08X}", makeitem.item.row2());
            println!("{:08X}", makeitem.item.row3());
            println!("{:08X}", makeitem.item.row4());
            dbg!(makeitem.as_packet(gs.floor, gs.position, gs.item_id()));
        };
    }
}




















