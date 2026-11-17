//!   Two I2c buses with one device each. No bus sharing.
//!   ssd1306 display on I2C1 and another on I2C2. Trying  ATOMICCELL.
//!   Has Local { display_a, display_b}. Does not build.

// NOT YET BUILDS with displays initialize in init and put in Local and using AtomicCell.

//  See displayX2_rticXXX.txt for documentation.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use rtic::app;
use rtic_monotonics::systick_monotonic;
systick_monotonic!(Mono, 1000); 

#[cfg_attr(feature = "stm32f4xx", app(device = stm32f4xx_hal::pac,   dispatchers = [TIM2, TIM3]))]
//#[cfg_attr(feature = "stm32g4xx", app(device = stm32g4xx_hal::pac,   dispatchers = [TIM2, TIM3]))]

mod app {
    
    use rtic;
    use crate::Mono;
    use rtic_monotonics::systick::prelude::*;
   
    use embedded_graphics::{
        mono_font::{iso_8859_1::FONT_10X20 as FONT, MonoTextStyleBuilder}, 
        pixelcolor::BinaryColor,
        prelude::*,
        text::{Baseline, Text},
    };

    use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

    use stm32f4xx_hal::{
      pac::{I2C1, I2C2,},
      rcc::{RccExt},
      i2c::I2c,   //this is a type
      gpio::{ GpioExt, },
      prelude::*,
      rcc::{Config as rccConfig},
    };

    use rtic_monotonics::fugit::ExtU32;
    const MONOCLOCK: u32 = 16_000_000; 

    use embedded_hal::delay::DelayNs;
    use embedded_hal_bus::i2c::AtomicDevice;
    use embedded_hal_bus::util::AtomicCell;

    type DisplayA = Ssd1306<
        //I2CInterface<I2c<I2C1>>, 
        I2CInterface<AtomicDevice<'static, I2c<I2C1>>>, 
        DisplaySize128x32,
        BufferedGraphicsMode<DisplaySize128x32>,
    >;

    type DisplayB = Ssd1306<
        //I2CInterface<I2c<I2C2>>, 
        I2CInterface<AtomicDevice<'static, I2c<I2C2>>>, 
        DisplaySize128x32,
        BufferedGraphicsMode<DisplaySize128x32>,
    >;


    fn show_message<S>(
       text: &str, 
       disp: &mut Ssd1306<impl WriteOnlyDataCommand, S, BufferedGraphicsMode<S>>,
    ) -> ()
    where
        S: DisplaySize,
    {         
       let text_style = MonoTextStyleBuilder::new().font(&FONT).text_color(BinaryColor::On).build();
    
       disp.clear_buffer();
       Text::with_baseline( &text, Point::new(0, 0), text_style, Baseline::Top)
               .draw(&mut *disp)
               .unwrap();

       disp.flush().unwrap();
       ()
    }

    #[shared]
    struct Shared { }
    
    #[local]
    struct Local {
        display_a: DisplayA,
        display_b: DisplayB,
    }

//    bind_interrupts..  in embassy ??

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
  
       Mono::start(cx.core.SYST, MONOCLOCK);

       let mut rcc = cx.device.RCC.constrain().freeze(rccConfig::hsi() );

       let gpiob = cx.device.GPIOB.split(&mut rcc);

       let scl = gpiob.pb8.into_alternate_open_drain(); 
       let sda = gpiob.pb9.into_alternate_open_drain(); 
       let i2c1 = I2c::new(cx.device.I2C1, (scl, sda), 400.kHz(), &mut rcc);
   //    let interfaceA = I2CDisplayInterface::new(i2c1);//default address 0x3C
       let i2c1_cell = AtomicCell::new(i2c1);
       let interfaceA = I2CDisplayInterface::new(AtomicDevice::new(&i2c1_cell));//default address 0x3C
  //     let interfaceA = I2CDisplayInterface::new(i2c1);//default address 0x3C
       let mut display_a = Ssd1306::new(interfaceA, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
       display_a.init().unwrap();

       let scl = gpiob.pb10.into_alternate_open_drain();
       let sda = gpiob.pb3.into_alternate_open_drain();
       let i2c2 = I2c::new(cx.device.I2C2, (scl, sda), 400.kHz(), &mut rcc);  
       //let interfaceB = I2CDisplayInterface::new(i2c2);//default address 0x3C
       let i2c2_cell = AtomicCell::new(i2c2);
       let interfaceB = I2CDisplayInterface::new(AtomicDevice::new(&i2c2_cell));//default address 0x3C
       //let interfaceB = I2CDisplayInterface::new(i2c2_cell);//default address 0x3C
       let mut display_b = Ssd1306::new(interfaceB, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
       display_b.init().unwrap();

        Mono.delay_ms(5000u32);  

        displayA_stuff::spawn().unwrap();
        displayB_stuff::spawn().unwrap();

      (Shared { },   Local { display_a, display_b})
    }

    #[task(shared = [], local = [display_a, ])]
    async fn displayA_stuff(cx: displayA_stuff::Context) {
        // Tasks that loop forever need an `await` somewhere in the loop, otherwise
        // they never yield back to the async executor,
        // so other lower or equal  priority tasks will not run.

        loop {
           show_message("display A stuff", cx.local.display_a);
           Mono::delay(ExtU32::secs(2)).await;
       }
    }

    #[task(shared = [], local = [display_b, ])]
    async fn displayB_stuff(cx: displayB_stuff::Context) {

        loop {
           show_message("display B stuff", cx.local.display_b);
           Mono::delay(ExtU32::secs(2)).await;
       }
    }

}
