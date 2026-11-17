//!   One I2c bus with two devices sharing it. One thead.
//!   Two ssd1306 displays on I2C1. Using RefCell<I2c<I2C1>>
//!   Has  Local { i2c1}. BUILDS (displays done in task and then loop)

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
      pac::{I2C1, },
      rcc::{RccExt},
      i2c::I2c,   //this is a type
      gpio::{ GpioExt, },
      prelude::*,
      rcc::{Config as rccConfig},
    };

    use rtic_monotonics::fugit::ExtU32;
    const MONOCLOCK: u32 = 16_000_000; 

    use embedded_hal::delay::DelayNs;

    use core::cell::RefCell;
    use embedded_hal_bus::i2c::RefCellDevice;

    type DisplayI2c1<'a> = Ssd1306<
        //I2CInterface<RefCellDevice<'static, I2c<'static, I2C1>>>, 
        I2CInterface<RefCellDevice<'a, I2c<I2C1>>>, 
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
        //display_a: DisplayI2c1,
        //display_b: DisplayI2c1,
        i2c1: RefCell<I2c<I2C1>>,

    }

//    bind_interrupts..  in embassy ??

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
  
       Mono::start(cx.core.SYST, MONOCLOCK);

       let mut rcc = cx.device.RCC.constrain().freeze(rccConfig::hsi() );

       let gpiob = cx.device.GPIOB.split(&mut rcc);

       let scl = gpiob.pb8.into_alternate_open_drain(); 
       let sda = gpiob.pb9.into_alternate_open_drain(); 
       let i2c1 = RefCell::new(I2c::new(cx.device.I2C1, (scl, sda), 400.kHz(), &mut rcc));
 
        Mono.delay_ms(5000u32);  

        displayAB_stuff::spawn().unwrap();

      (Shared { },   Local { i2c1})
    }

//    #[task(shared = [], local = [display_a, display_b, ])]
    #[task(shared = [], local = [i2c1, ])]
    async fn displayAB_stuff(cx: displayAB_stuff::Context) {
       let interfaceA = I2CDisplayInterface::new(
                  RefCellDevice::new(cx.local.i2c1));  //uses default address 0x3C
       //let interfaceA = I2CDisplayInterface::new_custom_address(
       //           RefCellDevice::new(cx.local.i2c1), 0x3C); //specify address (default)
       let mut display_a: DisplayI2c1 = Ssd1306::new(interfaceA, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
       display_a.init().unwrap();

       let interfaceB = I2CDisplayInterface::new_custom_address(
                  RefCellDevice::new(cx.local.i2c1), 0x3D); //alt address
       let mut display_b: DisplayI2c1 = Ssd1306::new(interfaceB, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
       display_b.init().unwrap();
        // Tasks that loop forever need an `await` somewhere in the loop, otherwise
        // they never yield back to the async executor,
        // so other lower or equal  priority tasks will not run.
        loop {
           show_message("display A stuff", &mut display_a);
           show_message("display B stuff", &mut display_b);
           Mono::delay(ExtU32::secs(2)).await;
       }
    }

}
