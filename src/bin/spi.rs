#![no_std]
#![no_main]
#![allow(dead_code)]

use defmt::*;
use {defmt_rtt as _, panic_probe as _};
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::spi;
use embassy_stm32::spi::Spi;
use embassy_stm32::time::Hertz;
use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_stm32::rcc::{
    Hse, HseMode, Pll, PllSource, PllPreDiv, PllMul, PllDiv, AHBPrescaler, APBPrescaler, Sysclk,
    VoltageScale
};

const AD7124_COMMS_REG: u8 = 0x00;
const AD7124_COMM_REG_WEN: u8 = 0 << 7;
const AD7124_COMM_REG_WR: u8 = 0 << 6;
const AD7124_COMM_REG_RD: u8 = 1 << 6;
const AD7124_COMM_REG_RA: fn(u8) -> u8 = |x| (x & 0x3F);

const AD7124_ID_REG: u8 = 0x05;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    config.rcc.hsi = None;
    config.rcc.hsi48 = Some(Default::default());
    config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::BypassDigital,
    });
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV2,
        mul: PllMul::MUL125,
        divp: Some(PllDiv::DIV2),
        divq: Some(PllDiv::DIV2),
        divr: None,
    });
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV1;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.apb3_pre = APBPrescaler::DIV1;
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.voltage_scale = VoltageScale::Scale0;

    let p = embassy_stm32::init(config);

    let mut spi_config = spi::Config::default();
    spi_config.frequency = Hertz(1_000_000);

    let mut spi = Spi::new_blocking(
        p.SPI1,
        p.PA5,
        p.PB5,
        p.PG9,
        spi_config
    );

    let mut cs = Output::new(
        p.PD14,
        Level::High,
        Speed::VeryHigh
    );

    cs.set_low();
    let command = AD7124_COMMS_REG | AD7124_COMM_REG_WEN | AD7124_COMM_REG_RD | AD7124_COMM_REG_RA(AD7124_ID_REG);
    let mut buf = [command, 0x00];
    let result = spi.blocking_transfer_in_place(&mut buf);
    if let Err(_) = result {
        defmt::panic!("Failed to read ID register");
    }
    info!("Read ID register via SPI: 0x{:02X}", buf[1]);
}