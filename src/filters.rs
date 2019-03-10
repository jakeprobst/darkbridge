use mio::*;
use mio::tcp::{TcpStream, TcpListener};
use std::net::{SocketAddr, Ipv4Addr};

use crate::proxy::{Proxy, SERVER, LISTENER};
use crate::packet::Packet;
use crate::gamecommand::{GameCommand, GameCommandAction};

#[derive(Debug, Clone)]
pub enum TargettedPacket {
    Client(Packet),
    Server(Packet),
}

pub type Filter = Fn(TargettedPacket, &mut Proxy) -> Vec<TargettedPacket>;
//pub type Filter = fn(Option<TargettedPacket>, GameState) -> (Option<TargettedPacket>);


pub fn connection_redirect(mut pkt: TargettedPacket, proxy: &mut Proxy) -> Vec<TargettedPacket> {
    if let TargettedPacket::Client(ref mut pkt) = pkt {
        if let Packet::Redirect(ref mut redirect) = pkt {
            println!("redirecting! {:?}:{}", redirect.ip, redirect.port);
            let new_sock = TcpStream::connect(&SocketAddr::from((redirect.ip, redirect.port))).unwrap();
            //poll.deregister(&self.server).unwrap();
            proxy.server = new_sock;
            proxy.poll.register(&proxy.server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

            proxy.server2proxy = None;
            proxy.proxy2server = None;

            redirect.ip = [192, 168, 1, 10];
            let ls = TcpListener::bind(&SocketAddr::from((Ipv4Addr::new(0,0,0,0), 0))).unwrap();
            redirect.port = ls.local_addr().unwrap().port();
            println!("re-redirecting! {:?}:{}", redirect.ip, redirect.port);
            proxy.poll.register(&ls, LISTENER, Ready::readable(), PollOpt::edge()).unwrap();
            proxy.poll.deregister(&proxy.server).unwrap();
            println!("listening on: {:?}", ls);
            proxy.listener = Some(ls);
        }
    }
    vec![pkt]
}

/*pub fn set_encryption(pkt: TargettedPacket, proxy: &mut Proxy) -> Vec<TargettedPacket> {
    if let TargettedPacket::Client(ref cpkt) = pkt {
        if let Packet::EncryptionKeys(keys) = cpkt {
            println!("encryption keys filter! c: {:08X} s: {:08X}", keys.client_seed, keys.server_seed);
            proxy.server2proxy = Some(Cipher::new(keys.server_seed));
            proxy.proxy2server = Some(Cipher::new(keys.client_seed));
            //self.proxy2gamecube = Some(Cipher::new(keys.server_seed));
            //self.gamecube2proxy = Some(Cipher::new(keys.client_seed));
            //return Vec::new();
        }
    }
    vec![pkt]
}*/


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
        println!("player pos: {:?}", proxy.gamestate.position);
    }

    vec![pkt]
}






/*fn passthrough(pkt: Option<Packet>, gamestate: &mut GameState) -> Option<Packet> {




    pkt
}
*/
