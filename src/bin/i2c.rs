#![no_std]
#![no_main]

use defmt::*;
use {defmt_rtt as _, panic_probe as _};
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_stm32::i2c::{I2c, Error as I2cError};
use embassy_stm32::time::Hertz;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

const BME280_ADDRESS: u8 = 0x77;
const BME280_ID_REG: u8 = 0xD0;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut i2c = I2c::new(
        p.I2C1,
        p.PB8, // SCL
        p.PB9, // SDA
        Irqs,
        p.GPDMA1_CH0,
        p.GPDMA1_CH1,
        Hertz(100_000),
        Default::default(),
    );

    let mut data = [0u8; 1];

    match i2c.blocking_write_read(BME280_ADDRESS, &[BME280_ID_REG], &mut data) {
        Ok(()) => info!("BME280 ID read successfully: 0x{:02X}", data[0]),
        Err(I2cError::Timeout) => error!("I2C timeout while reading BME280 ID"),
        Err(e) => error!("I2C error while reading BME280 ID: {:?}", e),
    }
}