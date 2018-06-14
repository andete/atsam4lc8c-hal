// (c) 2018 Joost Yervante Damad <joost@damad.be>

extern crate atsam4lc8c as target_device;

pub use target_device::Peripherals;

pub fn init(p:&Peripherals) {

    // inspiration from atsamd21-rs, tock and ASF

    bpm::set_power_scaling(p, bpm::PS2);
    
    bscif::enable_rc32k(p);

    hcache::enable_cache(p);

    hflashc::enable_high_speed_flash(p);

}

trait RegisterOffset {
    fn offset(&self) -> u16;
}

pub mod bpm;
pub mod bscif;
pub mod hcache;
pub mod hflashc;
pub mod pm;
