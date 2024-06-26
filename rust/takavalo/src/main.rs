#![no_std]
#![no_main]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};
use embassy_time::Timer;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
// use embassy_stm32::pac::syscfg::regs:
use embassy_stm32::pac::flash::regs::Optr;
use embassy_stm32::pac::flash::regs::Secr;

use embassy_stm32::exti::ExtiInput;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut o = Optr{
    ..Default::default()

    };
    o.set_n_boot0(false);
    o.set_n_boot1(true);
    o.set_n_boot_sel(true);

    let mut s = Secr{
        ..Default::default()
    };
    s.set_boot_lock(false);
    // Configure PA0 as input with pull-down
    let button = Input::new(p.PA0, Pull::Down);
    // Configure EXTI (External Interrupt) on PA0
    let mut exti = ExtiInput::new(button,p.EXTI0);
    let mut led = Output::new(p.PB9, Level::High, Speed::Low);


    
    // Blinking 30s and then try to sleep and waiting for interrupt
    loop {
        for _ in 0..100{
            Timer::after_millis(300).await;
            led.toggle();
        }
        led.set_low();
        exti.wait_for_rising_edge().await;
    }
}