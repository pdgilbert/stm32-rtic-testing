//!   Two I2c buses with one device each. No bus sharing.
//!   ssd1306 display on I2C1 and another on I2C2. Trying  MaybeUninit.
//!   Has Local { display_a, display_b}. Does not build. expected named lifetime

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

    type I2c1Type<'a> = I2c<I2C1>;
    type I2c2Type<'a> = I2c<I2C2>;

    type AtomicCell1Type<'a> = AtomicCell<I2c1Type<'a>>;
    type AtomicCell2Type<'a> = AtomicCell<I2c2Type<'a>>;

    type Interface1Type<'a> = I2CInterface<AtomicDevice<'a, I2c1Type<'a>>>;
    type Interface2Type<'a> = I2CInterface<AtomicDevice<'a, I2c2Type<'a>>>;

    type DisplayAType<'a> = Ssd1306<Interface1Type<'a>, 
           ssd1306::prelude::DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>;
    type DisplayBType<'a> = Ssd1306<Interface2Type<'a>, 
           ssd1306::prelude::DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>;


//let i2c1: I2c1Type<'static> = I2c::new(cx.device.I2C1, (scl, sda), 400.kHz(), &mut rcc);
//let spi = Spi::new_blocking_txonly(p.SPI1, p.PA5, p.PA7, config);

    type Display1 = Ssd1306<
        I2CInterface<AtomicDevice<'static, I2c<I2C1>>>,
        ssd1306::prelude::DisplaySize128x32,
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
        //i2c1_cell:AtomicCell1Type,
        //i2c2_cell:AtomicCell2Type,
        //interfaceA: Interface1Type<'a>,
        //interfaceB: Interface2Type<'a>,
        //display_a:  DisplayAType<'static>,
        //display_b:  DisplayBType<'static>,
        display_a:  & mut DisplayAType<'static>,
        display_b:  & mut DisplayBType<'static>,

    }

    use core::mem::MaybeUninit;

//    #[init]
     
    #[init(local = [
            // Note that MaybeUninit  fn write(&mut self, val: T) -> &mut T  returns '&mut T' so 
            //  if T is '&mut U' then result is '&mut &mut U'
            //  if T is '& U' then result is '& &mut U'
            //display_a_cx: MaybeUninit<Ssd1306<I2CInterface<AtomicDevice<'static, &'static mut I2c1Type>>, 
            //              ssd1306::prelude::DisplaySize128x32, 
            //              BufferedGraphicsMode<DisplaySize128x32>>> = MaybeUninit::uninit()
            display_a_cx: MaybeUninit<DisplayAType<'static>> = MaybeUninit::uninit(),
            display_b_cx: MaybeUninit<DisplayBType<'static>> = MaybeUninit::uninit(),
            //display_a_cx: MaybeUninit<&'static mut DisplayAType<'static>> = MaybeUninit::uninit(),
            //display_b_cx: MaybeUninit<&'static mut DisplayBType<'static>> = MaybeUninit::uninit(),
            //display_a_cx: MaybeUninit<&'static Ssd1306<I2CInterface<
            //                    &'static mut AtomicDevice<'static, I2c1Type>>, 
            //                    ssd1306::prelude::DisplaySize128x32, 
            //                    BufferedGraphicsMode<DisplaySize128x32>>> = MaybeUninit::uninit(),
            //display_a_cx: MaybeUninit<&'static Ssd1306<I2CInterface<
            //                    &'static mut I2c1Type<'static>>, 
            //                    ssd1306::prelude::DisplaySize128x32, 
            //                    BufferedGraphicsMode<DisplaySize128x32>>> = MaybeUninit::uninit(),
            //display_b_cx: MaybeUninit<&'static Ssd1306<I2CInterface<
            //                    &'static mut I2c2Type<'static>>, 
            //                    ssd1306::prelude::DisplaySize128x32, 
            //                    BufferedGraphicsMode<DisplaySize128x32>>> = MaybeUninit::uninit(),

    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
  
       Mono::start(cx.core.SYST, MONOCLOCK);

       let mut rcc = cx.device.RCC.constrain().freeze(rccConfig::hsi() );

       let gpiob = cx.device.GPIOB.split(&mut rcc);

       let scl = gpiob.pb8.into_alternate_open_drain(); 
       let sda = gpiob.pb9.into_alternate_open_drain(); 
       let i2c1: I2c1Type<'static> = I2c::new(cx.device.I2C1, (scl, sda), 400.kHz(), &mut rcc);
       let i2c1_cell:AtomicCell1Type<'static> = AtomicCell::new(i2c1);
       let interfaceA:Interface1Type<'static> = I2CDisplayInterface::new(AtomicDevice::new(&i2c1_cell));//default address 0x3C
       //let interfaceA = I2CDisplayInterface::new(i2c1);//default address 0x3C
       let mut display_a_tmp: Display1 = Ssd1306::new(interfaceA, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
       display_a_tmp.init().unwrap();

       //  write into local resource created by  MaybeUninit
       // Using `write` instead of assignment via `=` to not call `drop` on the old, uninitialized value.
       //  https://doc.rust-lang.org/stable/core/mem/union.MaybeUninit.html#method.write
       //       pub const fn write(&mut self, val: T) -> &mut T
       //    returns a mutable reference to T
       //let display_a_tmp2: &'static mut DisplayAType<'static> = cx.local.display_a_cx.write(display_a_tmp);
       //let display_a_tmp2: DisplayAType<'static> = cx.local.display_a_cx.write(&mut display_a_tmp);
       //let display_a_tmp2:DisplayAType<'static> = cx.local.display_a_cx.write(display_a_tmp);
       let display_a_tmp2 = cx.local.display_a_cx.write(display_a_tmp);


       let scl = gpiob.pb10.into_alternate_open_drain();
       let sda = gpiob.pb3.into_alternate_open_drain();
       let i2c2: I2c2Type<'static> = I2c::new(cx.device.I2C2, (scl, sda), 400.kHz(), &mut rcc);  
       let i2c2_cell:AtomicCell2Type<'static> = AtomicCell::new(i2c2);
       let interfaceB:Interface2Type<'static> = I2CDisplayInterface::new(AtomicDevice::new(&i2c2_cell));//default address 0x3C
       //let interfaceB = I2CDisplayInterface::new(i2c2);//default address 0x3C
       let mut display_b_tmp: DisplayBType<'static> = Ssd1306::new(interfaceB, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
       display_b_tmp.init().unwrap();

        //let display_b_tmp2: &'static mut DisplayBType<'static> = cx.local.display_b_cx.write(display_b_tmp);
        //let display_b_tmp2: DisplayBType<'static> = cx.local.display_b_cx.write(&mut display_b_tmp);
        //let display_b_tmp2: DisplayBType<'static> = cx.local.display_b_cx.write(display_b_tmp);
        let display_b_tmp2 = cx.local.display_b_cx.write(display_b_tmp);

        Mono.delay_ms(5000u32);  

        displayA_stuff::spawn().unwrap();
        displayB_stuff::spawn().unwrap();

      //(Shared { },   Local { interfaceA, interfaceB })
      (Shared { },   Local { display_a: display_a_tmp2, display_b: display_b_tmp2})
    }

    #[task(shared = [], local = [display_a, ])]
    async fn displayA_stuff(cx: displayA_stuff::Context) {
      // let interfaceA:Interface1Type = I2CDisplayInterface::new(AtomicDevice::new(&cx.local.i2c1_cell));//default address 0x3C
      // let interfaceA:Interface1Type = cx.local.interfaceA;
      // let mut display_a:DisplayAType = Ssd1306::new(interfaceA, DisplaySize128x32, DisplayRotation::Rotate0)
      //      .into_buffered_graphics_mode();

        let mut display_a:&mut DisplayAType = cx.local.display_a;

        display_a.init().unwrap();
        // Tasks that loop forever need an `await` somewhere in the loop, otherwise
        // they never yield back to the async executor,
        // so other lower or equal  priority tasks will not run.

        loop {
           show_message("display A stuff",  &mut display_a);
           Mono::delay(ExtU32::secs(2)).await;
       }
    }

    #[task(shared = [], local = [display_b, ])]
    async fn displayB_stuff(cx: displayB_stuff::Context) {
       // let interfaceB:Interface2Type = I2CDisplayInterface::new(AtomicDevice::new(&cx.local.i2c2_cell));//default address 0x3C
       // let interfaceB:Interface2Type = cx.local.interfaceB;
       // let mut display_b:DisplayBType = Ssd1306::new(interfaceB, DisplaySize128x32, DisplayRotation::Rotate0)
       //     .into_buffered_graphics_mode();

        let mut display_b:&mut DisplayBType = cx.local.display_b;

        display_b.init().unwrap();

        loop {
           show_message("display B stuff",  &mut display_b);
           Mono::delay(ExtU32::secs(2)).await;
       }
    }

}
