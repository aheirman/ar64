use std::{
    io::{prelude::*, BufReader, ErrorKind},
    net::{TcpListener, TcpStream},
};
use std::{panic, fs};
use std::{thread, time};
use std::collections::HashMap;

use serde_json;

mod sim;
use crate::sim::*;

/*
 * There are a couple types of packets, these are disambiguited with "action".
 *
 *  actions:
 *      i) "init":   Creates a new device returns a device key, initializes the device to a default state
 *      i) "load":   Loads from a default file
 *      i) "step":   Steps 1 clock cycle
 *
 * The requests are in a Json string
 *
 */
fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let mut simulators = HashMap::new();
    let mut next_index = 0;
    for stream in listener.incoming() {
        //println!("GOT A REQUEST! 1");
        match stream {
            Ok(stream) => {
                println!("GOT A REQUEST!");
        
                handle_connection(&mut next_index, stream, &mut simulators);
                //println!("GOT A REQUEST! finished");
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                //wait_for_fd();
                thread::sleep(time::Duration::from_millis(10));
                continue;
            },
            Err(e) => panic!("encountered IO error: {e}"),
        }

    }
}



fn handle_connection(next_index: &mut i32, mut stream: TcpStream, simulators: &mut HashMap<i32, Simulator>) {
    let buf_reader = BufReader::new(&mut stream);
    let mut first = true;

    println!("[handle_connection]");
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

        
    println!("[handle_connection] Request: {:#?}", http_request);
    let str_body = http_request.last().unwrap();
    //println!("body: {:?}", body);
    if str_body.starts_with('{') {
        let mut body: serde_json::Value = serde_json::from_str(str_body).unwrap();


        let contents = String::from("");
        let mut debug = match body.get_mut("debug"){
            Some(debug_text) => {
                println!("[handle_connection] debug_text: {:#?}", debug_text);
                debug_text == "1"
            }
            _ => false
        };

        let simulator_key = match body.get_mut("device_index"){
            Some(device_index) => {
                println!("[handle_connection] device_index: {:#?}", device_index);
                let index: i32 = device_index.to_string().parse::<i32>().unwrap();
                if !simulators.contains_key(&index){
                    println!("[handle_connection - ERROR] device_index: {:#?} is not in map", device_index);
                }
                index
            }
            _ => 0
        };
        let possible_sim = &mut simulators.get_mut(&simulator_key);

        if let Some(action) = body.get_mut("action").unwrap().as_str() {
        
            match action {
                "init" => {
                    println!("INIT");
                    simulators.insert(*next_index, default_sim());
                }
                "step" => {
                    //println!("STEP");
                    match possible_sim {
                        Some(ref mut sim) => step(sim),
                        _ => println!("ERROR"),
                    }
                    
                }
                "load image" => {
                    println!("load image");
                    //let res = &fs::read("/home/iame/Desktop/programming/non-school/ar64/image");
                    /*
                    let mut sim = &mut simulators.get_mut(&simulator_key);
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
                    }*/
                    
                    
                }
                _ => {
                    println!("ERROR unknown request");
                    debug = false;
                }
            }
        } else {
            println!("ERROR unknown status");
            debug = false;
        }


        if debug {
            //let mut sim = &mut simulators.get_mut(&simulator_key);
            //contents += &serde_json::to_string(&sim).unwrap();
        }
        let length = contents.len();
    
        // RETURN
        // PACKET
    
        let status_line = "HTTP/1.1 200 OK";


        let response = format!(
            "{status_line}\r\nsimulator_key: {simulator_key}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );


        println!("[handle_connection] response: {:#?}", response);
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404";
        let response = format!(
            "{status_line}"
        );
    
        println!("[ERROR - handle_connection] response: {:#?}", response);
        stream.write_all(response.as_bytes()).unwrap();
    }
    



    
}