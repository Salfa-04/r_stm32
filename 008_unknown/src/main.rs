#![no_std]
#![no_main]

mod mem_init;
mod oled;
mod panic_handler;
mod system_init;

use core::fmt::Write;
use cortex_m_rt::entry;
use mem_init::*;
use nb::block;
use stm32f1xx_hal::prelude::*;
use system_init::system_init;

#[entry]
fn entry() -> ! {
    system_init();

    // let hum = unsafe { &mut *HUM.as_mut_ptr() };
    // let led = unsafe { &mut *LED.as_mut_ptr() };
    let tx = unsafe { &mut *TX.as_mut_ptr() };
    let timer = unsafe { &mut *SYSDELAY.as_mut_ptr() };
    let phot = unsafe { &mut *PHOT.as_mut_ptr() };
    let adc = unsafe { &mut *ADC1.as_mut_ptr() };
    let oled = unsafe { &mut *OLED.as_mut_ptr() };

    oled.init().unwrap();
    oled.display_on().unwrap();
    oled.clear().unwrap();
    oled.show_str(0, 0, "Welcome!!").unwrap();

    let mut analog: u16;

    loop {
        analog = block!(adc.read(phot)).unwrap();
        writeln!(tx, "模拟量：{}", analog).unwrap();
        timer.delay_ms(300_u32);
    }
}
