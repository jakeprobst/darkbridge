//use std::net::TcpStream;
use mio::tcp::TcpStream;
use std::io::Read;

use cipher::Cipher;
use packet::Packet;





struct Ciphers {
    gc2proxy_in: Cipher,
    gc2proxy_out: Cipher,
    proxy2gc_in: Cipher,
    proxy2gc_out: Cipher,
}

impl Ciphers {
    
}



pub struct Connection {
    pub sock: TcpStream,
    ciphers: Option<Ciphers>,
}






impl Connection {
    pub fn new(sock: TcpStream) -> Connection {
        Connection {
            sock: sock,
            ciphers: None,
        }
    }


    pub fn recv_packet(&mut self) -> Option<Vec<u8>> {
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
        
        Some(Vec::new())
    }

    
    pub fn next_packet(&self) {
        //let buf = self.recv_packet();
        
        //let pkt = Packet::from_buf(buf);
        // catch set encryption pkt

        
        //pkt
    }


    pub fn send(&self, pkt: Vec<u8>) {
        //let buf = pkt.as_buf();

        
    }

    /*pub fn send(&self, pkt: Packet) {
        //let buf = pkt.as_buf();

        
    }*/
}
