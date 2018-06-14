// (c) 2018 Joost Yervante Damad <joost@damad.be>

use Peripherals;
use pm;

fn enable_picocache(p:&Peripherals) {
    p.HCACHE.ctrl.write(|w| w.cen().yes());
    while !p.HCACHE.sr.read().csts().bit_is_set() {}
}

pub fn enable_cache(p:&Peripherals) {
    pm::enable_clock_hsb_flashcal(p);
    pm::enable_clock_pbb_hramc1(p);
    enable_picocache(p);
}

/*

    pub fn enable_cache(&self) {
        // enable appropriate clocks

pm::enable_clock(pm::Clock::HSB(pm::HSBClock::FLASHCALWP));
// mask_clock!(HSB: hsbmask | 1 << (pm::HSBClock::FLASHCALWP as u32));

PM_REGS.unlock.set(0xAA000000 | HSB_MASK_OFFSET);
let val = PM_REGS.hsbmask.get() | (1 << (pm::HSBClock::FLASHCALWP as u32));
PM_REGS.hsbmask.set(val);

pm::enable_clock(pm::Clock::PBB(pm::PBBClock::HRAMC1));
// mask_clock!(PBB: pbbmask | 1 << (pm::PBBClock::HRAMC1 as u32)),

        // enable and wait for it to be ready
        self.enable_picocache(true);
        while !self.pico_enabled() {}
    }

*/
