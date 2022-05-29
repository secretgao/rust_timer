use std::net::UdpSocket;
use std::{io,str};

fn main() ->io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("127.0.0.1:8082")?;

   // loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send(input.as_bytes())?;
        let mut buffer = [0u8;512];
        socket.recv_from(&mut buffer)?;
        println!("rev :{}",
        str::from_utf8(&mut buffer).expect("msg: &str"));
   // }
    Ok(())
}
