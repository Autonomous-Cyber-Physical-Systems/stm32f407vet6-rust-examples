# STM32F407VET6 Rust Examples

This repository contains simple examples for STM32F407VET6 micro-controllers.

## Board

The board can be purchased from AliExpress for about 10$, [click here](https://www.aliexpress.com/item/32840837957.html)


#### Specifications:

* STM32F407VET6 ARM Cortex M4
* 168MHz, 210 DMIPS / 1.25 DMIPS / MHz
* 1.8V - 3.6V operating voltage
* 8MHz system crystal
* 32.768KHz RTC crystal
* 2.54mm pitch pins
* JTAG/SWD header
* 512KByte Flash, 192 + 4 KByte SRAM
* 3x SPI, 3x USART, 2x UART, 2x I2S, 3x I2C
* 1x FSMC, 1x SDIO, 2x CAN
* 1x USB 2.0 FS / HS controller (with dedicated DMA)
* 1x USB HS ULPI (for external USB HS PHY)
* Micro SD
* Winbond W25Q16 16Mbit SPI Flash
* RTC battery CR1220
* 1x 10/100 Ethernet MAC
* 1x 8 to 12-bit Parallel Camera interface
* 3x ADC (12-bit / 16-channel)
* 2x DAC (12-bit)
* 12x general timers, 2x advanced timers
* AMS1117-3.3V: 3.3V LDO voltage regulator, max current 800mA
* Micro USB for power and comms
* Red power LED D1
* Red user LED D2 (PA6) active low
* Red user LED D3 (PA7) active low
* 2x jumpers for bootloader selection
* Reset button, Wakeup button, 2x user buttons K0 (PE4) and K1 (PE3)
* 2x24 side pins + 2x16 bottom pins + 1x4 ISP pins
* 2x16 FMSC LCD Interface
* NRF24L01 socket
* M3 mounting holes
* Dimensions: 85.1mm x 72.45mm

#### Modifications:

* change HSE_VALUE from 8000000 to 25000000
* change PLL_M from 8 to 25

The above details have been picked up from [here](https://github.com/mcauser/BLACK_F407VE/).

Reference Manual : [click here](./Docs/)

## Examples

### LED

This code is to blink the D3 LED on the board.

To build the release binary : 
```shell
cargo build --release --example leds
```

In one terminal start openocd : 
```shell
openocd
```

In another terminal start setup gdb to run launch the application : 
```shell
arm-none-eabi-gdb -x openocd.gdb target/thumbv7em-none-eabi/release/examples/leds
```

### Button

The code prints the button status onto the console via semihosting.

To build the release binary : 
```shell
cargo build --release --example buttons
```

In one terminal start openocd : 
```shell
openocd
```

In another terminal start setup gdb to run launch the application : 
```shell
arm-none-eabi-gdb -x openocd.gdb target/thumbv7em-none-eabi/release/examples/buttons
```

### External interrupts via GPIO ports

This is a little more complicated and fun example. The board has 2 user buttons and 2 user LEDs. The code configures each button to trigger on being pressed, and the interrupt handler toggles a button.

To build the release binary : 
```shell
cargo build --release --example leds
```

In one terminal start openocd : 
```shell
openocd
```

In another terminal start setup gdb to run launch the application : 
```shell
arm-none-eabi-gdb -x openocd.gdb target/thumbv7em-none-eabi/release/examples/leds
```

> Note that the gdb configuration has been passed to gdb. But the application is not started on the board, run `continue` command in gdb to resume execution.

## References

* GPIO interrupts : [click here](https://flowdsp.io/blog/stm32f3-01-interrupts/)
* Rust embedded discovery book : [click here](https://rust-embedded.github.io/book)
* Blinking an LED : [click here](https://jonathanklimt.de/electrics/programming/rust-STM32F103-blink/)

## License

Licensed under the MIT License.