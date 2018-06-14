// (c) 2018 Joost Yervante Damad <joost@damad.be>

use Peripherals;

pub fn enable_high_speed_flash(p:&Peripherals) {

    // enable one wait state
    p.HFLASHC.fcr.modify(|_r,w| w.fws()._1());
    // enable high-speed flash
    // command key is automatically set to 0xA5
    p.HFLASHC.fcmd.modify(|_r,w| w.cmd().hsen());

    // wait for flash to be ready
    while !p.HFLASHC.fsr.read().frdy().bit_is_set() {}
}
