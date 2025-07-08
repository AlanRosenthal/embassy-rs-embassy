#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{gpio::{AnyPin, Level, Output, Speed}, Peri};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    unwrap!(spawner.spawn(blinky(p.PA5.into())));
}

#[embassy_executor::task]
async fn blinky(led: Peri<'static, AnyPin>) {
    let mut led = Output::new(led, Level::Low, Speed::Low);
    loop {
        for _ in 0..10 {
            info!("high");
            led.set_high();
            Timer::after_millis(300).await;

            info!("low");
            led.set_low();
            Timer::after_millis(300).await;
        }
        Timer::after_millis(10000).await;
    }
}
