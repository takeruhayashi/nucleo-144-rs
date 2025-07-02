#![no_std]
#![no_main]

use defmt::*;
use {defmt_rtt as _, panic_probe as _};
use embassy_executor::Spawner;
use embassy_stm32::{adc::AdcChannel, Config};
use embassy_stm32::rcc::{mux, HSIPrescaler, Pll, PllSource, PllDiv, PllPreDiv, PllMul, Sysclk, AHBPrescaler, APBPrescaler, VoltageScale};
use embassy_stm32::adc::{Adc, SampleTime, Resolution};
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    config.rcc.hsi = Some(HSIPrescaler::DIV1);
    config.rcc.csi = true;
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSI,
        prediv: PllPreDiv::DIV4,
        mul: PllMul::MUL25,
        divp: Some(PllDiv::DIV2),
        divq: Some(PllDiv::DIV4),
        divr: None,
    });
    config.rcc.pll2 = Some(Pll {
        source: PllSource::HSI,
        prediv: PllPreDiv::DIV4,
        mul: PllMul::MUL25,
        divp: None,
        divq: None,
        divr: Some(PllDiv::DIV4),
    });
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV2;
    config.rcc.apb2_pre = APBPrescaler::DIV2;
    config.rcc.apb3_pre = APBPrescaler::DIV2;
    config.rcc.voltage_scale = VoltageScale::Scale1;
    config.rcc.mux.adcdacsel = mux::Adcdacsel::PLL2_R;

    let mut p = embassy_stm32::init(config);
    info!("STM32 ADC Example");

    let mut adc = Adc::new(p.ADC1);
    adc.set_sample_time(SampleTime::CYCLES24_5);
    adc.set_resolution(Resolution::BITS12);
    // let resolution = Resolution::BITS12.to_bits();

    let mut vrefint_channel = adc.enable_vrefint();
    let mut temp_channel = adc.enable_temperature();

    loop {
        let vrefint_raw = adc.blocking_read(&mut vrefint_channel);
        let vref = 1.216 * 4095.0 / vrefint_raw as f32; 
        info!("vrefint raw: {}, vref: {}", vrefint_raw, vref);

        let temp_raw = adc.blocking_read(&mut temp_channel);
        let vsense = (temp_raw as f32 * vref) / 4095.0;
        let temp = (((0.62 - vsense) * 1000.0) / 2.0) + 30.0;
        info!("temp raw: {}, vsense: {}, temp: {}", temp_raw, vsense, temp);

        // let measured = adc.blocking_read(&mut p.PA0);
        // info!("measured: {}", measured);

        Timer::after_secs(5).await;
    }
}
