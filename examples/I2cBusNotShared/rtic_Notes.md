THIS PAGE MAY BE A LOT OF WORK TO MAINTAIN. Abandon for now.

# rtic Example Notes and Status Testing

## Summary
The rtic examples in this directory can be divided into 3 groups: 
- examples with no I2c bus
- examples with single devices on an I2c bus
- examples with more than one device on an I2c bus (shared i2c bus)

Note that "shared i2c bus" is a hardware configuration and not the same as an 
'rtic' 'Shared' resource, which is a software design.

Prior to 'embedded-hal 1.0' a shared i2c bus could be done with the 'shared_bus' crate
and examples in this directory mostly worked. With 'embedded-hal 1.0' a shared i2c bus
is more complicated (and not yet completely working for me).

Status indicated in the following is as of Nov 1, 2026. 'Status: compiles' means it compiles
with git versions of 'stm32f1xx-hal',  'stm32f4xx-hal', and 'stm32g4xx-hal'.
No hardware run tests have been done recently.


## No I2c bus

### resource-user-struct.rs
Status: compiles
- example modified for stm32 hal crates from 
  https://github.com/rtic-rs/rtic/examples/lm3s6965/examples/resource-user-struct.rs
- (Shared { shared: 0 }, Local {})

### schedule.rs
Status: compiles
- from  /cortex-m-rtic/examples/schedule.rs, reworked for rtic v2. 


## Single Devices I2c Bus(es)
### dht_rtic.rs
Status: compiles
- temperature and humidity from a DHT11 or DHT22 on data pin 
- SSD1306  display on i2c
- (Shared { led, }, Local {dht, display, delay })


### aht10-aht30_rtic.rs
Status: fails. trait and other problems
- temperature and humidity from sensor on i2c2. Display on ssd1306 OLED on i2c1.
- sensor uses crate 'aht30' which also supports 'AHT10' sensors.
- (Shared { led, },   Local {display, sensor, delay})
- Blink (onboard) LED with short pulse every read.

## Shared i2c bus(es)



===========================
### adc_dma.rs
### ads_ina_ssd_rtic.rs

### aht10_rtic.rs
### battery_monitor_ads1015_rtic_dma.rs
### battery_monitor_ads1015_rtic.rs
### blink_rtic.rs
### ccs811-co2-voc.rs
### display_stuff_rtic0.rs
### display_stuff_rtic_rcd.rs
### display_stuff_rtic.rs
### displayX2_rtic.rs
### gps_rw_rtic.rs
### htu2xd_rtic.rs  sensor on shared bus,  htu21df-sensor v0.1.4
### ina219_rtic.rs   no shared bus. ina219 uses embedded-hal v1.
### serial-dma.rs

ALSO  display_stuff_rtic*  displayX1_rtic_*

## Status Nov 1, 2026

f4         dev-testing $ ls -1 examples/rtic/
not used   adc_dma.rs
fail       ads_ina_ssd_rtic.rs
fail       aht10-aht30_rtic.rs
fail share aht10_rtic.rs
fail share battery_monitor_ads1015_rtic_dma.rs
compiles   battery_monitor_ads1015_rtic.rs
compiles   blink_rtic.rs
fail       ccs811-co2-voc.rs
compiles   dht_rtic.rs
compiles   display_stuff_rtic0.rs
not used   display_stuff_rtic_rcd.rs
fail       display_stuff_rtic.rs
fail life  displayX2_rtic.rs
fail read  gps_rw_rtic.rs
compiles   htu2xd_rtic.rs  sensor on shared bus,  htu21df-sensor v0.1.4 uses embedded-hal v0.2.7
compiles   ina219_rtic.rs   no shared bus. ina219 uses embedded-hal v1.0.0
compiles   resource-user-struct.rs
compiles   schedule.rs
fail       serial-dma.rs
