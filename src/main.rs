extern crate mio;

mod proxy;
mod cipher;
mod packet;
mod connection;


use proxy::Proxy;

use std::net::TcpListener;
use std::net::{SocketAddr, Ipv4Addr};
//use mio::*;
//use mio::tcp::TcpListener;

const PSOPORT: u16 = 9100;

fn main() {
    //let listener = TcpListener::bind(("localhost", PSOPORT)).unwrap();
    //let listener = TcpListener::bind("127.0.0.1:9100").unwrap();
    //let listener = TcpListener::bind("0.0.0.0:9100").unwrap();
    let listener = TcpListener::bind(&SocketAddr::from((Ipv4Addr::new(0,0,0,0), PSOPORT))).unwrap();
    println!("l: {:?}", listener);
    
    /*let poll = Poll::new().unwrap();

    poll.register(&listener, Token(0), Ready::readable(), PollOpt::edge()).unwrap();

    let mut events = Events::with_capacity(64);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            println!("event: {:?}", event);
            match listener.accept() {
                Ok((socket, addr)) => {
                    println!("recv! {:?} {:?}", socket, addr);
                    let mut proxy = Proxy::new(socket);
                    proxy.run();
                }
                Err(e) => {
                    println!("err: {:?}", e);
                }
            }
        }
    }*/


    
    loop {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("recv! {:?} {:?}", socket, addr);
                let mut proxy = Proxy::new(socket);
                proxy.run();
            }
            Err(e) => {
                println!("err: {:?}", e);
            }
        }
    }


    
    /*let buf = (0..32).collect();
    println!("{:?}", buf);

    let mut cipher = Cipher::new(23);
    
    let buf = cipher.encrypt(buf);
    for k in buf.iter() {
        print!("{:02X} ", k);
    }
    println!("");
    
    let buf = cipher.encrypt(buf);
    for k in buf.iter() {
        print!("{:02X} ", k);
    }
    println!("");
    
    let buf = cipher.encrypt(buf);
    for k in buf.iter() {
        print!("{:02X} ", k);
    }
    println!("");

    let buf = cipher.encrypt(buf);
    for k in buf.iter() {
        print!("{:02X} ", k);
    }
    println!("");*/
}
