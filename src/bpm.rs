// (c) 2018 Joost Yervante Damad <joost@damad.be>

use Peripherals;

const BPM_UNLOCK_KEY: u8 = 0xAA;

fn unlock_register(p:&Peripherals, register_offset: u16) {
    // TODO: calculate register offset from register?
    p.BPM.unlock.write(|w| unsafe {
        w.key().bits(BPM_UNLOCK_KEY);
        w.addr().bits(register_offset)
    });
}

pub enum PS {
    PS0,
    PS1,
    PS2,
}

pub use self::PS::*;

impl Into<u8> for PS {
    fn into(self) -> u8 {
        match self {
            PS0 => 0,
            PS1 => 1,
            PS2 => 2,
        }
    }
}

pub fn set_power_scaling(p:&Peripherals, mode:PS) {
    unlock_register(p, 0x1c);
    
    p.BPM.pmcon.modify(|_,w| unsafe {
        // PS mode `mode`
        w.ps().bits(mode.into());
        // Power Scaling Requested
        w.pscreq().set_bit();
        // Without CPU Halt
        w.pscm().set_bit()
    });
    
    // wait for PS ok bit
    while !p.BPM.sr.read().psok().bit() {}
}
