//use std::thread;
//use std::sync::{Arc, Mutex};
//use std::cell::RefCell;
use std::net;
use std::net::{SocketAddr, Ipv4Addr};
use mio::*;
use mio::tcp::{TcpStream, TcpListener};
//use std::net::TcpStream;
use std::io::{Read, Write, Cursor};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

//use connection::Connection;
use packet::Packet;
use cipher::Cipher;






//const PSOPORT: u16 = 9410;
const PSOPORT: u16 = 9100;

const GAMECUBE: Token = Token(0);
const SERVER: Token = Token(1);
const LISTENER: Token = Token(2);

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
    playerdata: PlayerData,
}


pub fn print_buffer(pkt: &Vec<u8>) {
    for row in pkt.chunks(16) {
        let mut hexbuf = Vec::new();
        let mut asciibuf = Vec::new();
        for item in row {
            hexbuf.push(format!("{:02X}", item));
            asciibuf.push(format!("{}", if *item > 0x20 && *item < 0x7E {*item as char} else {'.'}));
        }
        println!("| {:47} | {:16} |", hexbuf.join(" "), asciibuf.join(""));
    }
}


fn get_packet2(mut sock: &TcpStream, cipher: &mut Option<Cipher>) -> Option<(Packet, Vec<u8>)> {
    let mut thiscipher = cipher.clone();
    println!("get_packet-ing...");

    let mut obuf = Vec::new();
    
    println!("waiting on header...");    
    let mut header = vec![0u8; 4];
    
    if let Err(_e) = sock.peek(&mut header) {
        return None;
    }
    //println!("peeklen: {}", peeklen);
    sock.read_exact(&mut header).unwrap();
    obuf.extend(&header);
    println!("h (pre): {:02X?}", header);
    if let Some(ref mut cipher) = cipher {
        header = cipher.encrypt(&header.to_vec());
    };
    println!("h (dec): {:02X?}", header);
    let mut cur = Cursor::new(header);
    let cmd = cur.read_u8().unwrap();
    let flag = cur.read_u8().unwrap();
    let len = cur.read_u16::<LittleEndian>().unwrap();

    println!("cmd {} {} 0x{:X}", cmd, len, len);
    
    let mut buf = vec![0u8; len as usize - 4];
    println!("waiting on full packet...");
    while let Err(_a) = sock.read_exact(&mut buf) {
        continue;
    }
    obuf.extend(&buf);
    println!("data (pre): {:02X?}", buf);

    
    
    if let Some(ref mut cipher) = cipher {
        buf = cipher.encrypt(&buf.to_vec());
    };
    println!("data (dec): {:02X?}", buf);

    println!("parsing..");
    let pkt = Packet::parse(cmd, flag, len, &buf);
    println!("pkt: {:?}", pkt);
    Some((pkt, obuf))
    /*if let Packet::Redirect(ref redirect) = pkt {
    println!("moving servers...");
    sock = TcpStream::connect(&SocketAddr::from((redirect.ip, redirect.port))).unwrap();
    let mut sock2 = other_sock.lock().unwrap();
     *sock2 = sock.try_clone().unwrap();
    println!("newsock: {:?} {:?}", sock, sock2);
    set_readiness.set_readiness(Ready::readable()).unwrap();
    continue;
}*/
}


//fn get_packet(mut sock: &TcpStream, cipher: &mut Option<Cipher>) -> Option<(Packet, Vec<u8>)> {
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
        
        Proxy {
            //gamecube: sock,
            gamecube: TcpStream::from_stream(sock).unwrap(),
            server: server,
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

        let mut listener = None;
        
        let mut events = Events::with_capacity(64);

        let mut server2proxy = None;
        let mut proxy2server = None;
        let mut gamecube2proxy = None;
        let mut proxy2gamecube = None;

        //let mut buf: [u8; 1024*16] = [0; 1024*16];
        loop {
            println!("sitting at poll....");
            poll.poll(&mut events, None).unwrap();
            
            for event in events.iter() {
                println!("event!: {:?}", event);
                match event.token() {
                    GAMECUBE => {
                        println!("[GAMECUBE]");
                        //while let Some((pkt, obuf)) = get_packet(&self.gamecube, &mut gamecube2proxy) {
                        while let Some(mut pkt) = get_packet(&self.gamecube, &mut gamecube2proxy) {
                            println!("gc! {:?}", pkt);

                            /*let mut buf = pkt.as_bytes();
                            if let Some(ref mut cipher) = proxy2server {
                                buf = cipher.encrypt(&buf);
                            }
                            
                            println!("%%%%%%% same pkt? {}", obuf == buf);
                            if obuf != buf {
                                println!("!!!! noteq!");
                                println!("{:02X?}", obuf);
                                println!("{:02X?}", buf);
                            }
                            
                            self.server.write_all(&obuf).unwrap();*/

                            /*if let Packet::RawData(ref mut raw) = pkt {
                                if raw.cmd == 0x9E && raw.len == 0xEC {
                                    raw.cmd = 0x9D;
                                    //raw.len = 0x150;
                                    //raw.data.resize(0x150-4, 0);
                                }
                        }*/
                            if let Some(p) = self.handle_client_packet(pkt) {
                                send_packet(&mut self.server, &p, &mut proxy2server);
                            }
                        }
                    },
                    SERVER => {
                        println!("[SERVER]");
                        //while let Some((pkt, obuf)) = get_packet(&self.server, &mut server2proxy) {
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
                                //gamecube2proxy = None;
                                //proxy2gamecube = None;
                                
                                //continue

                                redirect.ip = [192, 168, 1, 10];
                                let ls = TcpListener::bind(&SocketAddr::from((Ipv4Addr::new(0,0,0,0), 0))).unwrap();
                                redirect.port = ls.local_addr().unwrap().port();
                                println!("re-redirecting! {:?}:{}", redirect.ip, redirect.port);
                                poll.register(&ls, LISTENER, Ready::readable(), PollOpt::edge()).unwrap();
                                poll.deregister(&self.server).unwrap();
                                println!("listening on: {:?}", ls);
                                listener = Some(ls);
                            }

                            /*if let Packet::AllowDenyAccess(ref mut allowdeny) = pkt {
                            allowdeny.allow = 0x11;
                        }*/


                            /*let mut buf = pkt.as_bytes();
                            if let Some(ref mut cipher) = proxy2gamecube {
                                buf = cipher.encrypt(&buf);
                            }

                            println!("@@@@@@@ same pkt? {}", obuf == buf);
                            if obuf != buf {
                                println!("!!!! noteq!");
                                println!("{:02X?}", obuf);
                                println!("{:02X?}", buf);
                            }
                            
                            self.gamecube.write_all(&obuf).unwrap();*/

                            if let Some(p) = self.handle_server_packet(pkt.clone()) {
                                send_packet(&mut self.gamecube, &p, &mut proxy2gamecube);
                            }

                            /*if let Packet::Redirect(ref redirect) = pkt {
                                println!("leaving!");
                                return;
                            }*/
                            
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
                        
                    }
                    _ => unreachable!()
                }
            }
        }
    }
}


/*
pub struct Proxy2 {
    //gamecube: Arc<Mutex<Connection>>,
    //server: Arc<Mutex<Connection>>,
    gamecube: Connection,
    server: Connection,
}




impl Proxy2 {
    pub fn new(sock: net::TcpStream) -> Proxy2 {

        //let server = TcpStream::connect(("172.245.5.200", PSOPORT)).unwrap();
        let server = TcpStream::connect(&SocketAddr::from((Ipv4Addr::new(172,245,5,200), PSOPORT))).unwrap();
        
        Proxy2 {
            //gamecube: Arc::new(Mutex::new(Connection::new(sock))),
            //server: Arc::new(Mutex::new(Connection::new(server))),
            //gamecube: Connection::new(TcpStream::from_stream(sock).unwrap()),
            gamecube: Connection::new(sock),
            server: Connection::new(server),
        }
    }




    pub fn run(&mut self) {
        let poll = Poll::new().unwrap();

        poll.register(&self.gamecube, GAMECUBE, Ready::readable(), PollOpt::edge()).unwrap();
        poll.register(&self.server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

        let mut events = Events::with_capacity(64);


        //let mut buf: [u8; 1024*16] = [0; 1024*16];
        loop {
            println!("sitting at poll....");
            poll.poll(&mut events, None).unwrap();

            println!("events! {:?}", events);
            for event in events.iter() {
                match event.token() {
                    GAMECUBE => {
                        println!("gc: {:?}", event);
                        while let Some(pkt) = self.gamecube.recv_packet() {
                            println!("gcpkt: {:?}", pkt);
                            self.server.send(pkt);
                        }
                        /*let len = self.gamecube.sock.read(&mut buf).unwrap();
                        for b in buf[0..len].iter() {
                            print!("{:02X} ", b);
                        }
                        println!("");
                        self.server.sock.write(&buf[0..len]).unwrap();*/
                    },
                    SERVER => {
                        println!("sv: {:?}", event);
                        while let Some(pkt) = self.server.recv_packet() {
                            println!("svpkt: {:?}", pkt);
                            if let Packet::Redirect(ref redirect) = pkt {
                                println!("redirecting!");
                                let new_sock = TcpStream::connect(&SocketAddr::from((redirect.ip, redirect.port))).unwrap();
                                poll.deregister(&self.server).unwrap();
                                self.server = Connection::new(new_sock);
                                poll.register(&self.server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

                                *self.gamecube.cipher_in.lock().unwrap() = None;
                                *self.gamecube.cipher_out.lock().unwrap() = None;
                                continue
                            }

                            // TODO: dont send to gc so those keys never matter...?
                            if let Packet::EncryptionKeys(ref enc_keys) = pkt {
                                println!("setting encryption keys!");
                                /* *self.server.cipher_in.lock().unwrap() = Some(Cipher::new(enc_keys.server_seed));
                                *self.server.cipher_out.lock().unwrap() = Some(Cipher::new(enc_keys.server_seed));

                                *self.gamecube.cipher_in.lock().unwrap() = Some(Cipher::new(enc_keys.client_seed));
                                 *self.gamecube.cipher_out.lock().unwrap() = Some(Cipher::new(enc_keys.client_seed));*/
                                continue
                            }


                            println!("sending to gc");
                            self.gamecube.send(pkt);
                        }

                        /*let len = self.server.sock.read(&mut buf).unwrap();
                        for b in buf[0..len].iter() {
                            print!("{:02X} ", b);
                        }
                        println!("");
                        
                        self.gamecube.sock.write(&buf[0..len]).unwrap();*/
                    }
                    _ => unreachable!()
                }
            }
            
        }




        
        
        //let gc_conn_orig = Arc::new(Mutex::new(&self.gamecube));
        //let serv_conn_orig = Arc::new(Mutex::new(&self.server));

        /*let gc_conn = self.gamecube.clone();
        let serv_conn = self.server.clone();
        let gc_thread = thread::spawn(move || {
            //let mut buf: [u8; 1024*16] = [0; 1024*16];
            loop {
                let pkt = (*gc_conn.lock().unwrap()).recv_packet();
                /*let pkt = {
                    let mut conn = gc_conn.lock().unwrap();
                    (*conn).recv_packet()
                };

                let mut out = gc_conn.lock().unwrap();*/
                
                //(*serv_conn.lock().unwrap().borrow()).send(pkt);
            }
        });

        let gc_conn = self.gamecube.clone();
        let serv_conn = self.server.clone();
        let serv_thread = thread::spawn(move || {
            
        });

        gc_thread.join().unwrap();
        serv_thread.join().unwrap();*/

        
    }
    
}
*/
