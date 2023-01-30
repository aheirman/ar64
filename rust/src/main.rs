use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod sim;
use crate::sim::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}



fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    let status_line = "HTTP/1.1 200 OK";

    let contents = "{log: \"\", uart_out: \"\", mem: [], pc: -1, states: []}";
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    println!("response: {:#?}", response);
    stream.write_all(response.as_bytes()).unwrap();



    
}