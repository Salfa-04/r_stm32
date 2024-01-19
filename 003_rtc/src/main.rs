#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::prelude::*;

use stm32f1xx_hal::rtc::Rtc;
use stm32f1xx_hal::{pac, timer::Timer};

static mut DELAY: MaybeUninit<stm32f1xx_hal::timer::SysDelay> = MaybeUninit::uninit();
static mut PANIC_LED: MaybeUninit<stm32f1xx_hal::gpio::Pin<'C', 13, stm32f1xx_hal::gpio::Output>> =
    MaybeUninit::uninit();

static mut BOARD_LED: MaybeUninit<stm32f1xx_hal::gpio::Pin<'A', 6, stm32f1xx_hal::gpio::Output>> =
    MaybeUninit::uninit();

static mut RTC: MaybeUninit<Rtc> = MaybeUninit::uninit();

#[entry]
fn entry() -> ! {
    system_init();

    let rtc = unsafe { &mut *RTC.as_mut_ptr() };
    let led = unsafe { &mut *BOARD_LED.as_mut_ptr() };
    rtc.select_frequency(64_u32.Hz());

    loop {
        rtc.set_time(1_u32);
        rtc.set_alarm(32_u32);
        block!(rtc.wait_alarm()).unwrap();
        led.toggle();
    }
}

fn system_init() {
    let mut dp = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    let delay = unsafe { &mut *DELAY.as_mut_ptr() };
    let pc13 = unsafe { &mut *PANIC_LED.as_mut_ptr() };

    let mut gpioc = dp.GPIOC.split();
    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clock = rcc.cfgr.freeze(&mut flash.acr);

    *pc13 = gpioc
        .pc13
        .into_push_pull_output_with_state(&mut gpioc.crh, stm32f1xx_hal::gpio::PinState::High);
    *delay = Timer::syst(cp.SYST, &clock).delay();

    {
        // Code Here ⬇️

        let rtc = unsafe { &mut *RTC.as_mut_ptr() };
        let mut bkp = rcc.bkp.constrain(dp.BKP, &mut dp.PWR);
        *rtc = Rtc::new(dp.RTC, &mut bkp);

        let pa6 = unsafe { &mut *BOARD_LED.as_mut_ptr() };
        let mut gpioa = dp.GPIOA.split();
        *pa6 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    let delay = unsafe { &mut *DELAY.as_mut_ptr() };
    let led = unsafe { &mut *PANIC_LED.as_mut_ptr() };

    loop {
        led.toggle();
        delay.delay_ms(150_u32);
        led.toggle();
        delay.delay_ms(900_u32);
    }
}
