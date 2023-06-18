#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused)]
#![allow(arithmetic_overflow)]

use std::{panic, fs};

use serde::{Serialize, Deserialize};

#[derive(Copy, Clone)]
pub enum CsrAddress {

    // Supervisor Trap Setup
    SSTATUS    = 0x100, 
    SIE        = 0x104, // interrupt-enable register
    STVEC      = 0x105, // trap handler base address
    SCONTEREN  = 0x106, // counter enable
    
    // Supervisor Configuration
    SENCVFG    = 0x10A, // environment configuration register

    // Supervisor Trap Handling
    SSCRATCH   = 0x140, // scratch reg for supervisor trap handlers
    SEPC       = 0x141, // Exception program counter
    SCAUSE     = 0x142, // trap cause
    STVAL      = 0x143, // bad address or instruction
    SIP        = 0x144, // interrupt pending

    // Supervisor Protection and Translation
    SATP       = 0x180, // Address Translation and Protection

    // Debut/Trace Registers
    SCONTEXT   = 0x5A8, // 
    // Hypervisor *

    // Machine Information Registers
    //MVENDORID  = 0xF11, // vendor ID
    //MARCHID    = 0xF12, // arch ID
    //MIMPID     = 0xF13, // implementation ID
    MHARTID    = 0xF14,
    // MCONFIGPTR = 0xF15, // physical address of config ptr, not yet standardized!

    //Machine Trap Setup
    MSTATUS    = 0x300, // HART operating state
    MISA       = 0x301, // WARL, ISA and extensions
    MEDELEG    = 0x302, // WARL, exception delegation reg, If AND ONLY IF S-mode exists
    MIDELEG    = 0x303, // WARL, interrupt delegation reg, If AND ONLY IF S-mode exists
    MIE        = 0x304, // WARL, interrupt enable
    MTVEC      = 0x305, // WARL, trap handler base address reg
    MCOUNTEREN = 0x306, // counter enable

    // Machine Trap Handling
    MSCRATCH   = 0x340, // register for trap handler
    MEPC       = 0x341, // WARL, machine exception program counter
    MCAUSE     = 0x342, // WLRL, trap cause
    //MTVAL    = 0x343, // WARL, bad address or instruction, optional
    MIP        = 0x344, // WARL, interrupt pending
    //MTINST   = 0x34A, // Hypervisor
    //MTVAL2   = 0x34B, // Hypervisor

    // Machine Configuration
    MENVCFG    = 0x30A, // environment configuration register
    // MSECCFG    = 0x747, // security configuration reg

    // Machine Memory Protection

    // Machine Counter/Timers

    // Machine Counter Setup

    // Debug/Trace Registers

    // Debug Mode Registers
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Simulator {
    pub states:   Vec<CpuState>,
    pub mem:      Vec<u8>,
    pub csr:      Vec<u64>,
    pub log:      String,
    pub sim_out:  String,
    pub uart_out: Vec<u8>,
    pub state:    bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CpuState {
    // x0: Zero
    // x1 - ra: Return address
    // x2 - sp: Stack pointer
    // x3 - gp: Global pointer
    // x4 - tp: Thread pointer
    // x5-7   - t0-2:  Tmp regs
    // x8-9   - s0-1:  Vallee-saved regs
    // x10-17 - a0-7:  Argument regs
    // x18-27 - s2-11: Callee-saved regs
    // x28-31 - t3-6:  Tmp regs.
    pub regs : Vec<u64>, 
    pub pc   : u64,
    pub last_pc : u64,
    pub last_instruction : String,
    /*
     * encoding:
     *      00: U
     *      01: S
     *      10: RESERVED
     *      11: M
     */
    pub current_mode : u8,

     
}


pub fn default_cpu_state() -> CpuState {
    return CpuState {
            regs: vec![0; 32],
            pc:   0,
            last_pc : 0,
            last_instruction : String::from("")
        };
}

fn default_csr() -> Vec<u64> {
    let mut csr = vec![0;4096];

    // 64 bit, BV64I, S, U
    csr[CsrAddress::MISA] = 0b10 << 62 | 1 << 8 | 1 << 18 | 1 << 20;
    return csr;
}

pub fn default_sim() -> Simulator {
    let mut states = Vec::new();
    for i in 0..1 {
        states.push(default_cpu_state());
    }
    return Simulator{
        states: states,
        // fill mem with NOP
        mem: vec![0; 8192],
        csr: ,
        log: String::from("OK"),
        sim_out: String::from(""),
        uart_out: vec![],
        state: true,
    };
}

// WPRI -- Reserved:  Writes Preserve Values, Reads Ignore Values
// WLRL -- Write legal, Read legal
// WARL -- Write Any value, Read Legal Values 



/*
 * There exists 4 kinds of traps
 *
 * 1) contained:  trap is visible to and handled by software running inside the execution environment
 * 2) Requested:  trap is synchronous exception that is an explicit call to the execution environment
 * 3) Invisible:  Trap is handled transparantly by the execution environment and execution resumes normally after trap is handled
 * 4) Fatal trap: Causes the execution environment to terminate execution
 *
 * By default all traps, at any priveledge level, are handled in machine mode. 
 * These can be redirected with MRET to the appropriate level.
 * Alternatively, with MEDELEG and MIDELEG can delegate the trap to the S-mode trap handler, 
 *  when this occurs, the delegated inturrupts are masked at the delegator level.
 * 
 * When a trap is taken into M-mode, MEPC is written with the virtual address of the instruction that was interrupted or that encountered the exception.
 * When a trap is taken into M-mode, MCAUSE is written with a code indicating the event that caused the trap.
 *
 */
fn catch_trap(pc : & mut u64, csr : & Vec<u64>) {
    let mtvec : u64 = csr[CsrAddress::MTVEC as usize];
    let mtvec_mode  : u8  =(mtvec &  0b11) as u8;
    let mtvec_base  : u64 = mtvec & !0b11;

    let mcause_exepction_code : u64 = csr[CsrAddress::MCAUSE as usize] & 0x7FFFFFFFFFFFFFFF;

    //TODO: handle MEDELEG & MIDELEG

    match mtvec_mode {
        // Direct
        0 => {*pc = mtvec_base;},
        //Vectored
        1 => {*pc = mtvec_base + 4*cause;},
        _ => {unreachable!();},
    }

}

/*
    Virtual addresses
        4.3.2 p 82
        PTE = page table entries
        PPN = physical page number
        VPN = virtual page number
        PMA = 
        PMP = 
        va  = virtual address

        WPRI = reserved Writes Preserve values, Reads Ignore values
        WLRL = Write Legal Read Legal
        WARL = Write Any values, Read Legal values

    CSRs listed in table 2.2 etc
*/

// access_type
// 0 read
// 1 write
// 2 execute
fn translate_address(csr : Vec<u64>, mem : Vec<u8>, va : u64, access_type : u8) -> u64{
    // Sv39
    const PAGESIZE: u64 = 4096;
    const LEVELS:   u64 = 3;
    const PTESIZE:  u64 = 8;


    // Supervisor Address Translation and Protection register
    let satp    = csr[CsrAddress::SATP    as usize];
    let mstatus = csr[CsrAddress::MSTATUS as usize];

    // physical page number
    let satp_ppn   = satp & 0xfffffffffff; // bottom 44 bits

    // address space identifier
    //satp_asid: 44 to 59

    
    //let satp_mode  = satp & 0xF000000000000000 >> 60;// 60 to 63

    
    //the effecitve privilege mode must be S or U
    let satp_active = false;
    if satp_active {
        let mut i = LEVELS - 1;
        let mut a = satp_ppn * PAGESIZE;

        while i > 0 {
            let va_vpn = [(va >> 12) & 0x1ff, (va >> 21) & 0x1ff, (va >> 30) & 0x1ff]; 
            

            let va_vpn_i = va_vpn[i as usize];
            
            let address = (a + va_vpn_i*PTESIZE)  as usize;
            let pte = u64::from_le_bytes(mem[address as usize .. (address + 8) as usize ].try_into().unwrap());
            //TODO generate access fault if needed

            let pte_v = pte & 0b00000001;
            let pte_r = pte & 0b00000010;
            let pte_w = pte & 0b00000100;
            let pte_x = pte & 0b00001000;
            let pte_u = pte & 0b00010000;
            let pte_g = pte & 0b00100000;
            let pte_a = pte & 0b01000000;
            let pte_d = pte & 0b10000000;

            let pte_unsupported = pte & 0xFFC0000000000000;
            if (pte_v != 0) || (pte_r != 0 && pte_w != 1)  ||  (pte_unsupported != 0) {
                println!("errored on: {}", line!());
                return 0;
            }

            // PTE is valid :D
            if (pte_r != 0) || (pte_x != 0) {
                // step 5

                let SUM = (mstatus & 0x40000) != 1; // permit Supervisor User Memory access
                let MXR = (mstatus & 0x80000) != 1; // Make eXecutable Readable

                // TODO
                // MXR = 1 --> loads from either readable or executable will succeed
                // MXR = 0 --> only loads from readable will succeed
                // MXR has no effect if page based virtual memory is NOT in effect.

                // SUM = 0 --> S-mode memory accesses to pages that are accessible by U-mode (U=1) will fault.
                // SUM = 1 --> permitted

                //step 6

                let pte_ppn_2 = pte & 0x3FFFFFF0000000;
                let pte_ppn_1 = pte & 0x0000000FF80000;
                let pte_ppn_0 = pte & 0x0000000007FC00;
                let pte_ppn = match i {
                    0 => 0,
                    1 => pte_ppn_0,
                    2 => pte_ppn_0 | pte_ppn_1,
                    3 => pte_ppn_0 | pte_ppn_1 | pte_ppn_2,
                    _ => unreachable!(),
                };


                if i>0 && pte_ppn != 0 {
                    // page fault
                    println!("errored on: {}, misaligned superpage", line!());
                    return 0;
                }

                // step 7
                if pte_a == 0 || ((access_type == 1) && (pte_d == 0)) {
                    println!("errored on: {}, page fault", line!());
                    return 0;
                }

                // step 8
                let mut pa : u64 = va & 0xfff; // set page offset
                pa |= match i {
                    0 => pte_ppn_0 | pte_ppn_1 | pte_ppn_2,
                    1 => va_vpn[0] | pte_ppn_1 | pte_ppn_2,
                    2 => va_vpn[0] | va_vpn[1] | pte_ppn_2,
                    _ => unreachable!(),
                };
                return  pa;
            }
            i = i-1;
            let pte_ppn = pte & 0x3FFFFFFFFFFC00;
            a = pte_ppn*PAGESIZE;
        }
        // page fault
        println!("errored on: {}", line!());
        return 0;
    } else {
        return va;
    }
}

fn load(mem: &mut Vec<u8>, func3: u8, address: u64) -> u64{
    let mut rd = 0;

    if address < 0x1000 {
        
        match func3 {
            0b000 => { rd = mem[address as usize] as i8 as u64 },
            0b001 => { rd = i16::from_le_bytes(mem[address as usize .. (address + 2) as usize ].try_into().unwrap()) as u64; },
            0b010 => { rd = i32::from_le_bytes(mem[address as usize .. (address + 4) as usize ].try_into().unwrap()) as u64; },
            0b100 => { rd = mem[address as usize] as u64 },
            0b101 => { rd = u16::from_le_bytes(mem[address as usize .. (address + 2) as usize ].try_into().unwrap()) as u64; },
            0b011 => { rd = i64::from_le_bytes(mem[address as usize .. (address + 8) as usize ].try_into().unwrap()) as u64; }
            _     => {
                //trap = 1;
                println!("errored on: {}, func3: {:#b}", line!(), func3);
            },
        }
        
    } 
    else if address == 0x10000000 {
        // uart
    } else if address <= 0x1000007 {
        println!("WARN on: {}, READING UART STATUS REGISTERS IS NOT SUPPORTED, address: {:X}", line!(), address);
    } else {
        println!("errored on: {}, address: {:X}", line!(), address);
        //sim.state = false;
    }
    return rd;
}

fn store(mem: &mut Vec<u8>, func3: u8, address: u64, rs2: u64, uart_out: &mut Vec<u8>){
    if address < 0x1000 {
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
            0b011 => {
                mem[ address      as usize] =  rs2        as u8;
                mem[(address + 1) as usize] = (rs2 >>  8) as u8;
                mem[(address + 2) as usize] = (rs2 >> 16) as u8;
                mem[(address + 3) as usize] = (rs2 >> 24) as u8;
                mem[(address + 4) as usize] = (rs2 >> 32) as u8;
                mem[(address + 5) as usize] = (rs2 >> 40) as u8;
                mem[(address + 6) as usize] = (rs2 >> 48) as u8;
                mem[(address + 7) as usize] = (rs2 >> 56) as u8;
            }
            _     => {
                //trap = 1;
                println!("errored on: {}", line!());
            },
        }
    } else if address == 0x10000000 {
        // UART
        uart_out.push(rs2 as u8)
    } else if address <= 0x1000007 {
        println!("WARN on: {}, WRITING UART STATUS REGISTERS IS NOT SUPPORTED, address: {:X}, value: {:X}", line!(), address, rs2);
    } else {
        println!("errored on: {}, address: {:X}", line!(), address);
        //sim.state = false;
    }
}

fn decode(ir: u32) -> ()) {}

pub fn step(sim: &mut Simulator) {
    let states = &mut sim.states;




    for i in 0..states.len(){ // step all HARTs
        let mut state = &mut states[i];
        
        // fetch
        let mut pc = state.pc;
        state.last_pc = pc;
        let ir: u32 = u32::from_le_bytes(sim.mem[pc as usize .. (pc + 4) as usize ].try_into().unwrap());
        
        // clear sim out
        sim.sim_out = String::from("");
        state.last_instruction = format!("{:X}", ir);

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
        
        //64 bit instructions
        //if (ir ^ 0b0111111) & 0b1111111 != 0 { 
        // 32 bit
        if ((ir & 0b11) != 3) || ((ir & 0b11100) == 0b11100)  {
            sim.log = err;
            println!("errored on: {}", line!());
            return
        }

        // decode 
        match opcode {
            
            // R-type
            0b01100 => {
                func7 =  (ir >> 25) as u8;
                rs2i  = ((ir >> 20) & 0b11111) as u8;
                rs1i  = ((ir >> 15) & 0b11111) as u8;
                func3 = ((ir >> 12) & 0b00111) as u8;
                rdi   = ((ir >>  7) & 0b11111) as u8;
            }

            // I-type [JARL | LOAD | ADD+ | ADDIW | SYSTEM
            0b11001 | 0b00000 | 0b00100 | 0b00110 | 0b11100 => { 
                //rs1 = state.regs[(ir & 0x00f8000) as usize];
                imm   =   ir >> 20;
                rs1i  = ((ir >> 15) & 0b11111) as u8;
                func3 = ((ir >> 12) & 0b00111) as u8;
                rdi   = ((ir >>  7) & 0b11111) as u8;
            }, 

            // S-type
            0b01000 => { // STORES
                imm   =  (ir >>  7) & 0b11111 | ((ir >>  25) & 0b1111111) << 5;
                rs2i  = ((ir >> 20) & 0b11111) as u8;
                rs1i  = ((ir >> 15) & 0b11111) as u8;
                func3 = ((ir >> 12) & 0b00111) as u8;
            },

            // B-type [BEQ]
            0b11000 => { 
                imm = (ir & 0x80000000) >> 19 |
                    (ir & 0x7e000000) >> 20 |
                    (ir & 0x00000f00) >> 7  |
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
                    (ir & 0x7fe00000) >> 20 |
                    (ir & 0x00100000) >> 9  |
                    (ir & 0x000ff000);
                rdi =((ir >>  7) & 0b11111) as u8;
            }, 

            _ => { 
                sim.log = err;
                println!("errored on: {}", line!());
                return
            }
        }

        let mut rs1: u64 = state.regs[rs1i as usize];
        let mut rs2: u64 = state.regs[rs2i as usize];
        let mut rd:  u64 = 0;

        // execute
        sim.log = String::from("execute");

        sim.sim_out.push_str(&*format!("\r\nrs1i: {:}, rs1: {:}, rs2i: {:}, rs2: {:}, rdi: {:}, imm: {:}, func3: {:}, func7: {:}", rs1i, rs1, rs2i, rs2, rdi, imm, func3, func7));

        // Instruction Set Listings p 130
        // TODO: sign extend to 64 not 32bits?
        let mut trap: u8 = 0;
        match opcode {
            
            0b01101 => { rd =       (ir & 0xfffff000) as u64;  }, // LUI
            0b00101 => { rd = pc + ((ir & 0xfffff000) as i64) as u64; }, // Add upper immediate to PC
            0b11011 => { // JAL: Jump and link
                if imm & 0x00100000 != 0 {imm |= 0xffe00000; }
                rd  = pc  + 4;
                println!("JAL: imm: {}", imm as i64);
                pc += imm as i32 as i64 as u64 - 4;
                //TODO: gen instruction-address-misaligned exception if the target address is not aligned to a four-byte boundary.
            }, 
            0b11001 => { // JALR: Jump and link indirect
                if imm & 0x0000800 != 0 {imm |= 0xffffe000; }
                rd = pc + 4;
                pc = ( (rs1 + imm as i64 as u64) & !1)   - 4;
                //TODO: gen instruction-address-misaligned exception if the target address is not aligned to a four-byte boundary.
            }, 
            0b11000 => { // BEQ
                if imm & 0x1000 != 0 {imm |= 0xffffe000; }
                let addr = pc + imm as i64 as u64 -4;

                // BEQ BNE BLT BGE BLTU BGEU
                println!("BEQ+: r{:}:{:} op r{:}:{:}; addr: {:X}={:X}+{}-4", rs1i, rs1, rs2i, rs2, addr, pc, imm);
                match func3 {
                    0b000 => { if  rs1 == rs2 {pc = addr;} }
                    0b001 => { if  rs1 != rs2 {pc = addr;} }
                    0b100 => { if (rs1 as i64) <  (rs2 as i64) {pc = addr;} }
                    0b101 => { if (rs1 as i64) >= (rs2 as i64) {pc = addr;} }
                    0b110 => { if (rs1 as u64) <  (rs2 as u64) {pc = addr;} }
                    0b111 => { if (rs1 as u64) >= (rs2 as u64) {pc = addr;} }
                    _     => {trap = 1;println!("errored on: {}", line!());}
                }
            },
            0b00000 => { // Loads
                // L-type
                //TODO: ??
                // LB LH LW LBU LHU

                if imm & 0x800 != 0 {imm |= 0xfffff000; }
                let address = rs1 + (imm as i32 as u64);
                rd = load(&mut sim.mem, func3, address);
            },
            0b01000 => { // Stores
                // S-type

                // TODO pipeline out
                if imm & 0x800 != 0 {imm |= 0xfffff000; }
                let address = rs1 + (imm as i32 as i64 as u64);
                println!("Stored rs{:}: {:} at (imm + r{:}): {:}+0x{:X}=0x{:X} with func3: {}", rs2i, rs2, rs1i, imm as i32 as i64, rs1, address, func3);
                store(&mut sim.mem, func3, address, rs2, &mut sim.uart_out);
            },
            0b00100 | 0b01100 => {
                // ADDI SLTI SLTIU XORI ANDI SLLI SDAI
                // ADD SUB SLL SLT SLTU XOR SRL SRA OR AND

                // Checks in the opcode
                let is_imm: bool = (opcode & 0b01000) == 0;
                if is_imm {
                    if imm & 0x00000800 != 0 {imm |= 0xfffff000; }
                    rs2 = imm as i32 as i64 as u64;
                    println!("Used immediate {:}, {:#b}", rs2 as i64, rs2 as i64);
                }


                //TODO RV32M
                match func3 {
                    0b000 => {rd = if is_imm || (ir & 0x40000000) == 0 {rs1+rs2} else {rs1-rs2}}, // ADDI ADD SUBI
                    0b001 => {
                        if rs2 > 63 {
                            sim.log = String::from("Attempted to bit shift left too much!");
                            println!("errored on: {}", line!());
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
                        println!("ERROR! incorrect func3!, line: {}", line!());
                        return
                    }
                }
            },
            0b00011 => {
                // Fence

            },
            0b11100 => { // SYSTEM
                let csr = &mut sim.csr;
                let is_imm2 = func3 & 0b100 != 0;
                if is_imm2 {rs1 = rs1i as u64;};

                match func3 & 0b11 {
                    0b00 => { 
                        if        imm == 0b000000000000 { // ECALL
                            // ECALL | EBREAK
                            // cause a precise trap to the supporting execution environment
                            // set epc register for the recieving privilidge mode to the address of the ECALL and EBREAK instructions themselves
                            
                            // set xPIE: holds the value of the interrupt-enable bit active prior to the trap
                            // xPP holds the previous priviledge mode
                            // MPP is 2 bits wide
                            // SPP is 1 bit wide


                            println!("ERROR! unimplemented, line: {}", line!());
                        } else if imm == 0b000000000001 { // EBREAK

                            println!("ERROR! unimplemented, line: {}", line!());
                        } 
                        
                        // TODO: pop the relevant lower-privilege interrupt enable and privilege mode stack
                        // TODO: An xRET instruction can be executed in privilege mode x or higher,
                        else if imm == 0b000100000010 { // SRET

                            // TODO: raise illegal instruction exception when TSR=1 in mstatus
                            pc = csr[CsrAddress::SEPC];
                            println!("ERROR! unimplemented, line: {}", line!());
                        } else if imm == 0b001100000010 { // MRET
                            pc = csr[CsrAddress::MEPC];
                            println!("ERROR! unimplemented, line: {}", line!());
                        } else{
                            println!("ERROR! incorrect func3!, line: {}", line!());
                        }
                        // set cause
                        // set pc
                        
                    },
                    //---------
                    //- Zicsr -
                    //---------
                    0b01 => {
                        // CSRRW(I)
                        if rdi != 0 {
                            rd = sim.csr[imm as usize]; //TODO zero extend
                        }
                        sim.csr[imm as usize] = rs1;
                    },
                    0b010 => {
                        // CSRRS(I)
                        rd = sim.csr[imm as usize]; //TODO zero extend
                        if rs1i != 0 { // THIS ALSO CHECKS THE uimm AS PER THE SPEC
                            sim.csr[imm as usize] |= rs1;
                        }
                        
                    },
                    0b011 => {
                        // CSRRC(I)
                        rd = sim.csr[imm as usize]; //TODO zero extend
                        if rs1i != 0 {
                            sim.csr[imm as usize] = sim.csr[imm as usize] & !rs1;
                        }
                    },
                    _ => unreachable!(),
                }

            },


            //---------
            //- RV64i -
            //---------
            0b00110 => {

                // SEXT
                if imm & 0x00000800 != 0 {imm |= 0xfffff000; }
                rs2 = imm as i32 as i64 as u64;
                println!("Used immediate {:}, {:#b}", rs2 as i64, rs2 as i64);

                match func3 {
                    0b000 => {
                        // ADDIW
                        rd = (rs1+rs2) & 0xffffffff;
                        if rd & 0x80000000 != 0 {rd |= 0xffffffff00000000; }
                    },
                    0b001 => {
                        //SLLIW
                        println!("errored on: {}", line!());
                    },
                    0b101 => {
                        //SRLIW & SRAIW
                        println!("errored on: {}", line!());
                        rd = if (ir & 0x40000000) != 0 { (rs1 as i64 >> rs2) as u64 } else {rs1 >> rs2 }
                    },
                    _ => {
                        sim.log = err;
                        println!("ERROR! incorrect func3!, line: {}", line!());
                        return
                    }
                }
            },

            _ => {
                sim.log = err;
                println!("errored on: {}", line!());
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