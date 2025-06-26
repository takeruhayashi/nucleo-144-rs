#![no_std]
#![no_main]

use defmt::*;
use {defmt_rtt as _, panic_probe as _};
use embassy_executor::Spawner;
use embassy_net::{Ipv4Address, Ipv4Cidr, StackResources};
use embassy_net::tcp::TcpSocket;
use embassy_stm32::{Config, bind_interrupts, eth, rng, peripherals};
use embassy_stm32::rcc::{
    Hse, HseMode, Pll, PllSource, PllPreDiv, PllMul, PllDiv, AHBPrescaler, APBPrescaler, Sysclk,
    VoltageScale
};
use embassy_stm32::time::Hertz;
use embassy_stm32::rng::Rng;
use embassy_stm32::eth::{Ethernet, PacketQueue};
use embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32::peripherals::ETH;
use embassy_time::Timer;
use embedded_io_async::Write;
use heapless::Vec;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
    RNG => rng::InterruptHandler<peripherals::RNG>;
});

type Device = Ethernet<'static, ETH, GenericSMI>;

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
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
    info!("STM32 Ethernet Example");

    let mut rng = Rng::new(p.RNG, Irqs);
    let mut seed = [0; 8];
    rng.async_fill_bytes(&mut seed).await.unwrap();
    let seed = u64::from_le_bytes(seed);

    let mac_addr = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];

    static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
    let device = Ethernet::new(
        PACKETS.init(PacketQueue::<4, 4>::new()),
        p.ETH,
        Irqs,
        p.PA1,
        p.PA2,
        p.PC1,
        p.PA7,
        p.PC4,
        p.PC5,
        p.PG13,
        p.PB15,
        p.PG11,
        GenericSMI::new(0),
        mac_addr,
    );

    let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
       address:  Ipv4Cidr::new(Ipv4Address::new(192, 168, 1, 2), 24),
       gateway: None,
       dns_servers: Vec::new(),
    });

    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

    unwrap!(spawner.spawn(net_task(runner)));

    info!("Network task initialized");

    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

        let remote_endpoint = (Ipv4Address::new(192, 168, 1, 1), 9999);
        info!("Connecting to {}:{}", remote_endpoint.0, remote_endpoint.1);
        let r = socket.connect(remote_endpoint).await;
        if let Err(e) = r {
            info!("Failed to connect: {:?}", e);
            Timer::after_secs(5).await;
            continue;
        }
        info!("Connected to {}:{}", remote_endpoint.0, remote_endpoint.1);
        loop {
            let r = socket.write_all(b"Hello, world!\n").await;
            if let Err(e) = r {
                info!("Failed to write: {:?}", e);
                break;
            }
            Timer::after_secs(1).await;
        }
    }
}