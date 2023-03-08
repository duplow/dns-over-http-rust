use std::net::UdpSocket;
use std::io::{self};

fn createServer() {
  let socket = UdpSocket::bind("127.0.0.1:5353").expect("couldn't bind to address");
  socket.send_to(&[0; 10], "127.0.0.1:4242").expect("couldn't send data");
  // TODO: How to close?
}

fn main() {
  let mut line = String::new();

  println!("Starting DNS server!");
  
  while(true) {
    println!("Please type the next phrase: ");
    line = String::new();
    io::stdin().read_line(&mut line);
    println!("You typed {}", line);
  }

  println!("The end!"); 
}
