#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    i2c::{self, I2c},
    time::Hertz,
};
use embassy_time::{Duration, Timer};
use tca9534_driver_rs::{addresses, PinConfig, PinLevel, TCA9534Async as TCA9534};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C1_EV => embassy_stm32::i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C1>;
    I2C1_ER => embassy_stm32::i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c_config = i2c::Config::default();
    let i2c = I2c::new(
        p.I2C1,
        p.PA15,         // SCL
        p.PB7,          // SDA
        Irqs,           // IRQs
        p.DMA1_CH1,     // TX DMA
        p.DMA1_CH2,     // RX DMA
        Hertz(100_000), // 100kHz
        i2c_config,
    );

    let mut tca9534 = TCA9534::new(i2c, addresses::ADDR_000).await.unwrap();
    for pin in 1..4 {
        tca9534
            .set_pin_config(pin, PinConfig::Output)
            .await
            .unwrap();
    }
    for pin in 4..8 {
        tca9534.set_pin_config(pin, PinConfig::Input).await.unwrap();
    }
    tca9534.set_pin_output(0, PinLevel::High).await.unwrap();
    tca9534.set_pin_output(1, PinLevel::High).await.unwrap();
    tca9534.set_pin_output(2, PinLevel::Low).await.unwrap();
    tca9534.toggle_pin_output(0).await.unwrap();
    let pin1_level = tca9534.read_pin_input(1).await.unwrap();
    info!("Pin 1 level: {:?}", pin1_level);

    loop {
        Timer::after(Duration::from_millis(100)).await;
    }
}
