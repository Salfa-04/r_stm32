#![no_std]
#![no_main]

use core::fmt::Write;
use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::prelude::*;

use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::{gpio, pac, serial, timer};

#[entry]
fn entry() -> ! {
    system_init();

    loop {}
}

#[interrupt]
fn USART1() {
    const BUFFER_LEN: usize = 64;
    const END_CHAR: char = '\n';

    let tx = unsafe { &mut *TX.as_mut_ptr() };
    let rx = unsafe { &mut *RX.as_mut_ptr() };
    unsafe { &mut *BOARD_LED.as_mut_ptr() }.toggle();

    let mut buffer = [0u8; BUFFER_LEN];
    let mut index = 0;

    while let Ok(x) = block!(rx.read()) {
        buffer[index] = x;
        index += 1;
        if index >= BUFFER_LEN || x == END_CHAR as u8 {
            break;
        }
    }

    tx.bwrite_all(&buffer[..]).unwrap();
}

static mut BOARD_LED: MaybeUninit<gpio::Pin<'A', 6, gpio::Output>> = MaybeUninit::uninit();
static mut RX: MaybeUninit<serial::Rx<pac::USART1>> = MaybeUninit::uninit();

fn system_init() {
    let dp = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();
    let mut afio = dp.AFIO.constrain();
    let mut flash = dp.FLASH.constrain();

    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .sysclk(8_u32.MHz())
        .pclk1(8_u32.MHz())
        .freeze(&mut flash.acr);

    let serial_config = serial::Config::default()
        .baudrate(19200_u32.bps())
        .parity_none()
        .wordlength_8bits()
        .stopbits(serial::StopBits::STOP1);
    let serial_pins = (
        // Serial USART1 PIN: tx-a9 rx-a10
        gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),
        gpioa.pa10,
    );
    let serial = serial::Serial::new(
        dp.USART1,
        serial_pins,
        &mut afio.mapr,
        serial_config,
        &clocks,
    )
    .split();

    unsafe {
        // System Init && Panic
        *TX.as_mut_ptr() = serial.0;
        *SYSDELAY.as_mut_ptr() = timer::Timer::syst(cp.SYST, &clocks).delay();
        *PANIC_LED.as_mut_ptr() = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    };

    {
        // User Code Here ⬇️
        let mut rx = serial.1;
        rx.listen();

        unsafe {
            *RX.as_mut_ptr() = rx;
            *BOARD_LED.as_mut_ptr() = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);

            pac::NVIC::unmask(pac::Interrupt::USART1);
        };
    }
}

static mut TX: MaybeUninit<serial::Tx<pac::USART1>> = MaybeUninit::uninit();
static mut PANIC_LED: MaybeUninit<gpio::Pin<'C', 13, gpio::Output>> = MaybeUninit::uninit();
static mut SYSDELAY: MaybeUninit<timer::SysDelay> = MaybeUninit::uninit();

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
