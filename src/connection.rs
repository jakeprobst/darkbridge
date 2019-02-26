use mio::{Ready, Registration, Poll, PollOpt, Token, Events};
//use mio::tcp::TcpStream;
//use std::net::SocketAddr;
use std::net::{TcpStream, SocketAddr};
use mio::event::Evented;
use std::io;
use std::io::{Read, Write, Cursor};
use std::thread;
use std::sync::{Mutex, Arc};

use cipher::Cipher;
use packet::Packet;
use byteorder::{ReadBytesExt, LittleEndian};




struct Ciphers {
    input: Cipher,
    output: Cipher,
}

impl Ciphers {
    
}


/*
pub struct Connection {
    //sock: TcpStream,
    pub cipher_in: Arc<Mutex<Option<Cipher>>>,
    pub cipher_out: Arc<Mutex<Option<Cipher>>>,
    //cipher_out: Option<Cipher>,
    //recv_thread: thread::Thread,
    packet_queue: Arc<Mutex<Vec<Packet>>>,
    //sock: TcpStream,
    sock: Arc<Mutex<TcpStream>>,
    //packet_queue: Arc<Mutex<Vec<Vec<u8>>>>,
    registration: Registration,
}




impl Evented for Connection {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        self.registration.register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        self.registration.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        self.registration.deregister(poll)
    }
}

impl Connection {
    pub fn new(mut sock: TcpStream) -> Connection {
        let (registration, set_readiness) = Registration::new2();


        let conn = Connection {
            sock: Arc::new(Mutex::new(sock)),
            cipher_in: Arc::new(Mutex::new(None)),
            cipher_out: Arc::new(Mutex::new(None)),
            //sock: sock,
            registration: registration,
            packet_queue: Arc::new(Mutex::new(Vec::new())),
            //recv_thread: thread,
            //ciphers: None,
        };

        let cipher_in = conn.cipher_in.clone();
        //let cipher_out = conn.cipher_in.clone();
        let other_sock = conn.sock.clone();
        let mut sock = other_sock.lock().unwrap().try_clone().unwrap();
        let packet_queue = conn.packet_queue.clone();
        thread::spawn(move || {
            /*let poll = Poll::new().unwrap();
            
            poll.register(&sock, Token(0), Ready::readable(), PollOpt::level());
            let mut events = Events::with_capacity(64);

            loop {
                poll.poll(&mut events, None).unwrap();
                
            }*/
            
            loop {
                //let mut header: [u8; 4] = [0; 4];
                let mut header = vec![0u8; 4];
                println!("waiting on header...");
                sock.read_exact(&mut header).unwrap();
                println!("h (pre): {:?}", header);
                if let Some(ref mut cipher) = *cipher_in.lock().unwrap() {
                    header = cipher.encrypt(&header.to_vec());
                };
                println!("h (dec): {:?}", header);
                let mut cur = Cursor::new(header);
                let cmd = cur.read_u16::<LittleEndian>().unwrap();
                let len = cur.read_u16::<LittleEndian>().unwrap();

                println!("cmd {} {} 0x{:X}", cmd, len, len);
                
                let mut buf = vec![0u8; len as usize - 4];
                println!("waiting on full packet...");
                sock.read_exact(&mut buf).unwrap();
                println!("data: {:?}", buf);

                println!("parsing..");
                let pkt = Packet::parse(cmd, len, &buf);
                println!("pkt: {:?}", pkt);
                /*if let Packet::Redirect(ref redirect) = pkt {
                    println!("moving servers...");
                    sock = TcpStream::connect(&SocketAddr::from((redirect.ip, redirect.port))).unwrap();
                    let mut sock2 = other_sock.lock().unwrap();
                    *sock2 = sock.try_clone().unwrap();
                    println!("newsock: {:?} {:?}", sock, sock2);
                    set_readiness.set_readiness(Ready::readable()).unwrap();
                    continue;
                }*/

                println!("gonna grab the queue!");
                {
                    let mut queue = packet_queue.lock().unwrap();
                    println!("pushing to queue");
                    queue.push(pkt);
                }
                println!("setting readiness...");
                set_readiness.set_readiness(Ready::readable()).unwrap();

                //set_readiness.set_readiness(Ready::empty()).unwrap();
            }
        });

        conn
    }


    pub fn recv_packet(&mut self) -> Option<Packet> {
        /*let header: [u8; 4] = [0; 4];

        self.sock.read(&mut header);

        let dec_header = if let Some(ciphers) = self.ciphers {
            ciphers.gc2proxy_in.encrypt(header.to_vec())
        }
        else {
            header.to_vec()
        };

        let size = dec_header[2] << 8 + dec_header[3];*/

        //let data: [u8; size];
        let mut queue = self.packet_queue.lock().unwrap();
        queue.pop()
        
        //Some(Vec::new())
    }

    
    pub fn send(&mut self, pkt: Packet) {
        println!("sending!");
        let buf = if let Some(ref mut cipher) = *self.cipher_out.lock().unwrap() {
            cipher.encrypt(&pkt.as_bytes())
        }
        else {
            pkt.as_bytes()
        };
        let len = self.sock.lock().unwrap().write(&buf).unwrap();
        println!("sent: {:?} ({}) to {:?}", buf, len, self.sock);
    }

    /*pub fn send(&self, pkt: Packet) {
        //let buf = pkt.as_buf();

        
    }*/
}
*/
