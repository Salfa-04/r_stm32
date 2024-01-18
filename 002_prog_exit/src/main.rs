#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::{
    gpio::{Edge, ExtiPin, Input, Output, PullUp, PushPull},
    pac,
    prelude::*,
    timer,
};

static mut LED: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA6<Output<PushPull>>> =
    MaybeUninit::uninit();
static mut INT_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA5<Input<PullUp>>> =
    MaybeUninit::uninit();
static mut TIMER: MaybeUninit<timer::SysDelay> = MaybeUninit::uninit();

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    system_init(dp, cp);

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI9_5);
    }

    loop {}
}

#[interrupt]
fn EXTI9_5() {
    let led = unsafe { &mut *LED.as_mut_ptr() };
    let int_pin = unsafe { &mut *INT_PIN.as_mut_ptr() };
    let timer = unsafe { &mut *TIMER.as_mut_ptr() };

    if int_pin.check_interrupt() {
        led.toggle();

        timer.delay_ms(80_u32);
        int_pin.clear_interrupt_pending_bit();
    }
}

fn system_init(mut dp: pac::Peripherals, cp: pac::CorePeripherals) {
    let mut gpioa = dp.GPIOA.split();
    let mut afio = dp.AFIO.constrain();

    let led = unsafe { &mut *LED.as_mut_ptr() };
    let int_pin = unsafe { &mut *INT_PIN.as_mut_ptr() };
    let timer = unsafe { &mut *TIMER.as_mut_ptr() };

    *led = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    *int_pin = gpioa.pa5.into_pull_up_input(&mut gpioa.crl);
    *timer = timer::Timer::syst(
        cp.SYST,
        &dp.RCC
            .constrain()
            .cfgr
            .freeze(&mut dp.FLASH.constrain().acr),
    )
    .delay();

    int_pin.make_interrupt_source(&mut afio);
    int_pin.trigger_on_edge(&mut dp.EXTI, Edge::Falling);
    int_pin.enable_interrupt(&mut dp.EXTI);
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
