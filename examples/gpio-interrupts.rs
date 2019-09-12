//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use stm32f4::stm32f407;
use stm32f4::stm32f407::interrupt;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

static CORTEX_PER: Mutex<RefCell<Option<cortex_m::Peripherals>>> = Mutex::new(RefCell::new(None));
static BOARD_PER: Mutex<RefCell<Option<stm32f407::Peripherals>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let cortexm_peripherals = cortex_m::Peripherals::take().unwrap();
    let board_peripherals = stm32f407::Peripherals::take().unwrap();

    // instances of configuration registers
    let rcc = &board_peripherals.RCC;
    let gpioe = &board_peripherals.GPIOE;
    let gpioa = &board_peripherals.GPIOA;
    let syscfg = &board_peripherals.SYSCFG;
    let exti = &board_peripherals.EXTI;

    // Enables the GPIOA(for the LEDs) and GPIOE(for the Buttons)
    rcc.ahb1enr
        .modify(|_, w| w.gpioeen().set_bit().gpioaen().set_bit());

    // Enables the clock
    rcc.apb2enr.write(|w| w.syscfgen().set_bit());

    // Sets the button K1(PE3) to input and pull_up
    gpioe.otyper.modify(|_, w| w.ot3().clear_bit());
    gpioe.moder.modify(|_, w| w.moder3().input());
    gpioe.pupdr.modify(|_, w| w.pupdr3().pull_up());
    // configures the external interrupt 3 to listen on PE, the number 0b0100 specifies the E GPIO bank (taken from the reference manual)
    syscfg
        .exticr1
        .modify(|_, w| unsafe { w.exti3().bits(0b0100) });

    // Sets the button K0(PE4) to input and pull_up
    gpioe.otyper.modify(|_, w| w.ot4().clear_bit());
    gpioe.moder.modify(|_, w| w.moder4().input());
    gpioe.pupdr.modify(|_, w| w.pupdr4().pull_up());
    // configures the external interrupt 4 to listen on PE, the number 0b0100 specifies the E GPIO bank (taken from the reference manual)
    syscfg
        .exticr2
        .modify(|_, w| unsafe { w.exti4().bits(0b0100) });

    // unmask the external interrupt 3 and 4
    exti.imr.modify(|_, w| w.mr3().set_bit().mr4().set_bit());

    // trigger the external interrupts 3 and 4 on rising-edge
    exti.rtsr.modify(|_, w| w.tr3().set_bit().tr4().set_bit());

    // enable the interrupts
    unsafe {
        NVIC::unmask(stm32f407::Interrupt::EXTI3);
        NVIC::unmask(stm32f407::Interrupt::EXTI4);
    }

    // clear pin 6 config
    gpioa
        .otyper
        .write(|w| w.ot6().clear_bit().ot7().clear_bit());

    // set LEDs D2(PA6), D3(PA7) as output
    gpioa.moder.write(|w| w.moder6().output().moder7().output());

    // set pull_up mode for LEDs
    gpioa
        .pupdr
        .write(|w| w.pupdr6().pull_up().pupdr7().pull_up());

    // set the globals
    cortex_m::interrupt::free(|cs| {
        CORTEX_PER.borrow(cs).replace(Some(cortexm_peripherals));
        BOARD_PER.borrow(cs).replace(Some(board_peripherals));
    });

    loop {}
}

#[interrupt]
fn EXTI3() {
    // clear the EXTI line 3 pending bit
    cortex_m::interrupt::free(|cs| {
        let refcell = BOARD_PER.borrow(cs).borrow();
        let perf = match refcell.as_ref() {
            None => return,
            Some(v) => v,
        };
        perf.EXTI.pr.write(|w| w.pr3().set_bit());
    });
    // toggle LED D2
    cortex_m::interrupt::free(|cs| {
        let refcell = BOARD_PER.borrow(cs).borrow();
        let perf = match refcell.as_ref() {
            None => return,
            Some(v) => v,
        };
        perf.GPIOA.odr.modify(|r, w| {
            let led2 = r.odr6().bit();
            if led2 {
                w.odr6().clear_bit()
            } else {
                w.odr6().set_bit()
            }
        });
    });
}

#[interrupt]
fn EXTI4() {
    // clear the EXTI line 4 pending bit
    cortex_m::interrupt::free(|cs| {
        let refcell = BOARD_PER.borrow(cs).borrow();
        let perf = match refcell.as_ref() {
            None => return,
            Some(v) => v,
        };
        perf.EXTI.pr.write(|w| w.pr4().set_bit());
    });
    // toggle LED D3
    cortex_m::interrupt::free(|cs| {
        let refcell = BOARD_PER.borrow(cs).borrow();
        let perf = match refcell.as_ref() {
            None => return,
            Some(v) => v,
        };
        perf.GPIOA.odr.modify(|r, w| {
            let led3 = r.odr7().bit();
            if led3 {
                w.odr7().clear_bit()
            } else {
                w.odr7().set_bit()
            }
        });
    });
}
