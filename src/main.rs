#![no_std]
#![no_main]
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::delay::FreeRtos;

use log::info;

#[no_mangle]
fn main(){
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    //let peripherials = Peripherals::take().unwrap();
    //let mut soil_moister = PinDriver::input(peripherials.pins.gpio7).unwrap();


    loop {

            info!("Delay!");
            FreeRtos::delay_ms(1000);

    }

}