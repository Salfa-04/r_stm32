#![no_std]
#![no_main]

use core::fmt::Write;
use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::prelude::*;

use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::{gpio, pac, rtc, serial};

#[entry]
fn entry() -> ! {
    system_init();

    let rtc = unsafe { &mut *RTC.as_mut_ptr() };
    let tx = unsafe { &mut *TX.as_mut_ptr() };

    loop {
        writeln!(tx, "邢彦瑶: 额锤死你啊(笑脸").unwrap();
        block!(rtc.wait_alarm()).unwrap();
    }
}

#[interrupt]
fn RTCALARM() {
    let rtc = unsafe { &mut *RTC.as_mut_ptr() };
    rtc.set_alarm(rtc.current_time() + 8);

    unsafe { &mut *LED.as_mut_ptr() }.toggle();
    unsafe { &mut *EXTI.as_mut_ptr() }
        .pr
        .write(|w| w.pr17().set_bit());
}

static mut LED: MaybeUninit<gpio::Pin<'A', 6, gpio::Output>> = MaybeUninit::uninit();
static mut EXTI: MaybeUninit<pac::EXTI> = MaybeUninit::uninit();

fn system_init() {
    let mut dp = pac::Peripherals::take().unwrap();

    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();
    let mut afio = dp.AFIO.constrain();
    let mut flash = dp.FLASH.constrain();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut bkp = rcc.bkp.constrain(dp.BKP, &mut dp.PWR);

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
        *RTC.as_mut_ptr() = rtc::Rtc::new(dp.RTC, &mut bkp);
        *PANIC_LED.as_mut_ptr() = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    };

    {
        // User Code Here ⬇️

        let rtc = unsafe { &mut *RTC.as_mut_ptr() };

        rtc.select_frequency(16_u32.Hz());
        rtc.set_alarm(rtc.current_time() + 16);
        rtc.listen_alarm();

        let exti = dp.EXTI;
        exti.ftsr.write(|w| w.tr17().set_bit());
        exti.imr.write(|w| w.mr17().set_bit());

        unsafe {
            *LED.as_mut_ptr() = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
            *EXTI.as_mut_ptr() = exti;

            pac::NVIC::unmask(pac::Interrupt::RTCALARM);
        };
    }
}

static mut TX: MaybeUninit<serial::Tx<pac::USART1>> = MaybeUninit::uninit();
static mut PANIC_LED: MaybeUninit<gpio::Pin<'C', 13, gpio::Output>> = MaybeUninit::uninit();
static mut RTC: MaybeUninit<rtc::Rtc> = MaybeUninit::uninit();

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let led = unsafe { &mut *PANIC_LED.as_mut_ptr() };
    let tx = unsafe { &mut *TX.as_mut_ptr() };
    let rtc = unsafe { &mut *RTC.as_mut_ptr() };

    rtc.select_frequency(16_u32.Hz());

    loop {
        if let Err(_) = writeln!(tx, "{}", info) {};

        led.set_low();
        rtc.set_alarm(rtc.current_time() + 3);
        block!(rtc.wait_alarm()).unwrap();

        led.set_high();
        rtc.set_alarm(rtc.current_time() + 13);
        block!(rtc.wait_alarm()).unwrap();
    }
}
