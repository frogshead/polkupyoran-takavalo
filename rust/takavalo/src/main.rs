#![no_std]
#![no_main]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};
use embassy_time::Timer;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::rtc::{DateTime, DayOfWeek, Rtc, RtcConfig};
use defmt::info;


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

    let now = DateTime::from(2023, 6, 14, DayOfWeek::Friday, 15, 59, 10);

    let mut rtc = Rtc::new(p.RTC, RtcConfig::default());

    rtc.set_datetime(now.unwrap()).expect("datetime not set");
    let now: DateTime = rtc.now().unwrap().into();

    info!("{}:{}:{}", now.hour(), now.minute(), now.second());
    
    // Blinking 30s and then try to sleep and waiting for interrupt
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