use std::net::UdpSocket;
use std::str::{from_utf8};

pub struct Online {
    port_tx: u16,
    ip: String,
    socket: UdpSocket
}

impl Online {
    const BUFFER_SIZE: usize = 1024;
    pub fn new(ip: String, port_rx: u16, port_tx: u16) -> Result<Online, std::io::Error> {
        let build = ip.clone() + ":" + &*port_rx.to_string();
        let socket = UdpSocket::bind(build)?;

        Ok(Online {
            port_tx,
            ip,
            socket
        })
    }
    pub fn send(&mut self, data: String) -> usize {
        let build = self.ip.clone() + ":" + &*self.port_tx.to_string();
        match self.socket.send_to(data.as_bytes(), build) {
            Ok(val) => {
                val
            }
            Err(_) => {
                0
            }
        }
    }

    pub fn receive(&mut self) -> Result<String, std::io::Error> {
        let mut buf = [0; Online::BUFFER_SIZE];
        match self.socket.recv_from(&mut buf) {
            Ok(val) => {
                let addr = val.1;

                println!("own {}, foreign {}", self.socket.local_addr().unwrap().ip().to_string(), addr.ip().to_string());

                if addr.ip().to_string() != self.socket.local_addr().unwrap().ip().to_string()
                {
                    return Ok("".to_string());
                }

                match from_utf8(&buf) {
                    Ok(data) => {
                        let sequence = data.trim_matches(char::from(0)).to_string();
                        Ok(sequence.to_string())
                    }
                    Err(_) => {
                        Ok("".to_string())
                    }
                }
            }
            Err(val) => {
                Err(val)
            }
        }
    }
}