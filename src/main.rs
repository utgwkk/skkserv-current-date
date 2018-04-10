use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::string::String;

#[derive(Debug)]
enum Request {
    Disconnect,
    Convert(String),
    GetServerVersion,
    GetServerInfo,
    GetCandidates(String),
}

fn parse_request(buffer: Vec<u8>) -> Request {
    let len_query = buffer.len();
    match buffer[0] {
        48 => Request::Disconnect,
        49 => {
            let query = String::from_utf8_lossy(&buffer[1..len_query - 1]).to_string();
            Request::Convert(query)
        },
        50 => Request::GetServerVersion,
        51 => Request::GetServerInfo,
        52 => {
            let query = String::from_utf8_lossy(&buffer[1..len_query - 1]).to_string();
            Request::GetCandidates(query)
        }
        _ => unreachable!()
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![];
    let _ = stream.read_to_end(&mut buffer).ok().unwrap();
    let request = parse_request(buffer);
    println!("{:?}", request);
    match request {
        Request::Disconnect => {
            stream.shutdown(Shutdown::Both);
        },
        Request::Convert(query) => {
            stream.write(b"1/testdayon/\n");
        },
        Request::GetServerVersion => {
            stream.write(b"skk-server-date-converter.0.0\n");
        },
        Request::GetServerInfo => {
            stream.write(b"localhost:1178");
        },
        Request::GetCandidates(query) => {
            stream.write(b"1 testdayon \n");
        },
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1178").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.ok().unwrap());
    }
}
