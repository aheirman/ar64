use std::{
    io::{prelude::*, BufReader, ErrorKind},
    net::{TcpListener, TcpStream},
};
use std::{panic, fs};

use serde_json;

mod sim;
use crate::sim::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let mut sim = default_sim();
    for stream in listener.incoming() {
        //println!("GOT A REQUEST! 1");
        match stream {
            Ok(stream) => {
                //println!("GOT A REQUEST! 2");
        
                handle_connection(stream, &mut sim);
                //println!("GOT A REQUEST! 3");
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                //wait_for_fd();
                continue;
            },
            Err(e) => panic!("encountered IO error: {e}"),
        }

    }
}



fn handle_connection(mut stream: TcpStream, sim: &mut Simulator ) {
    let buf_reader = BufReader::new(&mut stream);
    let mut first = true;
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| {
            if !line.is_empty() {return true;}
            if first {first = false; return true;}
            return false;
        }
        )
        .collect();

    //println!("Request: {:#?}", http_request);
    let body = http_request.last().unwrap();
    //println!("body: {:?}", body);
    if body.starts_with('{') {
        let mut body: serde_json::Value = serde_json::from_str(body).unwrap();


        let contents;
        
        if let Some(name) = body.get_mut("action").unwrap().as_str() {
        
            match name {
                "init" => {
                    println!("INIT");
                    *sim = default_sim();
                    contents = serde_json::to_string(&sim).unwrap();
                }
                "step" => {
                    //println!("STEP");
                    step(sim);
                    contents = serde_json::to_string(&sim).unwrap();
                }
                "load image" => {
                    println!("load image");
                    let res = &fs::read("/home/iame/Desktop/programming/non-school/ar64/image");

                    match res {
                        Err(e) => {sim.log =  String::from(format!("ERRROR: failed to load image! {:?}", e))},
                        Ok(file) => {
                            //sim.mem = file.to_vec();
                            for i in 0..file.len() {
                                sim.mem[i] = file[i]
                            }
                            for i in file.len()..sim.mem.len() {
                                sim.mem[i] = 0
                            }


                        },
                    }
                    
                    contents = serde_json::to_string(&sim).unwrap();
                }
                _ => {
                    println!("ERROR unknown request");
                    contents = String::from("");
                }
            }
        } else {
            contents = String::from("");
        }
        let length = contents.len();
    
        // RETURN
        // PACKET
    
        let status_line = "HTTP/1.1 200 OK";

    
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
    
        //println!("response: {:#?}", response);
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404";
        let response = format!(
            "{status_line}"
        );
    
        //println!("ERROR response: {:#?}", response);
        stream.write_all(response.as_bytes()).unwrap();
    }
    



    
}