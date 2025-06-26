#![no_std]
#![no_main]

use defmt::*;
use {defmt_rtt as _, panic_probe as _};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("STM32 LED Example");

    let mut ld1 = Output::new(p.PB0, Level::High, Speed::Low);
    let mut ld2 = Output::new(p.PF4, Level::High, Speed::Low);
    let mut ld3 = Output::new(p.PG4, Level::High, Speed::Low);

    info!("Blinking LD1 on PB0 (Green LED)");
    ld1.set_high();
    info!("Blinking LD2 on PF4 (Yellow LED)");
    ld2.set_high();
    info!("Blinking LD3 on PG4 (Red LED)");
    ld3.set_high();

    Timer::after_secs(5).await;

    info!("Turning LEDs off");
    ld1.set_low();
    ld2.set_low();
    ld3.set_low();
}
