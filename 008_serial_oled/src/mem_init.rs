use core::mem::MaybeUninit;
use stm32f1xx_hal::{adc, gpio, pac, serial, timer};

pub static mut TX: MaybeUninit<serial::Tx<pac::USART1>> = MaybeUninit::uninit();
pub static mut PANIC_LED: MaybeUninit<gpio::Pin<'C', 13, gpio::Output>> = MaybeUninit::uninit();
pub static mut SYSDELAY: MaybeUninit<timer::SysDelay> = MaybeUninit::uninit();

pub static mut LED: MaybeUninit<gpio::Pin<'A', 6, gpio::Output>> = MaybeUninit::uninit();
pub static mut HUM: MaybeUninit<gpio::Pin<'A', 5, gpio::Output>> = MaybeUninit::uninit();
pub static mut ADC1: MaybeUninit<adc::Adc<pac::ADC1>> = MaybeUninit::uninit();
pub static mut PHOT: MaybeUninit<gpio::Pin<'A', 0, gpio::Analog>> = MaybeUninit::uninit();
pub static mut OLED: MaybeUninit<crate::oled::Oled> = MaybeUninit::uninit();
pub static mut RX: MaybeUninit<serial::Rx<pac::USART1>> = MaybeUninit::uninit();
pub static mut COUNT: MaybeUninit<timer::CounterHz<pac::TIM2>> = MaybeUninit::uninit();

pub const WELCOME_STR: &[[u8; 32]] = &[
    [
        0x00, 0x80, 0x60, 0xF8, 0x07, 0x40, 0x20, 0x18, 0x0F, 0x08, 0xC8, 0x08, 0x08, 0x28, 0x18,
        0x00, 0x01, 0x00, 0x00, 0xFF, 0x00, 0x10, 0x0C, 0x03, 0x40, 0x80, 0x7F, 0x00, 0x01, 0x06,
        0x18, 0x00,
    ], /*"你",0*/
    [
        0x10, 0x10, 0xF0, 0x1F, 0x10, 0xF0, 0x00, 0x80, 0x82, 0x82, 0xE2, 0x92, 0x8A, 0x86, 0x80,
        0x00, 0x40, 0x22, 0x15, 0x08, 0x16, 0x61, 0x00, 0x00, 0x40, 0x80, 0x7F, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ], /*"好",1*/
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x58, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ], /*"，",0*/
    [
        0x04, 0x24, 0x44, 0x84, 0x64, 0x9C, 0x40, 0x30, 0x0F, 0xC8, 0x08, 0x08, 0x28, 0x18, 0x00,
        0x00, 0x10, 0x08, 0x06, 0x01, 0x82, 0x4C, 0x20, 0x18, 0x06, 0x01, 0x06, 0x18, 0x20, 0x40,
        0x80, 0x00,
    ], /*"欢",0*/
    [
        0x40, 0x40, 0x42, 0xCC, 0x00, 0x00, 0xFC, 0x04, 0x02, 0x00, 0xFC, 0x04, 0x04, 0xFC, 0x00,
        0x00, 0x00, 0x40, 0x20, 0x1F, 0x20, 0x40, 0x4F, 0x44, 0x42, 0x40, 0x7F, 0x42, 0x44, 0x43,
        0x40, 0x00,
    ], /*"迎",1*/
    [
        0x80, 0x60, 0xF8, 0x07, 0x04, 0xE4, 0x24, 0x24, 0x24, 0xFF, 0x24, 0x24, 0x24, 0xE4, 0x04,
        0x00, 0x00, 0x00, 0xFF, 0x00, 0x80, 0x81, 0x45, 0x29, 0x11, 0x2F, 0x41, 0x41, 0x81, 0x81,
        0x80, 0x00,
    ], /*"使",2*/
    [
        0x00, 0x00, 0xFE, 0x22, 0x22, 0x22, 0x22, 0xFE, 0x22, 0x22, 0x22, 0x22, 0xFE, 0x00, 0x00,
        0x00, 0x80, 0x60, 0x1F, 0x02, 0x02, 0x02, 0x02, 0x7F, 0x02, 0x02, 0x42, 0x82, 0x7F, 0x00,
        0x00, 0x00,
    ], /*"用",3*/
    [
        0x00, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    /*"！",4*/
];

pub const ANALOG_STR: &[[u8; 32]] = &[
    [
        0x10, 0x10, 0xD0, 0xFF, 0x90, 0x14, 0xE4, 0xAF, 0xA4, 0xA4, 0xA4, 0xAF, 0xE4, 0x04, 0x00,
        0x00, 0x04, 0x03, 0x00, 0xFF, 0x00, 0x89, 0x4B, 0x2A, 0x1A, 0x0E, 0x1A, 0x2A, 0x4B, 0x88,
        0x80, 0x00,
    ], /*"模",0*/
    [
        0x10, 0x10, 0x10, 0xFF, 0x90, 0x00, 0xF8, 0x00, 0x02, 0x04, 0x18, 0x00, 0xFF, 0x00, 0x00,
        0x00, 0x02, 0x42, 0x81, 0x7F, 0x00, 0x00, 0x3F, 0x10, 0x88, 0x40, 0x30, 0x0C, 0x0B, 0x30,
        0xC0, 0x00,
    ], /*"拟",1*/
    [
        0x20, 0x20, 0x20, 0xBE, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xBE, 0x20, 0x20, 0x20,
        0x00, 0x00, 0x80, 0x80, 0xAF, 0xAA, 0xAA, 0xAA, 0xFF, 0xAA, 0xAA, 0xAA, 0xAF, 0x80, 0x80,
        0x00, 0x00,
    ], /*"量",2*/
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x36, 0x36, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ], /*"：",3*/
];
