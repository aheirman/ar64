use std::{
    io::{prelude::*, BufReader, ErrorKind},
    net::{TcpListener, TcpStream},
};
use std::{panic, fs};
use std::{thread, time};
use std::collections::HashMap;
use std::env;
use std::process::ExitCode;

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

fn cli_help() {
    println!("Usage:
    -H port  HTML server
    -T path  Self Test
");
}

enum SimMode {
    NONE,
    HTML_SERVER,
    SELF_TEST,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    let mut sim_mode = SimMode::NONE;

    match args.len() {
        3 => {
            match args[1].as_str() {
                "-H" => sim_mode=SimMode::HTML_SERVER,
                "-T" => sim_mode=SimMode::SELF_TEST,
                _ => cli_help(),
            }
        }
        1 | 2 | _ => {
            cli_help();
        },
    }
    let mut exit_code = ExitCode::from(1);
    match sim_mode {
        SimMode::HTML_SERVER => {
            match args[2].parse() {
                Ok(port_number) => server_loop(port_number),
                _ => cli_help()
            }
            },
        SimMode::SELF_TEST => {exit_code = self_test(args[2].as_str());},
        _ => {}
    }
    return exit_code;
}

fn self_test(test_binary_location: &str) -> ExitCode {
    // TODO
    ExitCode::SUCCESS
}

fn server_loop(port_number: u32) {
    let listener = TcpListener::bind(format!("127.0.0.1:{:}", port_number)).unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let mut simulators = HashMap::new();
    let mut next_index = 0;
    for stream in listener.incoming() {
        //println!("GOT A REQUEST! 1");
        match stream {
            Ok(stream) => {
                //println!("GOT A REQUEST!");
        
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
    let mut last_line_non_empty = true;

    //println!("[handle_connection]");
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| {
            //println!("line: {:?}", line);
            if !line.is_empty() {last_line_non_empty = true; return true;}
            if last_line_non_empty {last_line_non_empty = false; return true;}
            return false;
        }
        )
        .collect();

        
    //println!("[handle_connection] Request: {:#?}", http_request);

    // INSANE HACK
    let str_body = &http_request[http_request.len()-2];
    //println!("request body: {:?}", str_body);
    if str_body.starts_with('{') {
        let mut body: serde_json::Value = serde_json::from_str(str_body).unwrap();


        let mut debug = match body.get_mut("debug"){
            Some(debug_text) => {
                println!("[handle_connection] debug_text: {:#?}", debug_text);
                debug_text == "1"
            }
            _ => false
        };

        let simulator_key = match body.get_mut("device_index"){
            Some(device_index) => {
                let index: i32 = device_index.to_string().parse::<i32>().unwrap();
                index
            }
            _ => 0
        };
        if !simulators.contains_key(&simulator_key){
            println!("[handle_connection - WARN] device_index: {:#?} is not in map", simulator_key);
        }

        //println!("[handle_connection] device_index: {:#?}", simulator_key);
        let possible_sim = &mut simulators.get_mut(&simulator_key);

        if let Some(action_name) = body["action"]["name"].as_str() {
        
            match action_name {
                "init" => {
                    println!("INIT at {:}", next_index);
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
                    let location = body["action"]["location"].as_str().unwrap();
                    println!("load image at: {:?}", location);
                    let res = &fs::read(location);
                    
                    let mut sim = &mut simulators.get_mut(&simulator_key).unwrap();
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

        
        let mut sim_contents = String::from("");
        let possible_sim2 = &mut simulators.get_mut(&simulator_key);
        match possible_sim2 {
            Some(ref mut sim) => sim_contents = (&*serde_json::to_string(&sim).unwrap()).to_string(),
            _ => println!("ERROR simulator not available"),
        }
        let http_contents = format!("{{\"simulator_key\": {simulator_key},\r\n\"sim\": {sim_contents}\r\n}}");

        // RETURN
        // PACKET
    
        let status_line = "HTTP/1.1 200 OK";
        let http_length = http_contents.len();

        let response = format!("{status_line}\r\nContent-Length: {http_length}\r\n\r\n{http_contents}");

        //println!("[handle_connection] response: {:#?}", response);
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404";
        let response = format!(
            "{status_line}\r\n\r\n"
        );
    
        println!("[ERROR - handle_connection] response: {:#?}", response);
        stream.write_all(response.as_bytes()).unwrap();
    }
    



    
}