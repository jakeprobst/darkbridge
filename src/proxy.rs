use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{RawFd, AsRawFd};
use std::net;
use std::net::{SocketAddr, Ipv4Addr};
use mio::*;
use mio::tcp::{TcpStream, TcpListener};
use mio::unix::EventedFd;
use std::io::{Read, Write, Cursor, BufReader, BufRead};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use nix::unistd;
use nix::sys::stat;

use crate::packet::Packet;
use crate::cipher::Cipher;

//const PSOPORT: u16 = 9410;
const PSOPORT: u16 = 9100;

const GAMECUBE: Token = Token(0);
const SERVER: Token = Token(1);
const LISTENER: Token = Token(2);
const CMDPIPE: Token = Token(3);

use std::time::Duration;

pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

pub struct PlayerData {
    position: Option<Position>
}

impl PlayerData {
    pub fn new() -> PlayerData {
        PlayerData {
            position: None,
        }
    }
}

pub struct Proxy {
    gamecube: TcpStream,
    server: TcpStream,
    cmd_pipe: File,
    playerdata: PlayerData,
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
    println!("get_packet");
    let mut local_cipher = cipher.clone();

    let mut header = vec![0u8; 4];
    if let Err(_e) = sock.peek(&mut header) {
        println!("could not read header");
        return None;
    }

    if let Some(ref mut cipher) = local_cipher {
        header = cipher.encrypt(&header.to_vec());
    };

    let mut cur = Cursor::new(header.clone());
    let cmd = cur.read_u8().unwrap();
    let flag = cur.read_u8().unwrap();
    let len = cur.read_u16::<LittleEndian>().unwrap();

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

fn send_packet(sock: &mut TcpStream, pkt: &Packet, cipher: &mut Option<Cipher>) {
    println!("sending to {:?}", sock);
    let mut buf = pkt.as_bytes();
    print_buffer(&buf);
    if let Some(ref mut cipher) = cipher {
        buf = cipher.encrypt(&buf);
    }
    while let Err(_) = sock.write_all(&buf) {
        continue;
    }
}

impl Proxy {
    pub fn new(sock: net::TcpStream) -> Proxy {
        //let server = TcpStream::connect(&SocketAddr::from((Ipv4Addr::new(172,245,5,200), PSOPORT))).unwrap();
        let server = TcpStream::connect(&SocketAddr::from((Ipv4Addr::new(99,199,199,42), PSOPORT))).unwrap();

        // TODO: delete and remake
        let _ = unistd::mkfifo("/tmp/darkbridge", stat::Mode::S_IRWXU);
        let cmd_pipe = OpenOptions::new()
            .custom_flags(libc::O_NONBLOCK)
            .read(true)
            .open("/tmp/darkbridge").unwrap();
        println!("does it hang?");
        
        Proxy {
            gamecube: TcpStream::from_stream(sock).unwrap(),
            server: server,
            cmd_pipe: cmd_pipe,
            playerdata: PlayerData::new(),
        }
    }

    fn handle_client_packet(&mut self, pkt: Packet) -> Option<Packet> {
        Some(pkt)
    }
    
    fn handle_server_packet(&mut self, pkt: Packet) -> Option<Packet> {
        Some(pkt)
    }

    pub fn run(&mut self) {
        let poll = Poll::new().unwrap();

        poll.register(&self.gamecube, GAMECUBE, Ready::readable(), PollOpt::edge()).unwrap();
        poll.register(&self.server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();
        poll.register(&EventedFd(&self.cmd_pipe.as_raw_fd()), CMDPIPE, Ready::readable(), PollOpt::edge()).unwrap();

        let mut listener = None;
        
        let mut events = Events::with_capacity(64);

        let mut server2proxy = None;
        let mut proxy2server = None;
        let mut gamecube2proxy = None;
        let mut proxy2gamecube = None;

        loop {
            poll.poll(&mut events, None).unwrap();
            
            for event in events.iter() {
                match event.token() {
                    GAMECUBE => {
                        println!("[GAMECUBE]");
                        while let Some(mut pkt) = get_packet(&self.gamecube, &mut gamecube2proxy) {
                            println!("gc! {:?}", pkt);

                            if let Some(p) = self.handle_client_packet(pkt) {
                                send_packet(&mut self.server, &p, &mut proxy2server);
                            }
                        }
                    },
                    SERVER => {
                        println!("[SERVER]");
                        while let Some(mut pkt) = get_packet(&self.server, &mut server2proxy) {
                            println!("serv! {:?}", pkt);
                            if let Packet::Redirect(ref mut redirect) = pkt {
                                println!("redirecting! {:?}:{}", redirect.ip, redirect.port);
                                let new_sock = TcpStream::connect(&SocketAddr::from((redirect.ip, redirect.port))).unwrap();
                                //poll.deregister(&self.server).unwrap();
                                self.server = new_sock;
                                poll.register(&self.server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

                                server2proxy = None;
                                proxy2server = None;

                                redirect.ip = [192, 168, 1, 10];
                                let ls = TcpListener::bind(&SocketAddr::from((Ipv4Addr::new(0,0,0,0), 0))).unwrap();
                                redirect.port = ls.local_addr().unwrap().port();
                                println!("re-redirecting! {:?}:{}", redirect.ip, redirect.port);
                                poll.register(&ls, LISTENER, Ready::readable(), PollOpt::edge()).unwrap();
                                poll.deregister(&self.server).unwrap();
                                println!("listening on: {:?}", ls);
                                listener = Some(ls);
                            }

                            if let Some(p) = self.handle_server_packet(pkt.clone()) {
                                send_packet(&mut self.gamecube, &p, &mut proxy2gamecube);
                            }
                           
                            if let Packet::EncryptionKeys(ref keys) = pkt {
                                println!("encryption keys! c: {:08X} s: {:08X}", keys.client_seed, keys.server_seed);
                                server2proxy = Some(Cipher::new(keys.server_seed));
                                proxy2server = Some(Cipher::new(keys.client_seed));
                                proxy2gamecube = Some(Cipher::new(keys.server_seed));
                                gamecube2proxy = Some(Cipher::new(keys.client_seed));
                            }
                        }
                    },
                    LISTENER => {
                        println!("[LISTENER]");
                        if let Some(listener) = listener {
                            println!("accepting!");
                            self.gamecube = listener.accept().unwrap().0;
                            println!("accepted new gc: {:?}", self.gamecube);
                            server2proxy = None;
                            proxy2server = None;
                            gamecube2proxy = None;
                            proxy2gamecube = None;
                            poll.register(&self.gamecube, GAMECUBE, Ready::readable(), PollOpt::edge()).unwrap();
                            poll.register(&self.server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();
                            //listener.shutdown();
                        }
                        listener = None;
                    },
                    CMDPIPE => {
                        println!("[CMDPIPE]");
                        let cmdbuf = BufReader::new(&self.cmd_pipe);
                        for cmd in cmdbuf.lines() {
                            //println!("cmd: {}", cmd.unwrap());
                            /*for (target, pkt) in commands::handle_command(cmd) {
                            }*/
                        }
                    }
                    _ => unreachable!()
                }
            }
        }
    }
}
