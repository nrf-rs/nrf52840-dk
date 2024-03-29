#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nb::block;

use panic_halt as _;

use nrf52840_dk_bsp::{
    hal::{
        prelude::*,
        timer::{self, Timer},
    },
    Board,
};

#[entry]
fn main() -> ! {
    let mut nrf52 = Board::take().unwrap();

    let mut timer = Timer::new(nrf52.TIMER0);

    // Alternately flash the red and blue leds
    loop {
        nrf52.leds.led_2.enable();
        delay(&mut timer, 250_000); // 250ms
        nrf52.leds.led_2.disable();
        delay(&mut timer, 1_000_000); // 1s
    }
}

fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
    T: timer::Instance,
{
    timer.start(cycles);
    let _ = block!(timer.wait());
}
