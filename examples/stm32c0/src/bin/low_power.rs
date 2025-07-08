// Notice:
// the MCU might need an extra reset to make the code actually running

#![no_std]
#![no_main]

use chrono::{NaiveDate, NaiveDateTime};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::low_power::Executor;
use embassy_stm32::rtc::{Rtc, RtcConfig};
use embassy_stm32::{Config, Peri};
use embassy_time::Timer;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    Executor::take().run(|spawner| {
        unwrap!(spawner.spawn(async_main(spawner)));
    })
}

#[embassy_executor::task]
async fn async_main(spawner: Spawner) {
    info!("Program Start");

    let p = embassy_stm32::init(Config::default());

    // give the RTC to the executor...
    let mut rtc: Rtc = Rtc::new(p.RTC, RtcConfig::default());

    let now = NaiveDate::from_ymd_opt(2020, 5, 15)
        .unwrap()
        .and_hms_opt(10, 30, 15)
        .unwrap();

    rtc.set_datetime(now.into()).expect("datetime not set");

    let now: NaiveDateTime = rtc.now().unwrap().into();
    info!("{}", now.and_utc().timestamp());

    static RTC: StaticCell<Rtc> = StaticCell::new();
    let rtc = RTC.init(rtc);
    embassy_stm32::low_power::stop_with_rtc(rtc);

    unwrap!(spawner.spawn(blinky(p.PA5.into())));
}

#[embassy_executor::task]
async fn blinky(led: Peri<'static, AnyPin>) {
    let mut led = Output::new(led, Level::Low, Speed::Low);
    loop {
        for _ in 0..10 {
            info!("pin high");
            led.set_high();
            Timer::after_millis(100).await;

            info!("pin low");
            led.set_low();
            Timer::after_millis(100).await;
        }
        // this will cause the CPU to go into STOP mode
        info!("sleep 5");
        Timer::after_millis(5000).await;
        // led.set_high();
        // info!("sleep 2");
        // Timer::after_millis(2000).await;
        // led.set_low();
    }
}
