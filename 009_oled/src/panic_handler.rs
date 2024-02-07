use crate::mem_init::{PANIC_LED, SYSDELAY, TX};
use core::fmt::Write;
use stm32f1xx_hal::prelude::*;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let led = unsafe { &mut *PANIC_LED.as_mut_ptr() };
    let tx = unsafe { &mut *TX.as_mut_ptr() };
    let timer = unsafe { &mut *SYSDELAY.as_mut_ptr() };

    loop {
        if let Err(_) = writeln!(tx, "{}", info) {};

        led.set_low();
        timer.delay_ms(150_u32);
        led.set_high();
        timer.delay_ms(900_u32);
    }
}
