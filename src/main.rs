use std::io::*;
use std::str::from_utf8;
use std::net::UdpSocket;




fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").expect("Could not connect to 127.0.0.1:8080");
    println!("Successfully binded host to 127.0.0.1:8080!");

    let mut buf = [0; 2048];

    loop {
        match socket.recv(&mut buf) {
            Ok(received) => {
                println!("received {received} bytes. The msg is: {}", from_utf8(&buf[..received]).unwrap());
                buf = buf;
            },
            Err(e) => println!("recv function failed: {e:?}"),
        }
        socket.send_to(&buf, "127.0.0.1:9800").unwrap();
    }
}