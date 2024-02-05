use crate::oled::Oled;
use core::mem::MaybeUninit;
use stm32f1xx_hal::{adc, gpio, pac, serial, timer};

pub static mut TX: MaybeUninit<serial::Tx<pac::USART1>> = MaybeUninit::uninit();
pub static mut PANIC_LED: MaybeUninit<gpio::Pin<'C', 13, gpio::Output>> = MaybeUninit::uninit();
pub static mut SYSDELAY: MaybeUninit<timer::SysDelay> = MaybeUninit::uninit();

pub static mut LED: MaybeUninit<gpio::Pin<'A', 6, gpio::Output>> = MaybeUninit::uninit();
pub static mut HUM: MaybeUninit<gpio::Pin<'A', 5, gpio::Output>> = MaybeUninit::uninit();
pub static mut ADC1: MaybeUninit<adc::Adc<pac::ADC1>> = MaybeUninit::uninit();
pub static mut PHOT: MaybeUninit<gpio::Pin<'A', 0, gpio::Analog>> = MaybeUninit::uninit();
pub static mut CLK: MaybeUninit<stm32f1xx_hal::rcc::Clocks> = MaybeUninit::uninit();
pub static mut OLED: MaybeUninit<Oled> = MaybeUninit::uninit();
