# soil_moisture1.2c6
soil moisture library for the esp32 series, was tested  currently only on esp32 c6.
If you managed to make it run on your esp32 please leave a commit/dm me so ill add it to the tested esp32.
Please note that the values are for my own sensor (dry,wet) readings, you can commit yours or change yourself, 
i will probably add support for custom later
example of usage:

```rust
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::{adc::{AdcDriver, config::Config, Atten11dB, AdcChannelDriver}, prelude::Peripherals, gpio::{Gpio5}, delay::Delay, };
use log::*;
use soil_moisture_hal::SoilMoisture;

fn main() {


    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("program starts");

    let peripherals = Peripherals::take().unwrap();

    let mut soil_moisture = SoilMoisture::new(peripherals.adc1, peripherals.pins.gpio5).unwrap();

    //let mut adc = AdcDriver::new(peripherals.adc1, &Config::new().calibration(true)).unwrap();
    /*let mut adc_pin: esp_idf_hal::adc::AdcChannelDriver<'_, Gpio5, Atten11dB<_>> =
        AdcChannelDriver::new(peripherals.pins.gpio5).unwrap();*/
    //let x = MyStruct::new(adc);



    loop
    {
        //println!("ADC value: {}", adc.read(&mut adc_pin).unwrap());
        println!("Moisture level: {}", soil_moisture.get_moisture_precentage().unwrap());
        Delay::delay_ms(1000);
    }

}
```

