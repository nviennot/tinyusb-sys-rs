[![crates.io](https://img.shields.io/crates/d/tinyusb-sys.svg)](https://crates.io/crates/tinyusb-sys)
[![crates.io](https://img.shields.io/crates/v/tinyusb-sys.svg)](https://crates.io/crates/tinyusb-sys)

TinyUSB Rust Bindings
=====================

[TinyUSB](https://github.com/hathach/tinyusb) is an open-source cross-platform
USB Host/Device stack for embedded system, designed to be memory-safe with no
dynamic allocation and thread-safe with all interrupt events are deferred then
handled in the non-ISR task function.

This crate provides Rust bindings for the library.

Declare the dependency as such, for example to host a mass storage class device
on an MCU from the STM32F1 family:

```
[dependencies]
tinyusb-sys = { version = "0.1", features = ["host", "msc", "stm32f1"] }
```

## Feature definitions

To use the crate, you must pick some features. You must pick one mode (host or
device), at least one USB device class, and an MCU family.

Code examples will come soon.

### Mode

* `host`
* `device`

### Host features

* `msc`
* `cdc`
* `vendor`
* `hub`
* `hid`
* `midi`

### Device features

* `bth`
* `video`
* `ecm_rndis`
* `dfu`
* `midi`
* `ncm`
* `dfu_runtime`
* `hid`
* `vendor`
* `usbtmc`
* `audio`
* `msc`
* `cdc`

### MCU Family

* `lpc11uxx`     NXP LPC11Uxx
* `lpc13xx`      NXP LPC13xx
* `lpc15xx`      NXP LPC15xx
* `lpc175x_6x`   NXP LPC175x, LPC176x
* `lpc177x_8x`   NXP LPC177x, LPC178x
* `lpc18xx`      NXP LPC18xx
* `lpc40xx`      NXP LPC40xx
* `lpc43xx`      NXP LPC43xx
* `lpc51uxx`     NXP LPC51U6x
* `lpc54xxx`     NXP LPC54xxx
* `lpc55xx`      NXP LPC55xx
* `nrf5x`        Nordic nRF5x series
* `samd21`       MicroChip SAMD21
* `samd51`       MicroChip SAMD51
* `samg`         MicroChip SAMDG series
* `same5x`       MicroChip SAM E5x
* `samd11`       MicroChip SAMD11
* `saml22`       MicroChip SAML22
* `saml21`       MicroChip SAML21
* `samx7x`       MicroChip SAME70, S70, V70, V71 family
* `stm32f0`      ST F0
* `stm32f1`      ST F1
* `stm32f2`      ST F2
* `stm32f3`      ST F3
* `stm32f4`      ST F4
* `stm32f7`      ST F7
* `stm32h7`      ST H7
* `stm32l1`      ST L1
* `stm32l0`      ST L0
* `stm32l4`      ST L4
* `stm32g0`      ST G0
* `stm32g4`      ST G4
* `cxd56`        SONY CXD56
* `msp430x5xx`   TI MSP430x5xx
* `msp432e4`     TI MSP432E4xx
* `tm4c123`      TI Tiva-C 123x
* `tm4c129`      TI Tiva-C 129x
* `valentyusb_eptri` Fomu eptri config
* `mimxrt10xx`   NXP iMX RT10xx
* `nuc121`
* `nuc126`
* `nuc120`
* `nuc505`
* `esp32s2`      Espressif ESP32-S2
* `esp32s3`      Espressif ESP32-S3
* `da1469x`      Dialog Semiconductor DA1469x
* `rp2040`       Raspberry Pi RP2040
* `mkl25zxx`     NXP MKL25Zxx
* `k32l2bxx`     NXP K32L2Bxx
* `efm32gg`      Silabs EFM32GG
* `rx63x`        Renesas RX63N/631
* `rx65x`        Renesas RX65N/RX651
* `rx72n`        Renesas RX72N
* `mm32f327x`    Mind Motion MM32F327
* `gd32vf103`    GigaDevice GD32VF103
* `bcm2711`      Broadcom BCM2711
* `bcm2835`      Broadcom BCM2835
* `bcm2837`      Broadcom BCM2837
* `xmc4000`      Infineon XMC4000
* `pic32mz`      MicroChip PIC32MZ family
* `ft90x`        BridgeTek FT90x
* `ft93x`        BridgeTek FT93x
* `f1c100s`      Allwinner F1C100s family
