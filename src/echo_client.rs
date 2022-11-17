use std::net::UdpSocket;
use std::io::{self, BufRead};
use std::env;
use std::str::from_utf8;


fn main() {
    let args: Vec<String> = env::args().collect();
    // if hostname isn't entered exit 
    while args.len() < 2 {
        println!("Need to specify host to connect to. Try again");
        std::process::exit(1);
    }

    let socket = UdpSocket::bind("127.0.0.1:9800").expect("Could not connect to 127.0.0.1:9800");
    println!("Successfully binded host to 127.0.0.1:9800!");
    println!("You may start sending messages to the host below:");

    // gets host addr arg
    let hostname = &args[1];

    // connects to the host on the entered addr + port 8080
    socket.connect(hostname.to_string() + &":8080").expect("Could not connect to 127.0.0.1:8080");

    // gets msg from stdin that we would like to send to the host
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        // "k" kills input reading
        if &line == "k" { break; }

        // sends msg to host. can use send() bc we explicitly connected to the host address/port
        socket.send(line.as_bytes()).expect("Error on send");

        // receives echo of input from server and prints as confirmation
        let mut buf = [0; 2048];
        let (amt, _src) = socket.recv_from(&mut buf).expect("didn't receive the expected data :(");
        let echo = from_utf8(&buf[..amt]).unwrap();
        println!("Echoed message from the host: {}", echo);
    }
}