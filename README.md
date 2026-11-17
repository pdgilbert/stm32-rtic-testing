# rtic embedded_hal Example testing   WIP

##  Contents

See the auto-generated menu in the github README display (above right).

## Summary

This `Rust` crate-like repository is an extract of rtic examples from
repository `rust-integration-testing`. See there for more details.

## Building

The examples can be built manually by setting one of these lines:
```
               environment variables for cargo                       openocd         embed        test board and processor
  _____________________________________________________________     _____________  _____________   ___________________________
  export HAL=stm32f0xx MCU=stm32f030xc TARGET=thumbv6m-none-eabi    PROC=stm32f0x  CHIP=STM32F0x  # none-stm32f030      Cortex-M0
  export HAL=stm32f0xx MCU=stm32f042   TARGET=thumbv6m-none-eabi    PROC=stm32f0x  CHIP=STM32F0x  # none-stm32f042      Cortex-M0
  export HAL=stm32f1xx MCU=stm32f103   TARGET=thumbv7m-none-eabi    PROC=stm32f1x  CHIP=STM32F103C8  # bluepill         Cortex-M3
  export HAL=stm32f1xx MCU=stm32f100   TARGET=thumbv7m-none-eabi    PROC=stm32f1x  CHIP=STM32F1x  # none-stm32f100      Cortex-M3
  export HAL=stm32f1xx MCU=stm32f101   TARGET=thumbv7m-none-eabi    PROC=stm32f1x  CHIP=STM32F1x  # none-stm32f101      Cortex-M3
  export HAL=stm32f3xx MCU=stm32f303xc TARGET=thumbv7em-none-eabihf PROC=stm32f3x  CHIP=STM32F3x  # discovery-stm32f303 Cortex-M3
  export HAL=stm32f4xx MCU=stm32f401   TARGET=thumbv7em-none-eabihf PROC=stm32f4x  CHIP=STM32F4x  # blackpill-stm32f401 Cortex-M4
  export HAL=stm32f4xx MCU=stm32f411   TARGET=thumbv7em-none-eabihf PROC=stm32f4x  CHIP=STM32F4x  # blackpill-stm32f411 Cortex-M4
  export HAL=stm32f4xx MCU=stm32f411   TARGET=thumbv7em-none-eabihf PROC=stm32f4x  CHIP=STM32F4x  # nucleo-64           Cortex-M4
  export HAL=stm32f7xx MCU=stm32f769   TARGET=thumbv7em-none-eabihf PROC=stm32f7x  CHIP=STM32F7x  # none-stm32f769      Cortex-M7
  (                    MCU=stm32f722    no adc breaks some examples  }
  export HAL=stm32g0xx MCU=stm32g081   TARGET=thumbv6m-none-eabi    PROC=stm32g0   CHIP=STM32G081 # none-stm32g081      Cortex-M0
  export HAL=stm32g0xx MCU=stm32g071   TARGET=thumbv6m-none-eabi    PROC=stm32g0   CHIP=STM32G071 # none-stm32g071      Cortex-M0
  export HAL=stm32g4xx MCU=stm32g431xB TARGET=thumbv7em-none-eabihf PROC=stm32g4x  CHIP=STM32G4x  # weact-stm32g431CBU6 Cortex-M4
  #export HAL=stm32g4xx MCU=stm32g473   TARGET=thumbv7em-none-eabihf PROC=stm32g4x  CHIP=STM32G4x  # none-stm32g473      Cortex-M4
  export HAL=stm32g4xx MCU=stm32g474xE TARGET=thumbv7em-none-eabihf PROC=stm32g4x  CHIP=STM32G4x  # weact-stm32g474CEU6 Cortex-M4
  #export HAL=stm32h7xx MCU=stm32h742   TARGET=thumbv7em-none-eabihf PROC=          CHIP=          # none-stm32h742      Cortex-M7
  #export HAL=stm32h7xx MCU=stm32h743   TARGET=thumbv7em-none-eabihf PROC=          CHIP=          # devEBox-stm32h743VIT6 Cortex-M7
  export HAL=stm32h7xx MCU=stm32h750   TARGET=thumbv7em-none-eabihf PROC=          CHIP=          # devEBox-stm32h750VBT6 Cortex-M7
  #export HAL=stm32l0xx MCU=stm32l042     TARGET=thumbv6m-none-eabi    PROC=stm32l0   CHIP=STM32L0   # none-stm32l042      Cortex-M0
  export  HAL=stm32l0xx MCU=stm32l0x2kztx TARGET=thumbv6m-none-eabi    PROC=stm32l0   CHIP=STM32L072KZTx # none-stm32l072  Cortex-M0
  #export HAL=stm32l0xx MCU=stm32l053r8tx TARGET=thumbv6m-none-eabi    PROC=stm32l0   CHIP=STM32L053R8Tx   # none-stm32l053 Cortex-M0
  export HAL=stm32l1xx MCU=stm32l100   TARGET=thumbv7m-none-eabi    PROC=stm32l1   CHIP=STM32L1   # discovery-stm32l100 Cortex-M3
  export HAL=stm32l1xx MCU=stm32l151   TARGET=thumbv7m-none-eabi    PROC=stm32l1   CHIP=STM32L1   # heltec-lora-node151 Cortex-M3
  export HAL=stm32l4xx MCU=stm32l422   TARGET=thumbv7em-none-eabi   PROC=stm32l4x  CHIP=STM32L4x  # none-stm32l4x1      Cortex-M4
```
then to build
```
cargo build --no-default-features --target $TARGET --features $MCU,$HAL --example xxx
```
where `xxx` is replaced by one of the example names, such as
```
cargo build --no-default-features --target $TARGET --features $MCU,$HAL --example blink_rtic
cargo build --no-default-features --target $TARGET --features $MCU,$HAL --example displayX2_rtic4a1

```
See directories in `examples/` or `Cargo.toml` for example names.

## Loading

The build testing here does not include linking, flashing, running, or any actual
hardware testing. Details about this process can be found elsewhere, briefly:

If `cargo-embed`, `probe-rs` and an appropriate probe are in place then 
```
cargo embed  --target $TARGET  --features $HAL,$MCU  --chip $CHIP --example xxx
```
where `xxx` is one of the examples, such as
```
cargo embed  --target $TARGET  --features $HAL,$MCU  --chip $CHIP --example veml6070-uv-display-bp
```

If `openocd`, `gdb`, `.cargo/config` with needed runners, and an appropriate probe are 
in place then in one window run
```
openocd -f interface/$INTERFACE.cfg -f target/$PROC.cfg 
```
where INTERFACE is set for your probe, for example, `export INTERFACE=stlink-v2` 
(or now simplified to `export INTERFACE=stlink`)  for a typical cheap dongle
or `export INTERFACE=stlink-v2-1` for a slightly newer version.
Note that sometimes, if an rtic or other program with sleep while idle is in the MCU flash, then
it may be necessary to hold boot0 and press nrst in order to get openocd to take control
and allow debug/flash to proceed.

In another window do
```
cargo  run --target $TARGET --features $HAL,$MCU --example xxx  [ --release]
```
The `--release` will be needed if code is too big for memory.

For examples that need a serial connection run a terminal session such as 
```
minicom -D /dev/ttyUSB0 -b9600
```
where 0 is replaced by the active USB connection number which can be found with ` dmesg | grep -i tty `

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
