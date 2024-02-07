use crate::mem_init::*;
use crate::oled::Oled;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::{i2c, pac, serial};

pub unsafe fn system_init() {
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
        .adcclk(8_u32.MHz())
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

    // System Init && Panic
    TX.write(serial.0);
    SYSDELAY.write(cp.SYST.delay(&clocks));
    PANIC_LED.write(gpioc.pc13.into_push_pull_output(&mut gpioc.crh));

    {
        // User Code Here ⬇️

        let mut gpiob = dp.GPIOB.split();
        let oled_pins = (
            gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl),
            gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl),
        );

        OLED.write(Oled::new(i2c::BlockingI2c::i2c1(
            dp.I2C1,
            oled_pins,
            &mut afio.mapr,
            i2c::Mode::Standard {
                frequency: 400_u32.kHz(),
            },
            clocks,
            1000,
            10,
            1000,
            1000,
        )));
    }
}
