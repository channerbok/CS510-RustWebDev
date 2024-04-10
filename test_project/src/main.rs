use std::io::{BufRead, BufReader, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

// tcp stream is the only var that can mutate
// Wrap pointer in a buff reader, provides buffering on a stream. Grabs a chunk of memory and passes it out.
// It provides a method called readline. Reads a line from the buffer and returns it.
// unwrap the line to error handle the result
// Trim end on the read line to ignore white space at the end of the line
// Check if its empty
// Print the trimmed line
// Resets the string
fn discard_request(tcp_stream: &mut TcpStream) {
    let mut reader = BufReader::new(tcp_stream);
    let mut line = String::new();
    loop {
        reader.read_line(&mut line).unwrap();
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            return;
        }
        eprintln!("{}", trimmed);
        line.clear();
    }
}

fn main() {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(localhost, 3000);
    eprintln!("server starts: {}", socket_addr);
    let tcp_listener = TcpListener::bind(socket_addr).unwrap();

    loop {
        let (mut tcp_stream, addr) = tcp_listener.accept().unwrap();
        eprintln!("connection from {}", addr);
        discard_request(&mut tcp_stream);
        write!(tcp_stream, "HTTP/1.0 200 OK\r\n").unwrap();
        write!(tcp_stream, "Content-Type: text/html; charset=utf-8\r\n\r\n <html> <head> <meta charset=\"UTF-8\"/> <title>hello world🦀</title> </head> <body> <em>hello world🦀</em> </body> </html>").unwrap();
        tcp_stream.flush().unwrap();
    }
}
