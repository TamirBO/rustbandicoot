use std::collections::HashSet;

use ps::{
    cpu::mipsr3000,
    playstation::{CYCLES_PER_FRAME, PlayStation},
};

pub struct Emulator {
    pub ps: PlayStation,
    pub running: bool,
    pub breakpoints: HashSet<u32>,
    pub step_over_target: Option<u32>,
}

impl Emulator {
    pub fn new(bios: Box<[u8]>) -> Emulator {
        let mut bps: HashSet<u32> = HashSet::new();
        //bps.insert(0xbfc019b4); // write to Ram 0x00000000 - 0x000003ff
        //bps.insert(0x00001000); // RAM instructions start
        //bps.insert(0xbfc06784);
        //bps.insert(0xbfc00420); // void LoadInitKernel ()
        //bps.insert(0xbfc06f0c); // at shit
        //bps.insert(0x000005B0); // RAM instructions start
        //bps.insert(0xbfc02ea4); // sra
        //bps.insert(0x00003140); // CHANGE IN DIFF
        //bps.insert(0xbfc01360); // mfhi
        //bps.insert(0x000030cc); // slti
        //bps.insert(0x800541c0); // dma read
        //bps.insert(0x00001010); //rfe
        //bps.insert(0x80056424);//srav
        //bps.insert(0x80052668);//multu
        bps.insert(0x80051054); // write to $zero
        Emulator {
            ps: PlayStation::new(bios),
            running: false,
            breakpoints: bps,
            step_over_target: None,
        }
    }

    pub fn run(&mut self) {
        let mut total_cycles: usize = 0;

        while total_cycles < CYCLES_PER_FRAME {
            let cycles: usize = mipsr3000::run_cycle(&mut self.ps);
            if self.breakpoints.contains(&self.ps.cpu.pc) {
                self.running = !self.running;
                break;
            }
            if let Some(target) = self.step_over_target {
                if self.ps.cpu.pc == target {
                    self.running = false;
                    self.step_over_target = None;
                }
            }
            // assume every instruction takes one cycle, I don't know if its accurate, will fix later.
            total_cycles += cycles;
        }
    }

    pub fn add_breakpoint(&mut self, address: u32) {
        self.breakpoints.insert(address);
    }

    pub fn remove_breakpoint(&mut self, address: u32) {
        self.breakpoints.remove(&address);
    }
}
