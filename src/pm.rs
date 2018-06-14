// (c) 2018 Joost Yervante Damad <joost@damad.be>

use {Peripherals, RegisterOffset};

const PM_UNLOCK_KEY: u8 = 0xAA;

enum Register {
    HSB,
    PBA,
    PBB,
    PBC,
    PBD,
}

use self::Register::*;

impl RegisterOffset for Register {
    fn offset(&self) -> u16 {
        match *self {
            HSB => 0x24,
            PBA => 0x28,
            PBB => 0x2C,
            PBC => 0x30,
            PBD => 0x34,
        }
    }
}

fn unlock_register(p:&Peripherals, reg:Register) {
    p.PM.unlock.write(|w| unsafe {
        w.key().bits(PM_UNLOCK_KEY);
        w.addr().bits(reg.offset())
    });
}

// TODO generalize clock functions

pub fn enable_clock_hsb_flashcal(p:&Peripherals) {
    unlock_register(p, HSB);
    p.PM.hsbmask.modify(|_r,w| w.flashcalw().set_bit());
}

pub fn enable_clock_pbb_hramc1(p:&Peripherals) {
    unlock_register(p, PBB);
    p.PM.pbbmask.modify(|_r,w| w.hramc1().set_bit());
}
