#![no_std]
#![no_main]

// use cortex_m::{asm::delay, delay::{self, Delay}, prelude::_embedded_hal_blocking_delay_DelayUs};
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};
use embassy_time::Timer;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::rtc::{DateTime, DayOfWeek, Rtc, RtcConfig};
// use embassy_stm32::adc::Adc;
// use embassy_stm32::rcc::Config as ClockConfig;
use defmt::info;
use core::str::FromStr;
use chrono::{DateTime as chronoDateTime, Datelike, Timelike};

const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");



bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut config = Config::default();
    config.baudrate = 115200;
    let mut usart = Uart::new(p.USART1, p.PA10, p.PA9, Irqs,p.DMA1_CH2, p.DMA1_CH3, config).unwrap();
    usart.write(b"Hello Embassy World!\r\n").await.unwrap();
    info!("wrote Hello, starting echo");

    // Configure PA0 as input with pull-down
    let button = Input::new(p.PA0, Pull::Down);
    // Configure EXTI (External Interrupt) on PA0
    let mut exti = ExtiInput::new(button,p.EXTI0);
    let mut led = Output::new(p.PB9, Level::High, Speed::Low);
    let timestamp = i64::from_str(BUILD_TIMESTAMP).expect("Invalid timestamp");

    // Parse the timestamp into a `NaiveDateTime`
    let datetime = chronoDateTime::from_timestamp(timestamp, 0)
       .expect("Failed to parse timestamp");
    // info!("Datetime from build: {:?}", datetime.to_string());

    let now = DateTime::from(datetime.year().try_into().unwrap(),
    datetime.month().try_into().unwrap(),
    datetime.day().try_into().unwrap(),
    match datetime.weekday() {
        chrono::Weekday::Mon => DayOfWeek::Monday,
        chrono::Weekday::Tue => DayOfWeek::Tuesday,
        chrono::Weekday::Wed => DayOfWeek::Wednesday,
        chrono::Weekday::Thu => DayOfWeek::Thursday,
        chrono::Weekday::Fri => DayOfWeek::Friday,
        chrono::Weekday::Sat => DayOfWeek::Saturday,
        chrono::Weekday::Sun => DayOfWeek::Sunday,
    },
    datetime.hour().try_into().unwrap(),
    datetime.minute().try_into().unwrap(),
    datetime.second().try_into().unwrap());

    let mut rtc = Rtc::new(p.RTC, RtcConfig::default());
    

    rtc.set_datetime(now.unwrap()).expect("datetime not set");
    let now: DateTime = rtc.now().unwrap().into();

    info!("{}:{}:{}", now.hour(), now.minute(), now.second());
    
    //let mut clk_config = ClockConfig::default();
    
    // let delay = Delay::new(p.SYSCFG., ahb_frequency).delay_us(200);
    
    // let mut adc = Adc::new(p.ADC1, delay);
    // // Blinking 30s and then try to sleep and waiting for interrupt
    loop {
        info!("Toggling LED");
        for _ in 0..100{
            Timer::after_millis(300).await;
            led.toggle();
        }
        led.set_low();
        exti.wait_for_rising_edge().await;
    }
}