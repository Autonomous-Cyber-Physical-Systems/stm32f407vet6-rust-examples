//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f4::stm32f407;

#[entry]
fn main() -> ! {
    // in STM32F4P7VET6 the Button K1 has been connected to GPIO pin E3
    let peripherals = stm32f407::Peripherals::take().unwrap();
    let gpio = &peripherals.GPIOE;
    // RCC : reset and control clock
    let rcc = &peripherals.RCC;

    // the following is to enable clock for GPIOE
    rcc.ahb1enr.write(|w| w.gpioeen().set_bit());
    // enable timer
    rcc.apb1enr.write(|w| w.tim2en().set_bit());

    // clear pin 3 config
    gpio.otyper.write(|w| w.ot3().clear_bit());
    // set mode for 3 as output
    gpio.moder.write(|w| w.moder3().input());
    // set mode for 3 as pull_up
    gpio.pupdr.write(|w| w.pupdr3().pull_up());

    loop {
        // prints if the button is pressed on not
        hprintln!("{}", gpio.idr.read().idr3().bit_is_clear());
    }
}
