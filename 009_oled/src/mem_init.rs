use core::mem::MaybeUninit;
use stm32f1xx_hal::{gpio, pac, serial, timer};

pub static mut TX: MaybeUninit<serial::Tx<pac::USART1>> = MaybeUninit::uninit();
pub static mut PANIC_LED: MaybeUninit<gpio::Pin<'C', 13, gpio::Output>> = MaybeUninit::uninit();
pub static mut SYSDELAY: MaybeUninit<timer::SysDelay> = MaybeUninit::uninit();

pub static mut OLED: MaybeUninit<crate::oled::Oled<'B', 6, 7>> = MaybeUninit::uninit();
