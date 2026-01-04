#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused)]
#![allow(arithmetic_overflow)]

use std::{panic, fs};
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

macro_rules! declare_csr_consts {
    ($vis:vis $GROUP:ident : &[$T:ty] = [$($name:ident = $value:expr; $mask:expr),* $(,)?]) => {
        mod csr_address {
            use std::collections::HashMap;


        $vis const $GROUP: &[$T] = &[$($name),*];
        pub fn get_address_to_name() -> HashMap<$T, String>{
            return HashMap::from([
                    $(($value, String::from(stringify!($name))),)*
            ]);
        }

        pub fn get_address_to_mask() -> HashMap<$T, u64>{
            return HashMap::from([
                    $(($value, $mask),)*
            ]);
        }

        $(
            $vis const $name: $T = $value;
        )*
        }
    };
}


declare_csr_consts!(pub CSR_ADDRESSES: &[u32] = [
    // Supervisor Trap Setup
    SSTATUS    = 0x100; 0xFFFFFFFF, 
    SIE        = 0x104; 0xFFFFFFFF, // interrupt-enable register
    STVEC      = 0x105; 0xFFFFFFFF, // trap handler base address
    SCONTEREN  = 0x106; 0xFFFFFFFF, // counter enable

    // Supervisor Configuration
    SENCVFG    = 0x10A; 0xFFFFFFFF, // environment configuration register

    // Supervisor Trap Handling
    SSCRATCH   = 0x140; 0xFFFFFFFF, // scratch reg for supervisor trap handlers
    SEPC       = 0x141; 0xFFFFFFFF, // Exception program counter
    SCAUSE     = 0x142; 0xFFFFFFFF, // trap cause
    STVAL      = 0x143; 0xFFFFFFFF, // bad address or instruction
    SIP        = 0x144; 0xFFFFFFFF, // interrupt pending

    // Supervisor Protection and Translation
    SATP       = 0x180; 0xFFFFFFFF, // Address Translation and Protection

    // Debut/Trace Registers
    SCONTEXT   = 0x5A8; 0xFFFFFFFF, // 
    // Hypervisor *

    // Machine Information Registers
    //MVENDORID = 0xF11; 0xFFFFFFFF, // vendor ID
    //MARCHID   = 0xF12; 0xFFFFFFFF, // arch ID
    //MIMPID    = 0xF13; 0xFFFFFFFF, // implementation ID
    MHARTID    = 0xF14; 0xFFFFFFFF,
    //MCONFIGPTR = 0xF15; 0xFFFFFFFF, // physical address of config ptr, not yet standardized!

    //Machine Trap Setup
    MSTATUS    = 0x300; 0xFFFFFFFF, // HART operating state
    MISA       = 0x301; 0xFFFFFFFF, // WARL, ISA and extensions
    MEDELEG    = 0x302; 0xFFFFFFFF, // WARL, exception delegation reg, If AND ONLY IF S-mode exists
    MIDELEG    = 0x303; 0xFFFFFFFF, // WARL, interrupt delegation reg, If AND ONLY IF S-mode exists
    MIE        = 0x304; 0xFFFFFFFF, // WARL, interrupt enable
    MTVEC      = 0x305; 0xFFFFFFFF, // WARL, trap handler base address reg
    MCOUNTEREN = 0x306; 0xFFFFFFFF, // counter enable

    // Machine Trap Handling
    MSCRATCH  = 0x340; 0xFFFFFFFF, // register for trap handler
    MEPC      = 0x341; 0xFFFFFFFF, // WARL, machine exception program counter
    MCAUSE    = 0x342; 0xFFFFFFFF, // WLRL, trap cause
    //MTVAL   = 0x343; 0xFFFFFFFF, // WARL, bad address or instruction, optional
    MIP       = 0x344; 0xFFFFFFFF, // WARL, interrupt pending
    // MTINST = 0x34A; 0xFFFFFFFF, // Hypervisor
    // MTVAL2 = 0x34B; 0xFFFFFFFF, // Hypervisor

    // Machine Configuration
    MENVCFG   = 0x30A; 0xFFFFFFFF, // environment configuration register
    // MSECCFG    = 0x747; 0xFFFFFFFF, // security configuration reg

    // Machine Memory Protection
    PMPCFG00  = 0x3A0; 0x00000000, // Physical memory protection configuration.
    PMPCFG02  = 0x3A2; 0x00000000,
    PMPCFG04  = 0x3A4; 0x00000000,
    PMPCFG06  = 0x3A6; 0x00000000,
    PMPCFG08  = 0x3A8; 0x00000000,
    PMPCFG10  = 0x3AA; 0x00000000,
    PMPCFG12  = 0x3AC; 0x00000000,
    PMPCFG14  = 0x3AE; 0x00000000,
    PMPADDR00 = 0x3B0; 0x00000000, // WARL, physical memory protection address register, only accessible to M-mode
    PMPADDR01 = 0x3B1; 0x00000000,
    PMPADDR02 = 0x3B2; 0x00000000,
    PMPADDR03 = 0x3B3; 0x00000000,
    PMPADDR04 = 0x3B4; 0x00000000,
    PMPADDR05 = 0x3B5; 0x00000000,
    PMPADDR06 = 0x3B6; 0x00000000,
    PMPADDR07 = 0x3B7; 0x00000000,
    PMPADDR08 = 0x3B8; 0x00000000,
    PMPADDR09 = 0x3B9; 0x00000000,
    PMPADDR10 = 0x3BA; 0x00000000,
    PMPADDR11 = 0x3BB; 0x00000000,
    PMPADDR12 = 0x3BC; 0x00000000,
    PMPADDR13 = 0x3BD; 0x00000000,
    PMPADDR14 = 0x3BE; 0x00000000,
    PMPADDR15 = 0x3BF; 0x00000000,
    PMPADDR16 = 0x3C0; 0x00000000,
    PMPADDR17 = 0x3C1; 0x00000000,
    PMPADDR18 = 0x3C2; 0x00000000,
    PMPADDR19 = 0x3C3; 0x00000000,
    PMPADDR20 = 0x3C4; 0x00000000,
    PMPADDR21 = 0x3C5; 0x00000000,
    PMPADDR22 = 0x3C6; 0x00000000,
    PMPADDR23 = 0x3C7; 0x00000000,
    PMPADDR24 = 0x3C8; 0x00000000,
    PMPADDR25 = 0x3C9; 0x00000000,
    PMPADDR26 = 0x3CA; 0x00000000,
    PMPADDR27 = 0x3CB; 0x00000000,
    PMPADDR28 = 0x3CC; 0x00000000,
    PMPADDR29 = 0x3CD; 0x00000000,
    PMPADDR30 = 0x3CE; 0x00000000,
    PMPADDR31 = 0x3CF; 0x00000000,
    PMPADDR32 = 0x3D0; 0x00000000,
    PMPADDR33 = 0x3D1; 0x00000000,
    PMPADDR34 = 0x3D2; 0x00000000,
    PMPADDR35 = 0x3D3; 0x00000000,
    PMPADDR36 = 0x3D4; 0x00000000,
    PMPADDR37 = 0x3D5; 0x00000000,
    PMPADDR38 = 0x3D6; 0x00000000,
    PMPADDR39 = 0x3D7; 0x00000000,
    PMPADDR40 = 0x3D8; 0x00000000,
    PMPADDR41 = 0x3D9; 0x00000000,
    PMPADDR42 = 0x3DA; 0x00000000,
    PMPADDR43 = 0x3DB; 0x00000000,
    PMPADDR44 = 0x3DC; 0x00000000,
    PMPADDR45 = 0x3DD; 0x00000000,
    PMPADDR46 = 0x3DE; 0x00000000,
    PMPADDR47 = 0x3DF; 0x00000000,
    PMPADDR48 = 0x3E0; 0x00000000,
    PMPADDR49 = 0x3E1; 0x00000000,
    PMPADDR50 = 0x3E2; 0x00000000,
    PMPADDR51 = 0x3E3; 0x00000000,
    PMPADDR52 = 0x3E4; 0x00000000,
    PMPADDR53 = 0x3E5; 0x00000000,
    PMPADDR54 = 0x3E6; 0x00000000,
    PMPADDR55 = 0x3E7; 0x00000000,
    PMPADDR56 = 0x3E8; 0x00000000,
    PMPADDR57 = 0x3E9; 0x00000000,
    PMPADDR58 = 0x3EA; 0x00000000,
    PMPADDR59 = 0x3EB; 0x00000000,
    PMPADDR60 = 0x3EC; 0x00000000,
    PMPADDR61 = 0x3ED; 0x00000000,
    PMPADDR62 = 0x3EE; 0x00000000,
    PMPADDR63 = 0x3EF; 0x00000000,
    // Machine Counter/Timers

    // Machine Counter Setup

    // Debug/Trace Registers

    // Debug Mode Registers
]);


#[derive(Serialize, Deserialize, Debug)]
pub struct Simulator {
    pub states:              Vec<CpuState>,
    pub mem:                 Vec<u8>,
    pub csr:                 HashMap<u32, u64>,
    pub csr_address_to_name: HashMap<u32, String>,
    pub log:                 String,
    pub sim_out:             String,
    pub uart_out:            Vec<u8>,
    pub state:               bool,
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
    pub priviledge_mode : u8,

     
}


pub fn default_cpu_state() -> CpuState {
    return CpuState {
            regs: vec![0; 32],
            pc:   0,
            last_pc : 0,
            last_instruction : String::from(""),
            priviledge_mode : 0b11,
        };
}

fn default_csr(address_to_name : &HashMap<u32, String>) -> HashMap<u32, u64> {
    let mut csr = HashMap::new();

    for k in address_to_name.keys() {
        csr.insert(*k, 0);
    }
    // 64 bit, BV64I, S, U
    csr.insert(csr_address::MISA, 0b10 << 62 | 1 << 8 | 1 << 18 | 1 << 20);
    return csr;
}

pub fn default_sim() -> Simulator {
    let mut states = Vec::new();
    for i in 0..1 {
        states.push(default_cpu_state());
    }
    let address_to_name = csr_address::get_address_to_name();
    return Simulator{
        states: states,
        // fill mem with NOP
        mem: vec![0; 8192],
        csr: default_csr(&address_to_name),
        csr_address_to_name: address_to_name,
        log: String::from("OK"),
        sim_out: String::from(""),
        uart_out: vec![],
        state: true,
    };
}

// WPRI -- Reserved:  Writes Preserve Values, Reads Ignore Values
// WLRL -- Write legal, Read legal
// WARL -- Write Any value, Read Legal Values 




fn handle_trap(pc : u64, state: &mut CpuState, csr : &mut HashMap<u32, u64>) -> u64{


    //let mcause_exepction_code : u64 = csr.insert(csr_address::MCAUSE ] & 0x7FFFFFFFFFFFFFFF;

    //TODO: handle MEDELEG & 
    

    // When a hart is executing in privilege mode x, interrupts are globally enabled when xIE=1 and globally disabled when xIE=0
    // nterrupts for lower-privilege modes, w<x, are always globally disabled
    // regardless of the setting of any global wIE bit for the lower-privilege mode. Interrupts for higher-
    // privilege modes, y>x, are always globally enabled regardless of the setting of the global yIE bit for the
    // higher-privilege mode
    //
    // xPIE:    holds the value of the interrupt-enable bit active prior to the trap
    // xPP:     holds the previous privilege mode up to mode x
    // MPP is 2 bits wide
    // SPP is 1 bit wide

    // When a trap is taken from privilege mode y 
    // into privilege mode x, xPIE is set to the value of xIE; xIE is set to 0; and xPP is set to y.

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

    let mtvec : u64 = csr[&csr_address::MTVEC ];
    let mtvec_mode  : u8  =(mtvec &  0b11) as u8;
    let mtvec_base  : u64 = mtvec & !0b11;
    
    let cause = match state.priviledge_mode {
        0b00 => 8,  // Environment call from U-mode
        0b01 => 9,  // Environment call from S-mode
        0b11 => 10, // Environment call from H-mode
        0b11 => 11, // Environment call from M-mode
        _ => {unreachable!();},
    };

    let is_synchronous_exception = true;

    csr.insert(csr_address::MEPC, pc & !0b11); // IALIGN is 32 bit

    /*
        *      00: U
        *      01: S
        *      10: RESERVED
        *      11: M
        */
    csr.insert(csr_address::MCAUSE, cause);
    
    let mut npc = 0;
    match mtvec_mode {
        // Direct
        0 => {npc = mtvec_base;},
        //Vectored
        1 => {
            if (is_synchronous_exception){
                npc = mtvec_base;
            } else {
                npc = mtvec_base + 4*cause;
            }
        },
        _ => {unreachable!();},
    }
    return npc;
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
fn translate_address(csr : HashMap<u32, u64>, mem : Vec<u8>, va : u64, access_type : u8) -> u64{
    // Sv39
    const PAGESIZE: u64 = 4096;
    const LEVELS:   u64 = 3;
    const PTESIZE:  u64 = 8;


    // Supervisor Address Translation and Protection register
    let satp    = csr[&csr_address::SATP    ];
    let mstatus = csr[&csr_address::MSTATUS ];

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

                let sum = (mstatus & 0x40000) != 1; // permit Supervisor User Memory access
                let mxr = (mstatus & 0x80000) != 1; // Make eXecutable Readable

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
        
    } else if address == 0x10000000 {
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
    if address < mem.len() as u64 {
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
    } else if (0x10000000..0x10000007).contains(&address)  {
        println!("WARN on: {}, WRITING UART STATUS REGISTERS IS NOT SUPPORTED, address: {:X}, value: {:X}", line!(), address, rs2);
    } else {
        println!("errored on: {}, address: 0x{:X}", line!(), address);
        //sim.state = false;
    }
}

pub fn step(sim: &mut Simulator) -> bool{
    let states = &mut sim.states;

    let should_continue = true;



    for i in 0..states.len(){ // step all HARTs
        let mut state = &mut states[i];
        
        // fetch
        let pc = state.pc;
        let mut npc: Option<u64> = None; // new pc

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
            println!("ERROR: line {}", line!());
            return false;
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
                return false;
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
                npc = Some(pc + imm as i32 as i64 as u64);
                //TODO: gen instruction-address-misaligned exception if the target address is not aligned to a four-byte boundary.
            }, 
            0b11001 => { // JALR: Jump and link indirect
                if imm & 0x0000800 != 0 {imm |= 0xffffe000; }
                rd = pc + 4;
                npc = Some( (rs1 + imm as i64 as u64) & !1);
                //TODO: gen instruction-address-misaligned exception if the target address is not aligned to a four-byte boundary.
            }, 
            0b11000 => { // BEQ
                if imm & 0x1000 != 0 {imm |= 0xffffe000; }
                let addr = pc + imm as i64 as u64;

                // BEQ BNE BLT BGE BLTU BGEU
                println!("BEQ+: r{:}:{:} op r{:}:{:}; addr: {:X}={:X}+{:X}-4", rs1i, rs1, rs2i, rs2, addr, pc, imm);
                match func3 {
                    0b000 => { if  rs1 == rs2 {npc = Some(addr);} }
                    0b001 => { if  rs1 != rs2 {npc = Some(addr);} }
                    0b100 => { if (rs1 as i64) <  (rs2 as i64) {npc = Some(addr);} }
                    0b101 => { if (rs1 as i64) >= (rs2 as i64) {npc = Some(addr);} }
                    0b110 => { if (rs1 as u64) <  (rs2 as u64) {npc = Some(addr);} }
                    0b111 => { if (rs1 as u64) >= (rs2 as u64) {npc = Some(addr);} }
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
                            return false;
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
                        return false;
                    }
                }
            },
            0b00011 => {
                // Fence

            },
            0b11100 => { // SYSTEM
                let mut csr = &mut sim.csr;

                // handle uimm versions
                let is_imm2 = func3 & 0b100 != 0;
                if is_imm2 {rs1 = rs1i as u64;};

                match func3 & 0b11 {
                    0b00 => { 
                        // p21
                        if        imm == 0b000000000000 { // ECALL
                            // ECALL | EBREAK
                            // cause a precise trap to the supporting execution environment
                            // set epc register for the recieving privilidge mode to the address of the ECALL and EBREAK instructions themselves
                            
                            npc = Some(handle_trap(pc, &mut state, &mut csr));

                            println!("ERROR! unimplemented, line: {}", line!());
                            return false;
                        } else if imm == 0b000000000001 { // EBREAK

                            println!("ERROR! unimplemented, line: {}", line!());
                            return false;
                        } 
                        
                        // TODO: pop the relevant lower-privilege interrupt enable and privilege mode stack
                        // TODO: An xRET instruction can be executed in privilege mode x or higher,
                        else if imm == 0b000100000010 { // SRET

                            // TODO: raise illegal instruction exception when TSR=1 in mstatus
                            
                            npc = Some(csr[&csr_address::SEPC ]);
                            println!("ERROR! unimplemented, line: {}", line!());
                            return false;
                        } else if imm == 0b001100000010 { // MRET 18.6.4
                            // new privilege mode based on MPP and MPV in mstatus ro mstatush
                            // mpv=0
                            // mpp=0
                            // mie=mpie
                            // mpie=1
                            // priv = new_privilege_mode;
                            npc = Some(csr[&csr_address::MEPC ]);
                            println!("ERROR! unimplemented, line: {}", line!());
                            return false;
                        } else{
                            println!("ERROR! incorrect func3!, line: {}", line!());
                            return false;
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
                            rd = sim.csr[&imm]; //TODO zero extend
                        }
                        sim.csr.insert(imm, rs1);
                        println!("INFO: executed CSRRW(I) on {}", sim.csr_address_to_name[&imm]);
                    },
                    0b10 => {
                        // CSRRS(I)
                        rd = sim.csr[&imm]; //TODO zero extend
                        if rs1i != 0 { // THIS ALSO CHECKS THE uimm AS PER THE SPEC
                            sim.csr.insert(imm, sim.csr[&imm] | rs1);
                        }
                        println!("INFO: executed CSRRS(I) on {}", sim.csr_address_to_name[&imm]);
                    },
                    0b11 => {
                        // CSRRC(I)
                        rd = sim.csr[&imm]; //TODO zero extend
                        if rs1i != 0 {
                            sim.csr.insert(imm, sim.csr[&imm] & !rs1);
                        }
                        println!("INFO: executed CSRRC(I) on {}", sim.csr_address_to_name[&imm]);
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
                        return false;
                    }
                }
            },

            _ => {
                sim.log = err;
                println!("errored on: {}", line!());
                return false;
            },
        }

        // store
        if rdi != 0 {
            state.regs[rdi as usize] = rd;
        }


        state.pc = match npc {
            Some(x) => x,
            None    => pc + 4
        };
         
        sim.log = rd.to_string();//String::from("OK");
    }
    return should_continue;
}