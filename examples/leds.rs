//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f4::stm32f407;

#[entry]
fn main() -> ! {
    let peripherals = stm32f407::Peripherals::take().unwrap();
    // in STM32F4P7VET6 the LED D2 has been connected to GPIO pin A6
    let gpio = &peripherals.GPIOA;
    // RCC : reset and control clock
    let rcc = &peripherals.RCC;

    // the following is to enable clock for GPIOA
    rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
    // enable timer
    rcc.apb1enr.write(|w| w.tim2en().set_bit());

    // clear pin 6 config
    gpio.otyper.write(|w| w.ot6().clear_bit());
    // set mode for 6 as output
    gpio.moder.write(|w| w.moder6().output());
    // set mode for 6 as pull_up
    gpio.pupdr.write(|w| w.pupdr6().pull_up());

    loop {
        // set
        gpio.bsrr.write(|w| w.bs6().set_bit());
        cortex_m::asm::delay(2000000);
        // reset
        gpio.bsrr.write(|w| w.br6().set_bit());
        cortex_m::asm::delay(2000000);
    }
}
