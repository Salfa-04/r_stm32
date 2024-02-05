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
use stm32f1xx_hal::{pac, pac::interrupt, prelude::*, timer::Event};
use system_init::system_init;

static mut LOCKER: bool = false;

#[interrupt]
fn USART1() {
    let rx = unsafe { &mut *RX.as_mut_ptr() };
    if let Ok(buff) = rx.read() {
        writeln!(unsafe { &mut *TX.as_mut_ptr() }, "接收到：{:x}", buff).unwrap();

        if buff == 0xA1 {
            unsafe {
                LOCKER = true;
            }
        } else if buff == 0xA2 {
            unsafe {
                LOCKER = false;
            }
        }

        if buff == 0xB0 {
            set_led(false);
        } else if buff == 0xB1 {
            set_led(true);
        }

        if buff == 0xC0 {
            set_hum(false);
        } else if buff == 0xC1 {
            set_hum(true);
        }
    }
}

#[interrupt]
fn TIM2() {
    let oled = unsafe { &mut *OLED.as_mut_ptr() };
    let analog: u16 =
        block!(unsafe { &mut *ADC1.as_mut_ptr() }.read(unsafe { &mut *PHOT.as_mut_ptr() }))
            .unwrap();

    if analog < 1000 {
        oled.show_char(88, 2, ' ').unwrap();
    }
    oled.show_number(64, 2, analog as u64).unwrap();

    if analog > 2700 {
        set_led(true);
        set_hum(true);
    } else if analog > 2000 {
        set_led(true);
        set_hum(false);
    } else {
        set_led(false);
        set_hum(false);
    }
}

#[entry]
fn entry() -> ! {
    system_init();

    let hum = unsafe { &mut *HUM.as_mut_ptr() };
    let led = unsafe { &mut *LED.as_mut_ptr() };
    let timer = unsafe { &mut *SYSDELAY.as_mut_ptr() };
    let oled = unsafe { &mut *OLED.as_mut_ptr() };

    oled.init().unwrap();
    oled.display_on().unwrap();
    oled.clear().unwrap();

    hum.set_low();
    led.set_high();
    oled.show_hzks(0, 0, WELCOME_STR).unwrap();

    timer.delay_ms(255u8);
    hum.set_high();
    led.set_low();
    oled.show_hzks(0, 2, ANALOG_STR).unwrap();
    oled.show_string(0, 4, "LED:").unwrap();
    oled.show_string(0, 6, "HUM:").unwrap();
    oled.show_string(64, 2, "NaN").unwrap();
    set_led(false);
    set_hum(false);

    unsafe {
        (&mut *RX.as_mut_ptr()).listen();
        (&mut *COUNT.as_mut_ptr()).listen(Event::Update);

        pac::NVIC::unmask(pac::Interrupt::USART1);
        pac::NVIC::unmask(pac::Interrupt::TIM2);
    };

    loop {}
}

fn set_led(status: bool) {
    if unsafe { LOCKER } {
        return;
    }

    if status {
        unsafe { &mut *OLED.as_mut_ptr() }
            .show_string(40, 4, "OPEN ")
            .unwrap();
        unsafe { &mut *LED.as_mut_ptr() }.set_high();
    } else {
        unsafe { &mut *OLED.as_mut_ptr() }
            .show_string(40, 4, "CLOSE")
            .unwrap();
        unsafe { &mut *LED.as_mut_ptr() }.set_low();
    }
}

fn set_hum(status: bool) {
    if unsafe { LOCKER } {
        return;
    }

    if status {
        unsafe { &mut *OLED.as_mut_ptr() }
            .show_string(40, 6, "OPEN ")
            .unwrap();
        unsafe { &mut *HUM.as_mut_ptr() }.set_low();
    } else {
        unsafe { &mut *OLED.as_mut_ptr() }
            .show_string(40, 6, "CLOSE")
            .unwrap();
        unsafe { &mut *HUM.as_mut_ptr() }.set_high();
    }
}
