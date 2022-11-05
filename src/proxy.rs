use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{RawFd, AsRawFd};
use std::net;
use std::net::{SocketAddr, Ipv4Addr};
use mio::Poll;
use mio::*;
use mio::net::{TcpStream, TcpListener};
use mio::unix::SourceFd;
use std::io::{Read, Write, Cursor, BufReader, BufRead};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use nix::unistd;
use nix::sys::stat;

use crate::filters;
use crate::filters::TargettedPacket;
use crate::packet::Packet;
use crate::cipher::Cipher;
use crate::commands::{Command, CommandRunner};

const PSOPORT: u16 = 9100;

// unseen
//const TARGET_SERVER: Ipv4Addr = Ipv4Addr::new(47, 87, 165, 199);
// scht
const TARGET_SERVER: Ipv4Addr = Ipv4Addr::new(149, 56, 167, 128);
// elsewhere
//const TARGET_SERVER: Ipv4Addr = Ipv4Addr::new(45, 33, 31, 247);


pub const GAMECUBE: Token = Token(0);
pub const SERVER: Token = Token(1);
pub const LISTENER: Token = Token(2);
pub const CMDPIPE: Token = Token(3);

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct GameState {
    pub self_client: u8,
    pub floor: u32,
    pub position: Position,
    pub itemdrop_id: u32,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            self_client: 0,
            floor: 0,
            position: Position {x:0.0, y:0.0, z:0.0},
            itemdrop_id: 0x11223344,
        }
    }
}

pub struct Proxy {
    pub gamecube: TcpStream,
    pub server: TcpStream,
    pub listener: Option<TcpListener>,
    //cmd_pipe: File,
    pub poll: Poll,

    pub gamestate: GameState,

    pub server2proxy: Option<Cipher>,
    pub proxy2server: Option<Cipher>,
    pub gamecube2proxy: Option<Cipher>,
    pub proxy2gamecube: Option<Cipher>,
}

pub fn print_buffer(pkt: &Vec<u8>) {
    for (i, row) in pkt.chunks(16).enumerate() {
        let mut hexbuf = Vec::new();
        let mut asciibuf = Vec::new();
        for item in row {
            hexbuf.push(format!("{:02X}", item));
            asciibuf.push(format!("{}", if *item > 0x20 && *item < 0x7E {*item as char} else {'.'}));
        }
        println!("{:04X} | {:47} | {:16} |", i*16, hexbuf.join(" "), asciibuf.join(""));
    }
}

fn get_packet(mut sock: &TcpStream, cipher: &mut Option<Cipher>) -> Option<Packet> {
    let mut local_cipher = cipher.clone();

    let mut header = vec![0u8; 4];
    match sock.peek(&mut header) {
        Ok(len) if len != 4 => return None,
        Err(_) => return None,
        _ => {}
    }

    if let Some(ref mut cipher) = local_cipher {
        header = cipher.encrypt(&header.to_vec());
    };

    let mut cur = Cursor::new(header.clone());
    let cmd = cur.read_u8().unwrap();
    let flag = cur.read_u8().unwrap();
    let len = cur.read_u16::<LittleEndian>().unwrap();

    if len == 0 {
        return None
    }

    let mut peek_buf = vec![0u8; len as usize];
    let peek_len = match sock.peek(&mut peek_buf) {
        Ok(l) => l,
        Err(_e) => return None
    };

    if len as usize != peek_len {
        return None;
    }

    let mut header_enc = vec![0u8; 4];
    let mut data_buf = vec![0u8; len as usize - 4];
    sock.read_exact(&mut header_enc).unwrap();
    sock.read_exact(&mut data_buf).unwrap();

    if let Some(ref mut cipher) = local_cipher {
        data_buf = cipher.encrypt(&data_buf.to_vec());
    };

    *cipher = local_cipher;

    let pkt = Packet::parse(cmd, flag, len, &data_buf);
    print_buffer(&header.into_iter().chain(data_buf.into_iter()).collect());

    Some(pkt)
}

fn send_packet(sock: &mut TcpStream, pkt: &Packet, cipher: &mut Option<Cipher>) -> Result<(), std::io::Error> {
    println!("sending to {:?}", sock);
    let mut buf = pkt.as_bytes();
    print_buffer(&buf);
    if let Some(ref mut cipher) = cipher {
        buf = cipher.encrypt(&buf);
    }

    loop {
        match sock.write_all(&buf) {
            Err(err) => {
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    continue
                }
                else {
                    println!("erroring {:?}!", err);
                    return Err(err)
                }
            },
            _ => return Ok(()),
        }
    }
}

impl Proxy {
    pub fn new(sock: net::TcpStream) -> Proxy {
        let server = TcpStream::connect(SocketAddr::from((TARGET_SERVER, PSOPORT))).unwrap();

        Proxy {
            gamecube: TcpStream::from_std(sock),
            server: server,
            listener: None,
            //cmd_pipe: cmd_pipe,
            poll: Poll::new().unwrap(),
            gamestate: GameState::new(),
            server2proxy: None,
            proxy2server: None,
            gamecube2proxy: None,
            proxy2gamecube: None,
        }
    }

    fn filter_packet(&mut self, filters: &Vec<Box<filters::Filter>>, pkt: TargettedPacket) -> Vec<TargettedPacket> {
        let mut pkts = vec![pkt];
        for filter in filters.iter() {
            let mut result_pkts = Vec::new();
            for p in pkts {
                result_pkts.extend(filter(p, self));
            }
            pkts = result_pkts;
        }
        pkts
    }

    fn send_packets(&mut self, pkts: Vec<TargettedPacket>) -> Result<(), std::io::Error>{
        for pkt in pkts {
            match pkt {
                TargettedPacket::Client(p) => {
                    send_packet(&mut self.gamecube, &p, &mut self.proxy2gamecube)?;

                    if let Packet::EncryptionKeys(ref keys) = p {
                        println!("encryption keys! c: {:08X} s: {:08X}", keys.client_seed, keys.server_seed);
                        self.server2proxy = Some(Cipher::new(keys.server_seed));
                        self.proxy2server = Some(Cipher::new(keys.client_seed));
                        self.proxy2gamecube = Some(Cipher::new(keys.server_seed));
                        self.gamecube2proxy = Some(Cipher::new(keys.client_seed));
                    }
                },
                TargettedPacket::Server(p) => {
                    send_packet(&mut self.server, &p, &mut self.proxy2server)?;
                }
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let _ = unistd::mkfifo("/tmp/darkbridge", stat::Mode::S_IRWXU);
        let mut cmd_pipe = OpenOptions::new()
            .custom_flags(libc::O_NONBLOCK)
            .read(true)
            .open("/tmp/darkbridge").unwrap();

        self.poll.registry().register(&mut self.gamecube, GAMECUBE, Interest::READABLE).unwrap();
        self.poll.registry().register(&mut self.server, SERVER, Interest::READABLE).unwrap();
        self.poll.registry().register(&mut SourceFd(&cmd_pipe.as_raw_fd()), CMDPIPE, Interest::READABLE).unwrap();

        let mut commandrunner = CommandRunner::new();

        let mut filters: Vec<Box<filters::Filter>> = Vec::new();
        filters.push(Box::new(filters::connection_redirect));
        filters.push(Box::new(filters::save_position));

        let mut events = Events::with_capacity(64);

        loop {
            self.poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {
                    GAMECUBE => {
                        println!("[GAMECUBE]");
                        while let Some(pkt) = get_packet(&self.gamecube, &mut self.gamecube2proxy) {
                            println!("gc! {:?}", pkt);
                            let filtered_pkts = self.filter_packet(&filters, TargettedPacket::Server(pkt));
                            self.send_packets(filtered_pkts)?;
                        }
                    },
                    SERVER => {
                        println!("[SERVER]");
                        while let Some(pkt) = get_packet(&self.server, &mut self.server2proxy) {
                            println!("serv! {:?}", pkt);
                            let filtered_pkts = self.filter_packet(&filters, TargettedPacket::Client(pkt));
                            self.send_packets(filtered_pkts)?;
                        }
                    },
                    LISTENER => {
                        println!("[LISTENER]");
                        if let Some(ref mut listener) = self.listener {
                            self.gamecube = listener.accept().unwrap().0;
                            println!("accepted new gc: {:?}", self.gamecube);
                            self.server2proxy = None;
                            self.proxy2server = None;
                            self.gamecube2proxy = None;
                            self.proxy2gamecube = None;
                            self.poll.registry().register(&mut self.gamecube, GAMECUBE, Interest::READABLE).unwrap();
                            self.poll.registry().register(&mut self.server, SERVER, Interest::READABLE).unwrap();
                            //listener.shutdown();
                        }
                        self.listener = None;
                    },
                    CMDPIPE => {
                        println!("[CMDPIPE]");
                        let cmdbuf = BufReader::new(&mut cmd_pipe);
                        for cmd in cmdbuf.lines() {
                            let command = Command::parse(cmd.unwrap().to_ascii_lowercase());
                            match command {
                                Ok(c) => {
                                    let pkts = commandrunner.run(c, self);
                                    self.send_packets(pkts)?;
                                },
                                Err(err) => println!("!!! command error: {:?}", err),
                            }
                        }
                    }
                    _ => unreachable!()
                }
            }
        }
    }
}
