#![no_std]

const MAX_DRY: u16 = 2491;
const MAX_WET: u16 = 1865;
const MOISTURE_RANGE: u16 = MAX_DRY - MAX_WET;
const FULL_PRECENTAGE: f32 = 100.0;
const NO_PRECENTAGE: f32 = 0.0;

use esp_idf_hal::{adc::{AdcDriver, config::Config, AdcChannelDriver, ADC1, Atten11dB, }, gpio::{ADCPin }, peripheral::Peripheral};
use esp_idf_sys::EspError;

type MyResult<T> = Result<T, EspError>;

pub struct SoilMoisture<'d, T: ADCPin> {
    adc_driver: AdcDriver<'d, ADC1>,
    adc_pin: AdcChannelDriver<'d, T , Atten11dB<ADC1>>,
}

impl<'d, T:ADCPin> SoilMoisture<'d, T>
where  T: ADCPin<Adc = ADC1>
 {
    /// adc -> the adc from the peripherals
    /// pin -> gpio from peripherals pins that is connected
    pub fn new(adc: ADC1, pin: impl Peripheral<P = T> + 'd, ) -> MyResult<Self> {
        let adc = AdcDriver::new(adc, &Config::new().calibration(true))?;
        let adc_pin: AdcChannelDriver<'_, _, Atten11dB<_>> = AdcChannelDriver::new(pin)?;
        Ok(SoilMoisture { adc_driver: adc, adc_pin })
    }

    /// Get the raw read of the moisture result, analog read 
    pub fn get_raw_moisture(&mut self) -> MyResult<u16>
    {
        self.adc_driver.read(&mut self.adc_pin)
    }

    /// Get precentage read of the moisture.
    pub fn get_moisture_precentage(&mut self) -> MyResult<f32>
    {
        let raw_read = self.get_raw_moisture()?;

        if raw_read > MAX_DRY{
            return Ok(NO_PRECENTAGE);
        }
        else if raw_read < MAX_WET{
            return Ok(FULL_PRECENTAGE);
        }

        let value_diff = MAX_DRY - raw_read;
        Ok((value_diff as f32 / MOISTURE_RANGE as f32) * FULL_PRECENTAGE)
    }

}
