#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer};

#[entry]
fn entry() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut timer =
        timer::Timer::syst(cp.SYST, &dp.RCC.constrain().cfgr.freeze(&mut flash.acr)).delay();

    let mut gpioc = dp.GPIOC.split();
    let mut board_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    loop {
        board_led.toggle();
        timer.delay_ms(300_u32);
    }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
