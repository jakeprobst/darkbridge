use std::convert::TryFrom;

use mio::*;
use mio::net::{TcpStream, TcpListener};
use std::net::{SocketAddr, Ipv4Addr};

use crate::proxy::{Proxy, SERVER, LISTENER};
use crate::packet::Packet;
use crate::items::Item;
use crate::gamecommand::{GameCommand, GameCommandAction};
use crate::commands::{Command, CommandRunner};

const LOCAL_PROXY_IP: [u8; 4] = [10, 0, 0, 179];

#[derive(Debug, Clone)]
pub enum TargettedPacket {
    Client(Packet),
    Server(Packet),
}

pub type Filter = Fn(TargettedPacket, &mut Proxy) -> Vec<TargettedPacket>;

pub fn connection_redirect(mut pkt: TargettedPacket, proxy: &mut Proxy) -> Vec<TargettedPacket> {
    if let TargettedPacket::Client(ref mut pkt) = pkt {
        if let Packet::Redirect(ref mut redirect) = pkt {
            println!("redirecting! {:?}:{}", redirect.ip, redirect.port);
            let new_sock = TcpStream::connect(SocketAddr::from((redirect.ip, redirect.port))).unwrap();
            //poll.registry().deregister(&self.server).unwrap();
            proxy.server = new_sock;
            proxy.poll.registry().register(&mut proxy.server, SERVER, Interest::READABLE).unwrap();

            proxy.server2proxy = None;
            proxy.proxy2server = None;

            redirect.ip = LOCAL_PROXY_IP;
            let mut ls = TcpListener::bind(SocketAddr::from((Ipv4Addr::new(0,0,0,0), 0))).unwrap();
            redirect.port = ls.local_addr().unwrap().port();
            println!("re-redirecting! {:?}:{}", redirect.ip, redirect.port);
            proxy.poll.registry().register(&mut ls, LISTENER, Interest::READABLE).unwrap();
            proxy.poll.registry().deregister(&mut proxy.server).unwrap();
            proxy.poll.registry().deregister(&mut proxy.gamecube).unwrap();
            println!("listening on: {:?}", ls);
            proxy.listener = Some(ls);
        }
    }
    vec![pkt]
}

pub fn save_position(pkt: TargettedPacket, proxy: &mut Proxy) -> Vec<TargettedPacket> {
    if let TargettedPacket::Server(ref spkt) = pkt {
        if let Packet::GameCommand(cmd) = spkt {
            if let GameCommandAction::PlayerStop(ref action) = cmd.cmd {
                proxy.gamestate.position.x = action.x;
                proxy.gamestate.position.y = action.y;
                proxy.gamestate.position.z = action.z;
            }
            if let GameCommandAction::PlayerWalk(ref action) = cmd.cmd {
                proxy.gamestate.position.x = action.x;
                proxy.gamestate.position.z = action.z;
            }
            if let GameCommandAction::PlayerRun(ref action) = cmd.cmd {
                proxy.gamestate.position.x = action.x;
                proxy.gamestate.position.z = action.z;
            }
            if let GameCommandAction::PlayerArea(ref action) = cmd.cmd {
                proxy.gamestate.floor = action.floor;
            }
        }
    }

    vec![pkt]
}


pub fn chat_command(pkt: TargettedPacket, proxy: &mut Proxy) -> Vec<TargettedPacket> {
    if let TargettedPacket::Server(ref spkt) = pkt {
        if let Packet::ChatMessage(chatmsg) = spkt {
            if chatmsg.message.starts_with("/") {
                println!("chat msg! {:?}", chatmsg.message);
                let command = Command::parse(chatmsg.message.to_ascii_lowercase().chars().skip(1).collect());
                let mut commandrunner = CommandRunner::new();
                return command.map(|command| {
                    commandrunner.run(command, proxy)
                }).unwrap_or(Vec::new())
            }
        }
    }
    vec![pkt]
}

pub fn update_inventory(pkt: TargettedPacket, proxy: &mut Proxy) -> Vec<TargettedPacket> {
    if let TargettedPacket::Server(ref spkt) = pkt {
        if let Packet::PlayerInventory(inventory_data) = spkt {
            proxy.gamestate.inventory = inventory_data.data.iter()
                //.skip(0x3C-4)
                .skip(0x10-4)
                .array_chunks::<28>()
                .filter_map(|chunk| {
                    //dbg!(&chunk);
                    let item = Item::try_from(chunk).ok();
                    //dbg!(&item);
                    item
                })
                .collect();
            dbg!(&proxy.gamestate.inventory);
        }
    }
    vec![pkt]
}
