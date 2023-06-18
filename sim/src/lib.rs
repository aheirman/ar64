/* WASM
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::{panic, fs};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Simulator {
    pub states:   Vec<CpuState>,
    pub mem:      Vec<u8>,
    pub log:      String,
    pub uart_out: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CpuState {
    // regs[0] MUST be zero
    // regs[1] convention return address
    // regs[5] alternate link register
    pub regs : Vec<u64>, 
    pub pc   : u64,

     
}

#[wasm_bindgen]
pub fn get_default_simulator() -> String {
    panic::set_hook(Box::new(|panic_info| {
        unsafe {
            let s = format!("{:?}", panic_info);
            alert(&s);
        }
    }));

    let mut sim = default_sim();
    /*
    for i in 0..(128/4) {
        sim.mem[i*4]   = 0b10010011;
        sim.mem[i*4+2] = 0b10000000;
    }*/
    return serde_json::to_string(&sim).unwrap();
}

#[wasm_bindgen]
pub fn step_json_simulator(sim_json: String) -> String{
    let mut sim:Simulator = serde_json::from_str(&sim_json).unwrap();
    step(&mut sim);
    return serde_json::to_string(&sim).unwrap();
}
*/
