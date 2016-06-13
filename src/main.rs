use std::io::prelude::*;
use std::io::BufReader;
use std::str;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: &mut TcpStream) {
    // TODO parse headers/body

    // let mut buf_reader = BufReader::new(4096, &mut stream);
    // let mut buf = Vec::new();

    let mut buf = [0u8; 2048];

    loop {
        let bytes_got = stream.read(&mut buf).unwrap();
        println!("{:?} got", bytes_got);
        if buf[0 .. 3] == [13,10,13,10] {
            println!("here");
            break
        }
        stream.write(&buf);
    }
    // let _ = buf_reader.read_exact(&mut buf);
    // buf
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            let vec = handle_client(&mut stream);
            let resp = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nServer: Rustoleum/0.0.1\r\nContent-Length: 39\r\nX-rust-opts: none\r\n\r\n<html><body><h1>hi</h1></body></html>\r\n";
            // let st_r = str::from_utf8(&vec).unwrap();
            // println!("{}", st_r);
            // stream.write(resp.as_bytes());
        });
    }

}
