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

pub fn set_power_scaling(p:&Peripherals, mode:u8) {
    unlock_register(p, 0x1c);
    p.BPM.pmcon.modify(|_,w| unsafe {
        w.ps().bits(mode); // PS mode
        w.pscreq().set_bit(); // Power Scaling Requested
        w.pscm().set_bit() // Without CPU Halt
    });
    while !p.BPM.sr.read().psok().bit() {}
}
