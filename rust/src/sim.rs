#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused)]
#![allow(arithmetic_overflow)]

use std::{panic, fs};

use serde::{Serialize, Deserialize};

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


pub fn default_cpu_state() -> CpuState {
    return CpuState {
            regs: vec![0; 32],
            pc:   0,
        };
}

pub fn default_sim() -> Simulator {
    let mut states = Vec::new();
    for i in 0..1 {
        states.push(default_cpu_state());
    }
    return Simulator{
        states: states,
        // fill mem with NOP
        mem: vec![0; 128],
        log: String::from("OK"),
        uart_out: String::from(""),
    };
}

pub fn step(sim: &mut Simulator) {
    let states = &mut sim.states;
    let mem    = &mut sim.mem;




    for i in 0..states.len(){
        let mut state = &mut states[i];

        // fetch
        let mut pc = state.pc;
        let ir: u32 = u32::from_be_bytes(mem[pc as usize .. (pc + 4) as usize ].try_into().unwrap());
        
        let mut err = format!("0b{:b} ", ir);
        err.push_str(&String::from("illegal instruction"));
    

        let mut imm:   u32 = 0;
        let mut func3: u8  = 0;
        let mut func7: u8  = 0;

        // 'decode'
        sim.log = String::from("decode");
        // 130
        let     opcode: u8 = ((ir & 0x7F) >> 2) as u8;
        let mut rs1i:   u8 = 0;
        let mut rs2i:   u8 = 0;
        let mut rdi:    u8 = 0;
        if (ir ^ 0b0111111) == 0b1111111 { 
            sim.log = err;
            return
        }

        match opcode {
            
            // R-type
            0b01100 => {
                func7 =  (ir >> 25) as u8;
                rs2i  = ((ir >> 20) & 0b11111) as u8;
                rs1i  = ((ir >> 15) & 0b11111) as u8;
                func3 = ((ir >> 12) & 0b00111) as u8;
                rdi   = ((ir >>  7) & 0b11111) as u8;
            }

            // I-type [JARL | LOAD]
            0b11001 | 0b00000 | 0b00100 => { 
                //rs1 = state.regs[(ir & 0x00f8000) as usize];
                imm   =   ir >> 20;
                rs1i  = ((ir >> 15) & 0b11111) as u8;
                func3 = ((ir >> 12) & 0b00111) as u8;
                rdi   = ((ir >>  7) & 0b11111) as u8;
            }, 

            // S-type
            0b10000 => { // STORES
                imm   =  (ir >>  7) & 0b11111 | (ir >>  25) & 0b1111111;
                rs2i  = ((ir >> 20) & 0b11111) as u8;
                rs1i  = ((ir >> 15) & 0b11111) as u8;
                func3 = ((ir >> 12) & 0b00111) as u8;
            },

            // B-type [BEQ]
            0b11000 => { 
                imm = (ir & 0x80000000) >> 19 |
                    (ir & 0x7e000000) >> 20 |
                    (ir & 0x00000400) >> 7  |
                    (ir & 0x00000080) << 4;
                rs2i  =((ir >> 20) & 0b11111) as u8;
                rs1i  =((ir >> 15) & 0b11111) as u8;
                func3 =((ir >> 12) & 0b00111) as u8;
            },

            // U-type [LUI AUIPC]
            0b01101 | 0b00101 => { 
                imm =   ir & 0xfffff000;
                rdi = ((ir >>  7) & 0b11111) as u8;
            }, 

            //J-type [JAL]
            0b11011 => { 
                imm = (ir & 0x80000000) >> 11 |
                    (ir & 0x7fe00000) >> 10 |
                    (ir & 0x00100000) >> 9  |
                    (ir & 0x000ff000);
                rdi =((ir >>  7) & 0b11111) as u8;
            }, 

            _ => { 
                sim.log = err;
                return
            }
        }

        let mut rs1: u64 = 0;
        let mut rs2: u64 = 0;
        let mut rd:  u64 = 0;

        // execute
        sim.log = String::from("execute");

        sim.uart_out = format!("rs1i: {:}, rs1: {:}, rs2i: {:}, rs2: {:}, rdi: {:}, imm: {:}, func3: {:}, func7: {:}", rs1i, rs1, rs2i, rs2, rdi, imm, func3, func7);

        // Instruction Set Listings p 130
        // TODO: sign extend to 64 not 32bits?
        let mut trap: u8 = 0;
        match opcode {
            
            0b01101 => { rd =       (ir & 0xfffff000) as u64;  }, // LUI
            0b00101 => { rd = pc + ((ir & 0xfffff000) as i64) as u64; }, // Add upper immediate to PC
            0b11011 => { // JAL: Jump and link
                if imm & 0x00100000 != 0 {imm |= 0xffe00000; }
                rd  = pc  + 4;
                pc += imm as i64 as u64 - 4;
                //TODO: gen instruction-address-misaligned exception if the target address is not aligned to a four-byte boundary.
            }, 
            0b11001 => { // JALR: Jump and link indirect
                if imm & 0x0000800 != 0 {imm |= 0xffffe000; }
                rd = pc + 4;
                pc = ( (rs1 + imm as i64 as u64) & !1)   - 4;
                //TODO: gen instruction-address-misaligned exception if the target address is not aligned to a four-byte boundary.
            }, 
            0b11000 => { // BEQ
                if imm & 0x100 != 0 {imm |= 0xffffe000; }
                let addr = pc + imm as i64 as u64 -4;

                // BEQ BNE BLT BGE BLTU BGEU
                match func3 {
                    0b000 => { if  rs1 == rs2 {pc = addr;} }
                    0b001 => { if  rs1 != rs2 {pc = addr;} }
                    0b100 => { if (rs1 as i64) <  (rs2 as i64) {pc = addr;} }
                    0b101 => { if (rs1 as i64) >= (rs2 as i64) {pc = addr;} }
                    0b110 => { if (rs1 as u64) <  (rs2 as u64) {pc = addr;} }
                    0b111 => { if (rs1 as u64) >= (rs2 as u64) {pc = addr;} }
                    _     => {trap = 1;}
                }
            },
            0b00000 => { // Loads
                // L-type
                //TODO: ??
                // LB LH LW LBU LHU

                if imm & 0x800 != 0 {imm |= 0xfffff000; }
                let address = rs1 + (imm as i32 as u64);

                match func3 {
                    0b000 => { rd = mem[address as usize] as i8 as u64 },
                    0b001 => { rd = i16::from_le_bytes(mem[address as usize .. (address + 2) as usize ].try_into().unwrap()) as u64; },
                    0b010 => { rd = i32::from_le_bytes(mem[address as usize .. (address + 4) as usize ].try_into().unwrap()) as u64; },
                    0b100 => { rd = mem[address as usize] as u64 },
                    0b101 => { rd = u16::from_le_bytes(mem[address as usize .. (address + 2) as usize ].try_into().unwrap()) as u64; },
                    _     => {trap = 1;},
                }
            },
            0b01000 => { // Stores
                // S-type
                // SB SH SW

                // TODO pipeline out
                if imm & 08100 != 0 {imm |= 0xfffff000; }
                let address = rs1 + (imm as i32 as u64);
                match func3 {
                    0b000 => { 
                        mem[address as usize] = rs2 as u8;
                    },
                    0b001 => { 
                        mem[ address      as usize] =  rs2       as u8;
                        mem[(address + 1) as usize] = (rs2 >> 8) as u8;
                    },
                    0b010 => {
                        mem[ address      as usize] =  rs2        as u8;
                        mem[(address + 1) as usize] = (rs2 >>  8) as u8;
                        mem[(address + 2) as usize] = (rs2 >> 16) as u8;
                        mem[(address + 3) as usize] = (rs2 >> 24) as u8;
                    },
                    _     => {trap = 1;},
                }
            },
            0b00100 | 0b01100 => {
                // ADDI SLTI SLTIU XORI ANDI SLLI SDAI
                // ADD SUB SLL SLT SLTU XOR SRL SRA OR AND
                let is_imm: bool = (opcode & 0b01000) == 0;
                if is_imm {
                    if imm & 0x00000800 != 0 {imm |= 0xfffff000; }
                    rs2 = imm as u64;
                    println!("Used immediate {rs2}, {rs2:#b}");
                }


                //TODO RV32M
                match func3 {
                    0b000 => {rd = if is_imm || (ir & 0x40000000) == 0 {rs1+rs2} else {rs1-rs2}}, // ADDI ADD SUBI
                    0b001 => {
                        if rs2 > 63 {
                            sim.log = String::from("Attempted to bit shift left too much!");
                            return
                        }
                        rd = rs1 << rs2
                    }, //SLLI SLL
                    0b010 => {rd = ((rs1 as i64) < (rs2 as i64)) as u64}, //SLTI SLT
                    0b011 => {rd = ((rs1 as u64) < (rs2 as u64)) as u64}, //SLTIU SLTU
                    0b100 => {rd = rs1 ^ rs2}, //XORI XOR
                    0b101 => {rd = if (ir & 0x40000000) != 0 { (rs1 as i64 >> rs2) as u64 } else {rs1 >> rs2 }}, //SRLI SRAI SRL SRA
                    0b110 => {rd = rs1 | rs2}, //ORI OR
                    0b111 => {rd = rs1 & rs2}, //AND I AND
                    _ => {
                        sim.log = err;
                        println!("ERROR! incorrect func3!")
                    }
                }
            },
            0b00011 => {
                // Fence
            },
            0b111001 => {
                // ECALL | EBREAK
            },
            
            _ => {
                sim.log = err;
                return
            },
        }

        // store
        if rdi != 0 {
            state.regs[rdi as usize] = rd;
        }
        state.pc = pc + 4; 
        sim.log = rd.to_string();//String::from("OK");
    }
    
}