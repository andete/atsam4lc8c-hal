// (c) 2018 Joost Yervante Damad <joost@damad.be>

use Peripherals;

const BSCIF_UNLOCK_KEY: u8 = 0xAA;

fn unlock_register(p:&Peripherals, register_offset: u16) {
    // TODO: calculate register offset from register?
    p.BSCIF.unlock.write(|w| unsafe {
        w.key().bits(BSCIF_UNLOCK_KEY);
        w.addr().bits(register_offset)
    });
}

// based on tock code

pub fn enable_rc32k(p:&Peripherals) {
    // configuration
    unlock_register(p, 0x24);
    p.BSCIF.rc32kcr.modify(|_,w| {
        w.en32k().set_bit(); // enable 32k output
        w.tcen().set_bit(); // enable temperature compensation
        w.en().set_bit() // enable clock
    });
    // wait for clock to be enabled
    while !p.BSCIF.rc32kcr.read().en().bit() {}

    // calibration magic bits
    unlock_register(p, 0x28);
    p.BSCIF.rc32ktune.write(|w| unsafe {
        w.coarse().bits(0x1d);
        w.fine().bits(0x15)
    });
}
