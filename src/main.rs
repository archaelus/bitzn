#![feature(plugin, start)]
#![no_std]
#![plugin(macro_zinc)]

#[macro_use] extern crate zinc;

use zinc::hal::timer::Timer;
use zinc::hal::pin::Gpio;
use zinc::hal::stm32f4::{init, pin, timer};

#[zinc_main]
pub fn main() {
    zinc::hal::mem_init::init_stack();
    zinc::hal::mem_init::init_data();

    let system = bitsys();
    system.setup(); // Setup system clock for 168MHz operation from 25MHz xtal

    let led = pin::Pin {
        port: pin::Port::PortA,
        pin: 8u8,
        function: pin::Function::GPIOOut
    };
    led.setup();
    
    let timer = timer::Timer::new(timer::TimerPeripheral::Timer2, 168u32); // 168u32 is 1us at 168MHz.

    loop {
        led.set_high();
        timer.wait_ms(500);
        led.set_low();
        timer.wait_ms(500);
    }
}

fn bitsys() -> init::SysConf {
    // https://github.com/libopencm3/libopencm3/blob/0cb1db09674cdf1413da696462f161142a98ec3b/lib/stm32/f4/rcc.c#L596
    init::SysConf {
        clock: init::ClockConf{
            source: init::SystemClockSource::SystemClockPLL( init::PLLConf{
                // https://github.com/libopencm3/libopencm3/blob/0cb1db09674cdf1413da696462f161142a98ec3b/lib/stm32/f4/rcc.c#L280-L293                
                source: init::PLLClockSource::PLLClockHSE(25_000_000),
                m: 25,
                n: 336,
                p: 2,
                q: 7
            } )
        }
    }
}
