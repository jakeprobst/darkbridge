//use std::thread;
//use std::sync::{Arc, Mutex};
//use std::cell::RefCell;
use std::net;
use std::net::{SocketAddr, Ipv4Addr};
use mio::*;
use mio::tcp::TcpStream;
use std::io::{Read, Write};

use connection::Connection;






const PSOPORT: u16 = 9100;

const GAMECUBE: Token = Token(0);
const SERVER: Token = Token(1);





pub struct Proxy {
    //gamecube: Arc<Mutex<Connection>>,
    //server: Arc<Mutex<Connection>>,
    gamecube: Connection,
    server: Connection,
}




impl Proxy {
    pub fn new(sock: net::TcpStream) -> Proxy {

        //let server = TcpStream::connect(("172.245.5.200", PSOPORT)).unwrap();
        let server = TcpStream::connect(&SocketAddr::from((Ipv4Addr::new(172,245,5,200), PSOPORT))).unwrap();
        
        Proxy {
            //gamecube: Arc::new(Mutex::new(Connection::new(sock))),
            //server: Arc::new(Mutex::new(Connection::new(server))),
            gamecube: Connection::new(TcpStream::from_stream(sock).unwrap()),
            server: Connection::new(server),
        }
    }




    pub fn run(&mut self) {
        let poll = Poll::new().unwrap();

        poll.register(&self.gamecube.sock, GAMECUBE, Ready::readable(), PollOpt::edge()).unwrap();
        poll.register(&self.server.sock, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

        let mut events = Events::with_capacity(64);


        let mut buf: [u8; 1024*16] = [0; 1024*16];
        loop {
            poll.poll(&mut events, None).unwrap();

            
            for event in events.iter() {
                match event.token() {
                    GAMECUBE => {
                        println!("gc: {:?}", event);
                        let len = self.gamecube.sock.read(&mut buf).unwrap();
                        for b in buf[0..len].iter() {
                            print!("{:02X} ", b);
                        }
                        println!("");
                        self.server.sock.write(&buf[0..len]).unwrap();
                    },
                    SERVER => {
                        println!("sv: {:?}", event);

                        let len = self.server.sock.read(&mut buf).unwrap();
                        for b in buf[0..len].iter() {
                            print!("{:02X} ", b);
                        }
                        println!("");
                        
                        self.gamecube.sock.write(&buf[0..len]).unwrap();
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
