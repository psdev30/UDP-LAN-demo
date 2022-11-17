use std::net::UdpSocket;
use std::io::{self, BufRead};
use std::env;
use std::str::from_utf8;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Need to specify host to connect to");
        std::process::exit(1);
    }

    let socket = UdpSocket::bind("127.0.0.1:9800").expect("Could not connect to 127.0.0.1:9800");
    println!("Successfully binded host to 127.0.0.1:9800!");

    let hostname = &args[1];

    socket.connect(hostname.to_string() + &":8080").expect("Could not connect to 127.0.0.1:8080");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        //println!("Line read from stdin '{}'", line);
        // "k" kills input reading
        if &line == "k" {
            break;
        }

        socket.send(line.as_bytes()).expect("Error on send");

        let mut buf = [0; 2048];
        let (amt, _src) = socket.recv_from(&mut buf).expect("didn't receive the expected data :(");
        let echo = from_utf8(&buf[..amt]).unwrap();
        println!("Echoed message from the host: {}", echo);
    }
}