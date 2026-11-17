//!   Two I2c buses with one device each. No bus sharing.
//!   ssd1306 display on I2C1 and another on I2C2. Trying AtomicCell<>
//!   Has (Shared { i2c1_cell,  },   Local {i2c2_cell}) Not building but did previously
//! AtomicCell and AtomicDevice are used to wrap the shared bus.

// BUILDS with i2c1_cell i2c2_cell in Local and displays constructed in spawned tasks.

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

    type I2c1Type = I2c<I2C1>;
    type I2c2Type = I2c<I2C2>;

    type AtomicCell1Type = AtomicCell<I2c1Type>;
    type AtomicCell2Type = AtomicCell<I2c2Type>;

    type Interface1Type<'a> = I2CInterface<AtomicDevice<'a, I2c1Type>>;
    type Interface2Type<'a> = I2CInterface<AtomicDevice<'a, I2c2Type>>;

    type DisplayAType<'a> = Ssd1306<Interface1Type<'a>, 
           ssd1306::prelude::DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>;
    type DisplayBType<'a> = Ssd1306<Interface2Type<'a>, 
           ssd1306::prelude::DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>;

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
    struct Shared {
        i2c1_cell:AtomicCell1Type,
    }
    
    #[local]
    struct Local {
        //i2c1_cell:AtomicCell1Type,
        i2c2_cell:AtomicCell2Type,
    }


    #[init]
     fn init(cx: init::Context) -> (Shared, Local) {
  
       Mono::start(cx.core.SYST, MONOCLOCK);

       let mut rcc = cx.device.RCC.constrain().freeze(rccConfig::hsi() );

       let gpiob = cx.device.GPIOB.split(&mut rcc);

       let scl = gpiob.pb8.into_alternate_open_drain(); 
       let sda = gpiob.pb9.into_alternate_open_drain(); 
       let i2c1:I2c1Type = I2c::new(cx.device.I2C1, (scl, sda), 400.kHz(), &mut rcc);
       let i2c1_cell:AtomicCell1Type = AtomicCell::new(i2c1);
 
       let scl = gpiob.pb10.into_alternate_open_drain();
       let sda = gpiob.pb3.into_alternate_open_drain();
       let i2c2:I2c2Type = I2c::new(cx.device.I2C2, (scl, sda), 400.kHz(), &mut rcc);  
       let i2c2_cell:AtomicCell2Type = AtomicCell::new(i2c2);

        Mono.delay_ms(5000u32);  

        displayA_stuff::spawn().unwrap();
        displayB_stuff::spawn().unwrap();

        (Shared { i2c1_cell,  },   Local {i2c2_cell})
    }

    #[task(shared = [i2c1_cell,], )]
    async fn displayA_stuff(cx: displayA_stuff::Context) {
       let interfaceA:Interface1Type = I2CDisplayInterface::new(AtomicDevice::new(&cx.shared.i2c1_cell));//default address 0x3C
       let mut display_a:DisplayAType = Ssd1306::new(interfaceA, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display_a.init().unwrap();

        loop {
           show_message("display A stuff",  &mut display_a);
           Mono::delay(ExtU32::secs(2)).await;
       }
    }

    #[task(shared = [i2c1_cell], )]
    async fn displayB_stuff(cx: displayB_stuff::Context) {
       let interfaceB:Interface2Type = I2CDisplayInterface::new(AtomicDevice::new(&cx.shared.i2c1_cell));//default address 0x3C
       let mut display_b:DisplayBType = Ssd1306::new(interfaceB, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display_b.init().unwrap();

        loop {
           show_message("display B stuff",  &mut display_b);
           Mono::delay(ExtU32::secs(2)).await;
       }
    }

}
