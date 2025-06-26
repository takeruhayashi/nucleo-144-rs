#![no_std]
#![no_main]

use defmt::*;
use {defmt_rtt as _, panic_probe as _};
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Pull;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("STM32 Button Example");
    
    let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down);

    loop {
        info!("Press the USER button");
        button.wait_for_rising_edge().await;
        info!("Button pressed!");
        button.wait_for_falling_edge().await;
        info!("Button released!");
    }
}