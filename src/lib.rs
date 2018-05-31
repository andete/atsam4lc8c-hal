// (c) 2018 Joost Yervante Damad <joost@damad.be>

extern crate atsam4lc8c as target_device;

pub use target_device::Peripherals;

pub fn init(p:&Peripherals) {

    // inspiration from atsamd21-rs, tock and ASF

    bpm::set_power_scaling(p, 2);
    
    bscif::enable_rc32k(p);

    // flashcalw::FLASH_CONTROLLER.enable_cache()

    // flashcalw::FLASH_CONTROLLER.enable_high_speed_flash();

}

mod bpm;
mod bscif;
