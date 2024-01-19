#![no_std]
#![no_main]

use core::fmt::Write;
use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::prelude::*;

use stm32f1xx_hal::{pac, serial, timer::Timer};

static mut TX: MaybeUninit<serial::Tx<pac::USART1>> = MaybeUninit::uninit();
static mut RX: MaybeUninit<serial::Rx<pac::USART1>> = MaybeUninit::uninit();
static mut BOARD_LED: MaybeUninit<stm32f1xx_hal::gpio::Pin<'A', 6, stm32f1xx_hal::gpio::Output>> =
    MaybeUninit::uninit();

#[entry]
fn entry() -> ! {
    system_init();

    let timer = unsafe { &mut *DELAY.as_mut_ptr() };
    let led = unsafe { &mut *BOARD_LED.as_mut_ptr() };

    let tx = unsafe { &mut *TX.as_mut_ptr() };
    let rx = unsafe { &mut *RX.as_mut_ptr() };

    const END: u8 = 's' as u8; // 终止符号
    const BUFFER_LEN: usize = 64; // 缓冲区大小

    let mut buffer = [0u8; BUFFER_LEN];
    let mut index = 0usize;

    loop {
        match block!(rx.read()) {
            Ok(x) => {
                buffer[index] = x;
                index += 1;
            }
            Err(_x) => {
                // write!(tx, "\nPanic: {:?}\n", _x).unwrap();
                continue;
            }
        }

        while let Ok(x) = block!(rx.read()) {
            buffer[index] = x;
            index += 1;

            if x == END || index >= BUFFER_LEN {
                break;
            }
        }

        tx.write_str("\n邢彦瑶🥰").unwrap();
        tx.bwrite_all(&buffer[..index]).unwrap();

        led.set_high();
        timer.delay_ms(300_u32);
        led.set_low();
        index = 0;
    }
}

fn system_init() {
    let dp = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();
    let mut afio = dp.AFIO.constrain();
    let mut flash = dp.FLASH.constrain();

    let rcc = dp.RCC.constrain();
    let clock = rcc.cfgr.freeze(&mut flash.acr);

    let serial_config = serial::Config::default()
        .baudrate(19200_u32.bps())
        .parity_none()
        .wordlength_8bits()
        .stopbits(serial::StopBits::STOP1);

    let serial_pins = (
        gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),
        gpioa.pa10,
    );

    let (tx, rx) = serial::Serial::new(
        dp.USART1,
        serial_pins,
        &mut afio.mapr,
        serial_config,
        &clock,
    )
    .split();

    unsafe {
        // System Init && Panic

        *TX.as_mut_ptr() = tx;
        *DELAY.as_mut_ptr() = Timer::syst(cp.SYST, &clock).delay();
        *PANIC_LED.as_mut_ptr() = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, stm32f1xx_hal::gpio::PinState::High);
    };

    {
        // User Code Here ⬇️

        unsafe {
            *RX.as_mut_ptr() = rx;
            *BOARD_LED.as_mut_ptr() = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
        };
    }
}

static mut DELAY: MaybeUninit<stm32f1xx_hal::timer::SysDelay> = MaybeUninit::uninit();
static mut PANIC_LED: MaybeUninit<stm32f1xx_hal::gpio::Pin<'C', 13, stm32f1xx_hal::gpio::Output>> =
    MaybeUninit::uninit();

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let delay = unsafe { &mut *DELAY.as_mut_ptr() };
    let led = unsafe { &mut *PANIC_LED.as_mut_ptr() };
    let tx = unsafe { &mut *TX.as_mut_ptr() };

    loop {
        if let Err(_) = writeln!(tx, "{}", info) {};

        led.toggle();
        delay.delay_ms(150_u32);
        led.toggle();
        delay.delay_ms(900_u32);
    }
}
