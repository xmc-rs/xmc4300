#![doc = "Peripheral access API for XMC4300 microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 6;
#[cfg(feature = "rt")]
extern "C" {
    fn SCU_0();
    fn ERU0_0();
    fn ERU0_1();
    fn ERU0_2();
    fn ERU0_3();
    fn ERU1_0();
    fn ERU1_1();
    fn ERU1_2();
    fn ERU1_3();
    fn PMU0_0();
    fn VADC0_C0_0();
    fn VADC0_C0_1();
    fn VADC0_C0_2();
    fn VADC0_C0_3();
    fn VADC0_G0_0();
    fn VADC0_G0_1();
    fn VADC0_G0_2();
    fn VADC0_G0_3();
    fn VADC0_G1_0();
    fn VADC0_G1_1();
    fn VADC0_G1_2();
    fn VADC0_G1_3();
    fn DAC0_0();
    fn DAC0_1();
    fn CCU40_0();
    fn CCU40_1();
    fn CCU40_2();
    fn CCU40_3();
    fn CCU41_0();
    fn CCU41_1();
    fn CCU41_2();
    fn CCU41_3();
    fn CCU80_0();
    fn CCU80_1();
    fn CCU80_2();
    fn CCU80_3();
    fn CAN0_0();
    fn CAN0_1();
    fn CAN0_2();
    fn CAN0_3();
    fn CAN0_4();
    fn CAN0_5();
    fn CAN0_6();
    fn CAN0_7();
    fn USIC0_0();
    fn USIC0_1();
    fn USIC0_2();
    fn USIC0_3();
    fn USIC0_4();
    fn USIC0_5();
    fn USIC1_0();
    fn USIC1_1();
    fn USIC1_2();
    fn USIC1_3();
    fn USIC1_4();
    fn USIC1_5();
    fn LEDTS0_0();
    fn FCE0_0();
    fn GPDMA0_0();
    fn SDMMC0_0();
    fn USB0_0();
    fn ETH0_0();
    fn ECAT0_0();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 110] = [
    Vector { _handler: SCU_0 },
    Vector { _handler: ERU0_0 },
    Vector { _handler: ERU0_1 },
    Vector { _handler: ERU0_2 },
    Vector { _handler: ERU0_3 },
    Vector { _handler: ERU1_0 },
    Vector { _handler: ERU1_1 },
    Vector { _handler: ERU1_2 },
    Vector { _handler: ERU1_3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PMU0_0 },
    Vector { _reserved: 0 },
    Vector { _handler: VADC0_C0_0 },
    Vector { _handler: VADC0_C0_1 },
    Vector { _handler: VADC0_C0_2 },
    Vector { _handler: VADC0_C0_3 },
    Vector { _handler: VADC0_G0_0 },
    Vector { _handler: VADC0_G0_1 },
    Vector { _handler: VADC0_G0_2 },
    Vector { _handler: VADC0_G0_3 },
    Vector { _handler: VADC0_G1_0 },
    Vector { _handler: VADC0_G1_1 },
    Vector { _handler: VADC0_G1_2 },
    Vector { _handler: VADC0_G1_3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DAC0_0 },
    Vector { _handler: DAC0_1 },
    Vector { _handler: CCU40_0 },
    Vector { _handler: CCU40_1 },
    Vector { _handler: CCU40_2 },
    Vector { _handler: CCU40_3 },
    Vector { _handler: CCU41_0 },
    Vector { _handler: CCU41_1 },
    Vector { _handler: CCU41_2 },
    Vector { _handler: CCU41_3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CCU80_0 },
    Vector { _handler: CCU80_1 },
    Vector { _handler: CCU80_2 },
    Vector { _handler: CCU80_3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CAN0_0 },
    Vector { _handler: CAN0_1 },
    Vector { _handler: CAN0_2 },
    Vector { _handler: CAN0_3 },
    Vector { _handler: CAN0_4 },
    Vector { _handler: CAN0_5 },
    Vector { _handler: CAN0_6 },
    Vector { _handler: CAN0_7 },
    Vector { _handler: USIC0_0 },
    Vector { _handler: USIC0_1 },
    Vector { _handler: USIC0_2 },
    Vector { _handler: USIC0_3 },
    Vector { _handler: USIC0_4 },
    Vector { _handler: USIC0_5 },
    Vector { _handler: USIC1_0 },
    Vector { _handler: USIC1_1 },
    Vector { _handler: USIC1_2 },
    Vector { _handler: USIC1_3 },
    Vector { _handler: USIC1_4 },
    Vector { _handler: USIC1_5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LEDTS0_0 },
    Vector { _reserved: 0 },
    Vector { _handler: FCE0_0 },
    Vector { _handler: GPDMA0_0 },
    Vector { _handler: SDMMC0_0 },
    Vector { _handler: USB0_0 },
    Vector { _handler: ETH0_0 },
    Vector { _handler: ECAT0_0 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - System Control"]
    SCU_0,
    #[doc = "1 - External Request Unit 0"]
    ERU0_0,
    #[doc = "2 - External Request Unit 0"]
    ERU0_1,
    #[doc = "3 - External Request Unit 0"]
    ERU0_2,
    #[doc = "4 - External Request Unit 0"]
    ERU0_3,
    #[doc = "5 - External Request Unit 1"]
    ERU1_0,
    #[doc = "6 - External Request Unit 1"]
    ERU1_1,
    #[doc = "7 - External Request Unit 1"]
    ERU1_2,
    #[doc = "8 - External Request Unit 1"]
    ERU1_3,
    #[doc = "12 - Program Management Unit"]
    PMU0_0,
    #[doc = "14 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_0,
    #[doc = "15 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_1,
    #[doc = "16 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_2,
    #[doc = "17 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_3,
    #[doc = "18 - Analog to Digital Converter Group 0"]
    VADC0_G0_0,
    #[doc = "19 - Analog to Digital Converter Group 0"]
    VADC0_G0_1,
    #[doc = "20 - Analog to Digital Converter Group 0"]
    VADC0_G0_2,
    #[doc = "21 - Analog to Digital Converter Group 0"]
    VADC0_G0_3,
    #[doc = "22 - Analog to Digital Converter Group 1"]
    VADC0_G1_0,
    #[doc = "23 - Analog to Digital Converter Group 1"]
    VADC0_G1_1,
    #[doc = "24 - Analog to Digital Converter Group 1"]
    VADC0_G1_2,
    #[doc = "25 - Analog to Digital Converter Group 1"]
    VADC0_G1_3,
    #[doc = "42 - Digital to Analog Converter"]
    DAC0_0,
    #[doc = "43 - Digital to Analog Converter"]
    DAC0_1,
    #[doc = "44 - Capture Compare Unit 4 (Module 0)"]
    CCU40_0,
    #[doc = "45 - Capture Compare Unit 4 (Module 0)"]
    CCU40_1,
    #[doc = "46 - Capture Compare Unit 4 (Module 0)"]
    CCU40_2,
    #[doc = "47 - Capture Compare Unit 4 (Module 0)"]
    CCU40_3,
    #[doc = "48 - Capture Compare Unit 4 (Module 1)"]
    CCU41_0,
    #[doc = "49 - Capture Compare Unit 4 (Module 1)"]
    CCU41_1,
    #[doc = "50 - Capture Compare Unit 4 (Module 1)"]
    CCU41_2,
    #[doc = "51 - Capture Compare Unit 4 (Module 1)"]
    CCU41_3,
    #[doc = "60 - Capture Compare Unit 8 (Module 0)"]
    CCU80_0,
    #[doc = "61 - Capture Compare Unit 8 (Module 0)"]
    CCU80_1,
    #[doc = "62 - Capture Compare Unit 8 (Module 0)"]
    CCU80_2,
    #[doc = "63 - Capture Compare Unit 8 (Module 0)"]
    CCU80_3,
    #[doc = "76 - MultiCAN"]
    CAN0_0,
    #[doc = "77 - MultiCAN"]
    CAN0_1,
    #[doc = "78 - MultiCAN"]
    CAN0_2,
    #[doc = "79 - MultiCAN"]
    CAN0_3,
    #[doc = "80 - MultiCAN"]
    CAN0_4,
    #[doc = "81 - MultiCAN"]
    CAN0_5,
    #[doc = "82 - MultiCAN"]
    CAN0_6,
    #[doc = "83 - MultiCAN"]
    CAN0_7,
    #[doc = "84 - Universal Serial Interface Channel (Module 0)"]
    USIC0_0,
    #[doc = "85 - Universal Serial Interface Channel (Module 0)"]
    USIC0_1,
    #[doc = "86 - Universal Serial Interface Channel (Module 0)"]
    USIC0_2,
    #[doc = "87 - Universal Serial Interface Channel (Module 0)"]
    USIC0_3,
    #[doc = "88 - Universal Serial Interface Channel (Module 0)"]
    USIC0_4,
    #[doc = "89 - Universal Serial Interface Channel (Module 0)"]
    USIC0_5,
    #[doc = "90 - Universal Serial Interface Channel (Module 1)"]
    USIC1_0,
    #[doc = "91 - Universal Serial Interface Channel (Module 1)"]
    USIC1_1,
    #[doc = "92 - Universal Serial Interface Channel (Module 1)"]
    USIC1_2,
    #[doc = "93 - Universal Serial Interface Channel (Module 1)"]
    USIC1_3,
    #[doc = "94 - Universal Serial Interface Channel (Module 1)"]
    USIC1_4,
    #[doc = "95 - Universal Serial Interface Channel (Module 1)"]
    USIC1_5,
    #[doc = "102 - LED and Touch Sense Control Unit (Module 0)"]
    LEDTS0_0,
    #[doc = "104 - Flexible CRC Engine"]
    FCE0_0,
    #[doc = "105 - General Purpose DMA Unit 0"]
    GPDMA0_0,
    #[doc = "106 - Multi Media Card Interface"]
    SDMMC0_0,
    #[doc = "107 - Universal Serial Bus (Module 0)"]
    USB0_0,
    #[doc = "108 - Ethernet (Module 0)"]
    ETH0_0,
    #[doc = "109 - EtherCAT (Module 0)"]
    ECAT0_0,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SCU_0 => 0,
            Interrupt::ERU0_0 => 1,
            Interrupt::ERU0_1 => 2,
            Interrupt::ERU0_2 => 3,
            Interrupt::ERU0_3 => 4,
            Interrupt::ERU1_0 => 5,
            Interrupt::ERU1_1 => 6,
            Interrupt::ERU1_2 => 7,
            Interrupt::ERU1_3 => 8,
            Interrupt::PMU0_0 => 12,
            Interrupt::VADC0_C0_0 => 14,
            Interrupt::VADC0_C0_1 => 15,
            Interrupt::VADC0_C0_2 => 16,
            Interrupt::VADC0_C0_3 => 17,
            Interrupt::VADC0_G0_0 => 18,
            Interrupt::VADC0_G0_1 => 19,
            Interrupt::VADC0_G0_2 => 20,
            Interrupt::VADC0_G0_3 => 21,
            Interrupt::VADC0_G1_0 => 22,
            Interrupt::VADC0_G1_1 => 23,
            Interrupt::VADC0_G1_2 => 24,
            Interrupt::VADC0_G1_3 => 25,
            Interrupt::DAC0_0 => 42,
            Interrupt::DAC0_1 => 43,
            Interrupt::CCU40_0 => 44,
            Interrupt::CCU40_1 => 45,
            Interrupt::CCU40_2 => 46,
            Interrupt::CCU40_3 => 47,
            Interrupt::CCU41_0 => 48,
            Interrupt::CCU41_1 => 49,
            Interrupt::CCU41_2 => 50,
            Interrupt::CCU41_3 => 51,
            Interrupt::CCU80_0 => 60,
            Interrupt::CCU80_1 => 61,
            Interrupt::CCU80_2 => 62,
            Interrupt::CCU80_3 => 63,
            Interrupt::CAN0_0 => 76,
            Interrupt::CAN0_1 => 77,
            Interrupt::CAN0_2 => 78,
            Interrupt::CAN0_3 => 79,
            Interrupt::CAN0_4 => 80,
            Interrupt::CAN0_5 => 81,
            Interrupt::CAN0_6 => 82,
            Interrupt::CAN0_7 => 83,
            Interrupt::USIC0_0 => 84,
            Interrupt::USIC0_1 => 85,
            Interrupt::USIC0_2 => 86,
            Interrupt::USIC0_3 => 87,
            Interrupt::USIC0_4 => 88,
            Interrupt::USIC0_5 => 89,
            Interrupt::USIC1_0 => 90,
            Interrupt::USIC1_1 => 91,
            Interrupt::USIC1_2 => 92,
            Interrupt::USIC1_3 => 93,
            Interrupt::USIC1_4 => 94,
            Interrupt::USIC1_5 => 95,
            Interrupt::LEDTS0_0 => 102,
            Interrupt::FCE0_0 => 104,
            Interrupt::GPDMA0_0 => 105,
            Interrupt::SDMMC0_0 => 106,
            Interrupt::USB0_0 => 107,
            Interrupt::ETH0_0 => 108,
            Interrupt::ECAT0_0 => 109,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Cortex-M4 Private Peripheral Block"]
pub struct PPB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PPB {}
impl PPB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ppb::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for PPB {
    type Target = ppb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PPB::ptr() }
    }
}
#[doc = "Cortex-M4 Private Peripheral Block"]
pub mod ppb;
#[doc = "DMA Line Router"]
pub struct DLR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DLR {}
impl DLR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dlr::RegisterBlock {
        0x5000_4900 as *const _
    }
}
impl Deref for DLR {
    type Target = dlr::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DLR::ptr() }
    }
}
#[doc = "DMA Line Router"]
pub mod dlr;
#[doc = "Event Request Unit 0"]
pub struct ERU0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ERU0 {}
impl ERU0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eru0::RegisterBlock {
        0x5000_4800 as *const _
    }
}
impl Deref for ERU0 {
    type Target = eru0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ERU0::ptr() }
    }
}
#[doc = "Event Request Unit 0"]
pub mod eru0;
#[doc = "Event Request Unit 1"]
pub struct ERU1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ERU1 {}
impl ERU1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eru0::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for ERU1 {
    type Target = eru0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ERU1::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0 {}
impl GPDMA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0::RegisterBlock {
        0x5001_42c0 as *const _
    }
}
impl Deref for GPDMA0 {
    type Target = gpdma0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH0 {}
impl GPDMA0_CH0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch0::RegisterBlock {
        0x5001_4000 as *const _
    }
}
impl Deref for GPDMA0_CH0 {
    type Target = gpdma0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH0::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0_ch0;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH1 {}
impl GPDMA0_CH1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch0::RegisterBlock {
        0x5001_4058 as *const _
    }
}
impl Deref for GPDMA0_CH1 {
    type Target = gpdma0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH1::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH2 {}
impl GPDMA0_CH2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_40b0 as *const _
    }
}
impl Deref for GPDMA0_CH2 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH2::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0_ch2;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH3 {}
impl GPDMA0_CH3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_4108 as *const _
    }
}
impl Deref for GPDMA0_CH3 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH3::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH4 {}
impl GPDMA0_CH4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_4160 as *const _
    }
}
impl Deref for GPDMA0_CH4 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH4::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH5 {}
impl GPDMA0_CH5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_41b8 as *const _
    }
}
impl Deref for GPDMA0_CH5 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH5::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH6 {}
impl GPDMA0_CH6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_4210 as *const _
    }
}
impl Deref for GPDMA0_CH6 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH6::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH7 {}
impl GPDMA0_CH7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_4268 as *const _
    }
}
impl Deref for GPDMA0_CH7 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH7::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub struct FCE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE {}
impl FCE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fce::RegisterBlock {
        0x5002_0000 as *const _
    }
}
impl Deref for FCE {
    type Target = fce::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCE::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub mod fce;
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE0 {}
impl FCE_KE0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fce_ke0::RegisterBlock {
        0x5002_0020 as *const _
    }
}
impl Deref for FCE_KE0 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCE_KE0::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub mod fce_ke0;
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE1 {}
impl FCE_KE1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fce_ke0::RegisterBlock {
        0x5002_0040 as *const _
    }
}
impl Deref for FCE_KE1 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCE_KE1::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE2 {}
impl FCE_KE2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fce_ke0::RegisterBlock {
        0x5002_0060 as *const _
    }
}
impl Deref for FCE_KE2 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCE_KE2::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE3 {}
impl FCE_KE3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fce_ke0::RegisterBlock {
        0x5002_0080 as *const _
    }
}
impl Deref for FCE_KE3 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCE_KE3::ptr() }
    }
}
#[doc = "Peripheral Bridge AHB 0"]
pub struct PBA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PBA0 {}
impl PBA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pba0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for PBA0 {
    type Target = pba0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PBA0::ptr() }
    }
}
#[doc = "Peripheral Bridge AHB 0"]
pub mod pba0;
#[doc = "Peripheral Bridge AHB 1"]
pub struct PBA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PBA1 {}
impl PBA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pba0::RegisterBlock {
        0x4800_0000 as *const _
    }
}
impl Deref for PBA1 {
    type Target = pba0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PBA1::ptr() }
    }
}
#[doc = "Flash Memory Controller"]
pub struct FLASH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH0 {}
impl FLASH0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash0::RegisterBlock {
        0x5800_1000 as *const _
    }
}
impl Deref for FLASH0 {
    type Target = flash0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH0::ptr() }
    }
}
#[doc = "Flash Memory Controller"]
pub mod flash0;
#[doc = "Prefetch Unit"]
pub struct PREF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PREF {}
impl PREF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pref::RegisterBlock {
        0x5800_4000 as *const _
    }
}
impl Deref for PREF {
    type Target = pref::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PREF::ptr() }
    }
}
#[doc = "Prefetch Unit"]
pub mod pref;
#[doc = "Program Management Unit"]
pub struct PMU0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU0 {}
impl PMU0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmu0::RegisterBlock {
        0x5800_0508 as *const _
    }
}
impl Deref for PMU0 {
    type Target = pmu0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMU0::ptr() }
    }
}
#[doc = "Program Management Unit"]
pub mod pmu0;
#[doc = "Watch Dog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watch Dog Timer"]
pub mod wdt;
#[doc = "Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x5000_4a00 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock"]
pub mod rtc;
#[doc = "System Control Unit"]
pub struct SCU_CLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_CLK {}
impl SCU_CLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_clk::RegisterBlock {
        0x5000_4600 as *const _
    }
}
impl Deref for SCU_CLK {
    type Target = scu_clk::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_CLK::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_clk;
#[doc = "System Control Unit"]
pub struct SCU_OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_OSC {}
impl SCU_OSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_osc::RegisterBlock {
        0x5000_4700 as *const _
    }
}
impl Deref for SCU_OSC {
    type Target = scu_osc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_OSC::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_osc;
#[doc = "System Control Unit"]
pub struct SCU_PLL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_PLL {}
impl SCU_PLL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_pll::RegisterBlock {
        0x5000_4710 as *const _
    }
}
impl Deref for SCU_PLL {
    type Target = scu_pll::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_PLL::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_pll;
#[doc = "System Control Unit"]
pub struct SCU_GENERAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_GENERAL {}
impl SCU_GENERAL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_general::RegisterBlock {
        0x5000_4000 as *const _
    }
}
impl Deref for SCU_GENERAL {
    type Target = scu_general::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_GENERAL::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_general;
#[doc = "System Control Unit"]
pub struct SCU_INTERRUPT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_INTERRUPT {}
impl SCU_INTERRUPT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_interrupt::RegisterBlock {
        0x5000_4074 as *const _
    }
}
impl Deref for SCU_INTERRUPT {
    type Target = scu_interrupt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_INTERRUPT::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_interrupt;
#[doc = "System Control Unit"]
pub struct SCU_PARITY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_PARITY {}
impl SCU_PARITY {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_parity::RegisterBlock {
        0x5000_413c as *const _
    }
}
impl Deref for SCU_PARITY {
    type Target = scu_parity::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_PARITY::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_parity;
#[doc = "System Control Unit"]
pub struct SCU_TRAP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_TRAP {}
impl SCU_TRAP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_trap::RegisterBlock {
        0x5000_4160 as *const _
    }
}
impl Deref for SCU_TRAP {
    type Target = scu_trap::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_TRAP::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_trap;
#[doc = "System Control Unit"]
pub struct SCU_HIBERNATE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_HIBERNATE {}
impl SCU_HIBERNATE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_hibernate::RegisterBlock {
        0x5000_4300 as *const _
    }
}
impl Deref for SCU_HIBERNATE {
    type Target = scu_hibernate::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_HIBERNATE::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_hibernate;
#[doc = "System Control Unit"]
pub struct SCU_POWER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_POWER {}
impl SCU_POWER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_power::RegisterBlock {
        0x5000_4200 as *const _
    }
}
impl Deref for SCU_POWER {
    type Target = scu_power::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_POWER::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_power;
#[doc = "System Control Unit"]
pub struct SCU_RESET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_RESET {}
impl SCU_RESET {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_reset::RegisterBlock {
        0x5000_4400 as *const _
    }
}
impl Deref for SCU_RESET {
    type Target = scu_reset::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_RESET::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_reset;
#[doc = "LED and Touch Sense Unit 0"]
pub struct LEDTS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEDTS0 {}
impl LEDTS0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ledts0::RegisterBlock {
        0x4801_0000 as *const _
    }
}
impl Deref for LEDTS0 {
    type Target = ledts0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LEDTS0::ptr() }
    }
}
#[doc = "LED and Touch Sense Unit 0"]
pub mod ledts0;
#[doc = "SD and Multimediacard Control Register"]
pub struct SDMMC_CON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC_CON {}
impl SDMMC_CON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc_con::RegisterBlock {
        0x5000_40b4 as *const _
    }
}
impl Deref for SDMMC_CON {
    type Target = sdmmc_con::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC_CON::ptr() }
    }
}
#[doc = "SD and Multimediacard Control Register"]
pub mod sdmmc_con;
#[doc = "SD and Multimediacard Interface"]
pub struct SDMMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC {}
impl SDMMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc::RegisterBlock {
        0x4801_c000 as *const _
    }
}
impl Deref for SDMMC {
    type Target = sdmmc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC::ptr() }
    }
}
#[doc = "SD and Multimediacard Interface"]
pub mod sdmmc;
#[doc = "Ethernet Control Register"]
pub struct ETH0_CON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETH0_CON {}
impl ETH0_CON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eth0_con::RegisterBlock {
        0x5000_4040 as *const _
    }
}
impl Deref for ETH0_CON {
    type Target = eth0_con::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETH0_CON::ptr() }
    }
}
#[doc = "Ethernet Control Register"]
pub mod eth0_con;
#[doc = "Ethernet Unit 0"]
pub struct ETH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETH0 {}
impl ETH0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eth0::RegisterBlock {
        0x5000_c000 as *const _
    }
}
impl Deref for ETH0 {
    type Target = eth0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETH0::ptr() }
    }
}
#[doc = "Ethernet Unit 0"]
pub mod eth0;
// #[doc = "EtherCAT 0 Control Register"]
// pub struct ECAT0_CON {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_CON {}
// impl ECAT0_CON {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_con::RegisterBlock {
//         0x5000_41b0 as *const _
//     }
// }
// impl Deref for ECAT0_CON {
//     type Target = ecat0_con::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_CON::ptr() }
//     }
// }
// #[doc = "EtherCAT 0 Control Register"]
// pub mod ecat0_con;
// #[doc = "EtherCAT 0"]
// pub struct ECAT0 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0 {}
// impl ECAT0 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0::RegisterBlock {
//         0x5401_0000 as *const _
//     }
// }
// impl Deref for ECAT0 {
//     type Target = ecat0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub mod ecat0;
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_FMMU0 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_FMMU0 {}
// impl ECAT0_FMMU0 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_fmmu0::RegisterBlock {
//         0x5401_0600 as *const _
//     }
// }
// impl Deref for ECAT0_FMMU0 {
//     type Target = ecat0_fmmu0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_FMMU0::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub mod ecat0_fmmu0;
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_FMMU1 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_FMMU1 {}
// impl ECAT0_FMMU1 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_fmmu0::RegisterBlock {
//         0x5401_0610 as *const _
//     }
// }
// impl Deref for ECAT0_FMMU1 {
//     type Target = ecat0_fmmu0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_FMMU1::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_FMMU2 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_FMMU2 {}
// impl ECAT0_FMMU2 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_fmmu0::RegisterBlock {
//         0x5401_0620 as *const _
//     }
// }
// impl Deref for ECAT0_FMMU2 {
//     type Target = ecat0_fmmu0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_FMMU2::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_FMMU3 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_FMMU3 {}
// impl ECAT0_FMMU3 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_fmmu0::RegisterBlock {
//         0x5401_0630 as *const _
//     }
// }
// impl Deref for ECAT0_FMMU3 {
//     type Target = ecat0_fmmu0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_FMMU3::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_FMMU4 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_FMMU4 {}
// impl ECAT0_FMMU4 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_fmmu0::RegisterBlock {
//         0x5401_0640 as *const _
//     }
// }
// impl Deref for ECAT0_FMMU4 {
//     type Target = ecat0_fmmu0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_FMMU4::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_FMMU5 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_FMMU5 {}
// impl ECAT0_FMMU5 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_fmmu0::RegisterBlock {
//         0x5401_0650 as *const _
//     }
// }
// impl Deref for ECAT0_FMMU5 {
//     type Target = ecat0_fmmu0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_FMMU5::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_FMMU6 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_FMMU6 {}
// impl ECAT0_FMMU6 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_fmmu0::RegisterBlock {
//         0x5401_0660 as *const _
//     }
// }
// impl Deref for ECAT0_FMMU6 {
//     type Target = ecat0_fmmu0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_FMMU6::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_FMMU7 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_FMMU7 {}
// impl ECAT0_FMMU7 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_fmmu0::RegisterBlock {
//         0x5401_0670 as *const _
//     }
// }
// impl Deref for ECAT0_FMMU7 {
//     type Target = ecat0_fmmu0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_FMMU7::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_SM0 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_SM0 {}
// impl ECAT0_SM0 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_sm0::RegisterBlock {
//         0x5401_0800 as *const _
//     }
// }
// impl Deref for ECAT0_SM0 {
//     type Target = ecat0_sm0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_SM0::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub mod ecat0_sm0;
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_SM1 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_SM1 {}
// impl ECAT0_SM1 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_sm0::RegisterBlock {
//         0x5401_0808 as *const _
//     }
// }
// impl Deref for ECAT0_SM1 {
//     type Target = ecat0_sm0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_SM1::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_SM2 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_SM2 {}
// impl ECAT0_SM2 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_sm0::RegisterBlock {
//         0x5401_0810 as *const _
//     }
// }
// impl Deref for ECAT0_SM2 {
//     type Target = ecat0_sm0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_SM2::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_SM3 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_SM3 {}
// impl ECAT0_SM3 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_sm0::RegisterBlock {
//         0x5401_0818 as *const _
//     }
// }
// impl Deref for ECAT0_SM3 {
//     type Target = ecat0_sm0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_SM3::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_SM4 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_SM4 {}
// impl ECAT0_SM4 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_sm0::RegisterBlock {
//         0x5401_0820 as *const _
//     }
// }
// impl Deref for ECAT0_SM4 {
//     type Target = ecat0_sm0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_SM4::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_SM5 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_SM5 {}
// impl ECAT0_SM5 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_sm0::RegisterBlock {
//         0x5401_0828 as *const _
//     }
// }
// impl Deref for ECAT0_SM5 {
//     type Target = ecat0_sm0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_SM5::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_SM6 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_SM6 {}
// impl ECAT0_SM6 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_sm0::RegisterBlock {
//         0x5401_0830 as *const _
//     }
// }
// impl Deref for ECAT0_SM6 {
//     type Target = ecat0_sm0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_SM6::ptr() }
//     }
// }
// #[doc = "EtherCAT 0"]
// pub struct ECAT0_SM7 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for ECAT0_SM7 {}
// impl ECAT0_SM7 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const ecat0_sm0::RegisterBlock {
//         0x5401_0838 as *const _
//     }
// }
// impl Deref for ECAT0_SM7 {
//     type Target = ecat0_sm0::RegisterBlock;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*ECAT0_SM7::ptr() }
//     }
// }
#[doc = "Universal Serial Bus"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0::RegisterBlock {
        0x5004_0000 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb0;
#[doc = "Universal Serial Bus"]
pub struct USB0_EP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP0 {}
impl USB0_EP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep0::RegisterBlock {
        0x5004_0900 as *const _
    }
}
impl Deref for USB0_EP0 {
    type Target = usb0_ep0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP0::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb0_ep0;
#[doc = "Universal Serial Bus"]
pub struct USB0_EP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP1 {}
impl USB0_EP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_0920 as *const _
    }
}
impl Deref for USB0_EP1 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP1::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb0_ep1;
#[doc = "Universal Serial Bus"]
pub struct USB0_EP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP2 {}
impl USB0_EP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_0940 as *const _
    }
}
impl Deref for USB0_EP2 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP2::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP3 {}
impl USB0_EP3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_0960 as *const _
    }
}
impl Deref for USB0_EP3 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP3::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP4 {}
impl USB0_EP4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_0980 as *const _
    }
}
impl Deref for USB0_EP4 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP4::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP5 {}
impl USB0_EP5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_09a0 as *const _
    }
}
impl Deref for USB0_EP5 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP5::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP6 {}
impl USB0_EP6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_09c0 as *const _
    }
}
impl Deref for USB0_EP6 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP6::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH0 {}
impl USB0_CH0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_0500 as *const _
    }
}
impl Deref for USB0_CH0 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH0::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb0_ch0;
#[doc = "Universal Serial Bus"]
pub struct USB0_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH1 {}
impl USB0_CH1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_0520 as *const _
    }
}
impl Deref for USB0_CH1 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH1::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH2 {}
impl USB0_CH2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_0540 as *const _
    }
}
impl Deref for USB0_CH2 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH2::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH3 {}
impl USB0_CH3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_0560 as *const _
    }
}
impl Deref for USB0_CH3 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH3::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH4 {}
impl USB0_CH4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_0580 as *const _
    }
}
impl Deref for USB0_CH4 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH4::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH5 {}
impl USB0_CH5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_05a0 as *const _
    }
}
impl Deref for USB0_CH5 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH5::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH6 {}
impl USB0_CH6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_05c0 as *const _
    }
}
impl Deref for USB0_CH6 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH6::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH7 {}
impl USB0_CH7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_05e0 as *const _
    }
}
impl Deref for USB0_CH7 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH7::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH8 {}
impl USB0_CH8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_0600 as *const _
    }
}
impl Deref for USB0_CH8 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH8::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH9 {}
impl USB0_CH9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_0620 as *const _
    }
}
impl Deref for USB0_CH9 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH9::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH10 {}
impl USB0_CH10 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_0640 as *const _
    }
}
impl Deref for USB0_CH10 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH10::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH11 {}
impl USB0_CH11 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_0660 as *const _
    }
}
impl Deref for USB0_CH11 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH11::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH12 {}
impl USB0_CH12 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_0680 as *const _
    }
}
impl Deref for USB0_CH12 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH12::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH13 {}
impl USB0_CH13 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ch0::RegisterBlock {
        0x5004_06a0 as *const _
    }
}
impl Deref for USB0_CH13 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_CH13::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0 {}
impl USIC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0::RegisterBlock {
        0x4003_0008 as *const _
    }
}
impl Deref for USIC0 {
    type Target = usic0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC0::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub mod usic0;
#[doc = "Universal Serial Interface Controller 1"]
pub struct USIC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC1 {}
impl USIC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0::RegisterBlock {
        0x4802_0008 as *const _
    }
}
impl Deref for USIC1 {
    type Target = usic0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC1::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0_CH0 {}
impl USIC0_CH0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0_ch0::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for USIC0_CH0 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC0_CH0::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub mod usic0_ch0;
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0_CH1 {}
impl USIC0_CH1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0_ch0::RegisterBlock {
        0x4003_0200 as *const _
    }
}
impl Deref for USIC0_CH1 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC0_CH1::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC1_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC1_CH0 {}
impl USIC1_CH0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0_ch0::RegisterBlock {
        0x4802_0000 as *const _
    }
}
impl Deref for USIC1_CH0 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC1_CH0::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC1_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC1_CH1 {}
impl USIC1_CH1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0_ch0::RegisterBlock {
        0x4802_0200 as *const _
    }
}
impl Deref for USIC1_CH1 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC1_CH1::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN {}
impl CAN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can::RegisterBlock {
        0x4801_4000 as *const _
    }
}
impl Deref for CAN {
    type Target = can::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub mod can;
#[doc = "Controller Area Networks"]
pub struct CAN_NODE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_NODE0 {}
impl CAN_NODE0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_node0::RegisterBlock {
        0x4801_4200 as *const _
    }
}
impl Deref for CAN_NODE0 {
    type Target = can_node0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_NODE0::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub mod can_node0;
#[doc = "Controller Area Networks"]
pub struct CAN_NODE1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_NODE1 {}
impl CAN_NODE1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_node0::RegisterBlock {
        0x4801_4300 as *const _
    }
}
impl Deref for CAN_NODE1 {
    type Target = can_node0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_NODE1::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO {}
impl CAN_MO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo::RegisterBlock {
        0x4801_5000 as *const _
    }
}
impl Deref for CAN_MO {
    type Target = can_mo::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub mod can_mo;
#[doc = "Analog to Digital Converter"]
pub struct VADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC {}
impl VADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vadc::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for VADC {
    type Target = vadc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*VADC::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod vadc;
#[doc = "Analog to Digital Converter"]
pub struct VADC_G0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC_G0 {}
impl VADC_G0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vadc_g0::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for VADC_G0 {
    type Target = vadc_g0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*VADC_G0::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod vadc_g0;
#[doc = "Analog to Digital Converter"]
pub struct VADC_G1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC_G1 {}
impl VADC_G1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vadc_g0::RegisterBlock {
        0x4000_4800 as *const _
    }
}
impl Deref for VADC_G1 {
    type Target = vadc_g0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*VADC_G1::ptr() }
    }
}
#[doc = "Digital to Analog Converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4801_8000 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital to Analog Converter"]
pub mod dac;
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40 {}
impl CCU40 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for CCU40 {
    type Target = ccu40::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub mod ccu40;
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41 {}
impl CCU41 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for CCU41 {
    type Target = ccu40::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC40 {}
impl CCU40_CC40 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4000_c100 as *const _
    }
}
impl Deref for CCU40_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU40_CC40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub mod ccu40_cc40;
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC41 {}
impl CCU40_CC41 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4000_c200 as *const _
    }
}
impl Deref for CCU40_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU40_CC41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC42 {}
impl CCU40_CC42 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4000_c300 as *const _
    }
}
impl Deref for CCU40_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU40_CC42::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC43 {}
impl CCU40_CC43 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4000_c400 as *const _
    }
}
impl Deref for CCU40_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU40_CC43::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC40 {}
impl CCU41_CC40 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4001_0100 as *const _
    }
}
impl Deref for CCU41_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU41_CC40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC41 {}
impl CCU41_CC41 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4001_0200 as *const _
    }
}
impl Deref for CCU41_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU41_CC41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC42 {}
impl CCU41_CC42 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4001_0300 as *const _
    }
}
impl Deref for CCU41_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU41_CC42::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC43 {}
impl CCU41_CC43 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4001_0400 as *const _
    }
}
impl Deref for CCU41_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU41_CC43::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80 {}
impl CCU80 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu80::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for CCU80 {
    type Target = ccu80::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU80::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub mod ccu80;
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC80 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC80 {}
impl CCU80_CC80 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu80_cc80::RegisterBlock {
        0x4002_0100 as *const _
    }
}
impl Deref for CCU80_CC80 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU80_CC80::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub mod ccu80_cc80;
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC81 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC81 {}
impl CCU80_CC81 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu80_cc80::RegisterBlock {
        0x4002_0200 as *const _
    }
}
impl Deref for CCU80_CC81 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU80_CC81::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC82 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC82 {}
impl CCU80_CC82 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu80_cc80::RegisterBlock {
        0x4002_0300 as *const _
    }
}
impl Deref for CCU80_CC82 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU80_CC82::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC83 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC83 {}
impl CCU80_CC83 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu80_cc80::RegisterBlock {
        0x4002_0400 as *const _
    }
}
impl Deref for CCU80_CC83 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU80_CC83::ptr() }
    }
}
#[doc = "Port 0"]
pub struct PORT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT0 {}
impl PORT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port0::RegisterBlock {
        0x4802_8000 as *const _
    }
}
impl Deref for PORT0 {
    type Target = port0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT0::ptr() }
    }
}
#[doc = "Port 0"]
pub mod port0;
#[doc = "Port 1"]
pub struct PORT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT1 {}
impl PORT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port1::RegisterBlock {
        0x4802_8100 as *const _
    }
}
impl Deref for PORT1 {
    type Target = port1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT1::ptr() }
    }
}
#[doc = "Port 1"]
pub mod port1;
#[doc = "Port 2"]
pub struct PORT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT2 {}
impl PORT2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port2::RegisterBlock {
        0x4802_8200 as *const _
    }
}
impl Deref for PORT2 {
    type Target = port2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT2::ptr() }
    }
}
#[doc = "Port 2"]
pub mod port2;
#[doc = "Port 3"]
pub struct PORT3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT3 {}
impl PORT3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port3::RegisterBlock {
        0x4802_8300 as *const _
    }
}
impl Deref for PORT3 {
    type Target = port3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT3::ptr() }
    }
}
#[doc = "Port 3"]
pub mod port3;
#[doc = "Port 4"]
pub struct PORT4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT4 {}
impl PORT4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port4::RegisterBlock {
        0x4802_8400 as *const _
    }
}
impl Deref for PORT4 {
    type Target = port4::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT4::ptr() }
    }
}
#[doc = "Port 4"]
pub mod port4;
#[doc = "Port 5"]
pub struct PORT5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT5 {}
impl PORT5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port5::RegisterBlock {
        0x4802_8500 as *const _
    }
}
impl Deref for PORT5 {
    type Target = port5::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT5::ptr() }
    }
}
#[doc = "Port 5"]
pub mod port5;
#[doc = "Port 14"]
pub struct PORT14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT14 {}
impl PORT14 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port14::RegisterBlock {
        0x4802_8e00 as *const _
    }
}
impl Deref for PORT14 {
    type Target = port14::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT14::ptr() }
    }
}
#[doc = "Port 14"]
pub mod port14;
#[doc = "Port 15"]
pub struct PORT15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT15 {}
impl PORT15 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port15::RegisterBlock {
        0x4802_8f00 as *const _
    }
}
impl Deref for PORT15 {
    type Target = port15::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT15::ptr() }
    }
}
#[doc = "Port 15"]
pub mod port15;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PPB"]
    pub PPB: PPB,
    #[doc = "DLR"]
    pub DLR: DLR,
    #[doc = "ERU0"]
    pub ERU0: ERU0,
    #[doc = "ERU1"]
    pub ERU1: ERU1,
    #[doc = "GPDMA0"]
    pub GPDMA0: GPDMA0,
    #[doc = "GPDMA0_CH0"]
    pub GPDMA0_CH0: GPDMA0_CH0,
    #[doc = "GPDMA0_CH1"]
    pub GPDMA0_CH1: GPDMA0_CH1,
    #[doc = "GPDMA0_CH2"]
    pub GPDMA0_CH2: GPDMA0_CH2,
    #[doc = "GPDMA0_CH3"]
    pub GPDMA0_CH3: GPDMA0_CH3,
    #[doc = "GPDMA0_CH4"]
    pub GPDMA0_CH4: GPDMA0_CH4,
    #[doc = "GPDMA0_CH5"]
    pub GPDMA0_CH5: GPDMA0_CH5,
    #[doc = "GPDMA0_CH6"]
    pub GPDMA0_CH6: GPDMA0_CH6,
    #[doc = "GPDMA0_CH7"]
    pub GPDMA0_CH7: GPDMA0_CH7,
    #[doc = "FCE"]
    pub FCE: FCE,
    #[doc = "FCE_KE0"]
    pub FCE_KE0: FCE_KE0,
    #[doc = "FCE_KE1"]
    pub FCE_KE1: FCE_KE1,
    #[doc = "FCE_KE2"]
    pub FCE_KE2: FCE_KE2,
    #[doc = "FCE_KE3"]
    pub FCE_KE3: FCE_KE3,
    #[doc = "PBA0"]
    pub PBA0: PBA0,
    #[doc = "PBA1"]
    pub PBA1: PBA1,
    #[doc = "FLASH0"]
    pub FLASH0: FLASH0,
    #[doc = "PREF"]
    pub PREF: PREF,
    #[doc = "PMU0"]
    pub PMU0: PMU0,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SCU_CLK"]
    pub SCU_CLK: SCU_CLK,
    #[doc = "SCU_OSC"]
    pub SCU_OSC: SCU_OSC,
    #[doc = "SCU_PLL"]
    pub SCU_PLL: SCU_PLL,
    #[doc = "SCU_GENERAL"]
    pub SCU_GENERAL: SCU_GENERAL,
    #[doc = "SCU_INTERRUPT"]
    pub SCU_INTERRUPT: SCU_INTERRUPT,
    #[doc = "SCU_PARITY"]
    pub SCU_PARITY: SCU_PARITY,
    #[doc = "SCU_TRAP"]
    pub SCU_TRAP: SCU_TRAP,
    #[doc = "SCU_HIBERNATE"]
    pub SCU_HIBERNATE: SCU_HIBERNATE,
    #[doc = "SCU_POWER"]
    pub SCU_POWER: SCU_POWER,
    #[doc = "SCU_RESET"]
    pub SCU_RESET: SCU_RESET,
    #[doc = "LEDTS0"]
    pub LEDTS0: LEDTS0,
    #[doc = "SDMMC_CON"]
    pub SDMMC_CON: SDMMC_CON,
    #[doc = "SDMMC"]
    pub SDMMC: SDMMC,
    #[doc = "ETH0_CON"]
    pub ETH0_CON: ETH0_CON,
    #[doc = "ETH0"]
    pub ETH0: ETH0,
    // #[doc = "ECAT0_CON"]
    // pub ECAT0_CON: ECAT0_CON,
    // #[doc = "ECAT0"]
    // pub ECAT0: ECAT0,
    // #[doc = "ECAT0_FMMU0"]
    // pub ECAT0_FMMU0: ECAT0_FMMU0,
    // #[doc = "ECAT0_FMMU1"]
    // pub ECAT0_FMMU1: ECAT0_FMMU1,
    // #[doc = "ECAT0_FMMU2"]
    // pub ECAT0_FMMU2: ECAT0_FMMU2,
    // #[doc = "ECAT0_FMMU3"]
    // pub ECAT0_FMMU3: ECAT0_FMMU3,
    // #[doc = "ECAT0_FMMU4"]
    // pub ECAT0_FMMU4: ECAT0_FMMU4,
    // #[doc = "ECAT0_FMMU5"]
    // pub ECAT0_FMMU5: ECAT0_FMMU5,
    // #[doc = "ECAT0_FMMU6"]
    // pub ECAT0_FMMU6: ECAT0_FMMU6,
    // #[doc = "ECAT0_FMMU7"]
    // pub ECAT0_FMMU7: ECAT0_FMMU7,
    // #[doc = "ECAT0_SM0"]
    // pub ECAT0_SM0: ECAT0_SM0,
    // #[doc = "ECAT0_SM1"]
    // pub ECAT0_SM1: ECAT0_SM1,
    // #[doc = "ECAT0_SM2"]
    // pub ECAT0_SM2: ECAT0_SM2,
    // #[doc = "ECAT0_SM3"]
    // pub ECAT0_SM3: ECAT0_SM3,
    // #[doc = "ECAT0_SM4"]
    // pub ECAT0_SM4: ECAT0_SM4,
    // #[doc = "ECAT0_SM5"]
    // pub ECAT0_SM5: ECAT0_SM5,
    // #[doc = "ECAT0_SM6"]
    // pub ECAT0_SM6: ECAT0_SM6,
    // #[doc = "ECAT0_SM7"]
    // pub ECAT0_SM7: ECAT0_SM7,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "USB0_EP0"]
    pub USB0_EP0: USB0_EP0,
    #[doc = "USB0_EP1"]
    pub USB0_EP1: USB0_EP1,
    #[doc = "USB0_EP2"]
    pub USB0_EP2: USB0_EP2,
    #[doc = "USB0_EP3"]
    pub USB0_EP3: USB0_EP3,
    #[doc = "USB0_EP4"]
    pub USB0_EP4: USB0_EP4,
    #[doc = "USB0_EP5"]
    pub USB0_EP5: USB0_EP5,
    #[doc = "USB0_EP6"]
    pub USB0_EP6: USB0_EP6,
    #[doc = "USB0_CH0"]
    pub USB0_CH0: USB0_CH0,
    #[doc = "USB0_CH1"]
    pub USB0_CH1: USB0_CH1,
    #[doc = "USB0_CH2"]
    pub USB0_CH2: USB0_CH2,
    #[doc = "USB0_CH3"]
    pub USB0_CH3: USB0_CH3,
    #[doc = "USB0_CH4"]
    pub USB0_CH4: USB0_CH4,
    #[doc = "USB0_CH5"]
    pub USB0_CH5: USB0_CH5,
    #[doc = "USB0_CH6"]
    pub USB0_CH6: USB0_CH6,
    #[doc = "USB0_CH7"]
    pub USB0_CH7: USB0_CH7,
    #[doc = "USB0_CH8"]
    pub USB0_CH8: USB0_CH8,
    #[doc = "USB0_CH9"]
    pub USB0_CH9: USB0_CH9,
    #[doc = "USB0_CH10"]
    pub USB0_CH10: USB0_CH10,
    #[doc = "USB0_CH11"]
    pub USB0_CH11: USB0_CH11,
    #[doc = "USB0_CH12"]
    pub USB0_CH12: USB0_CH12,
    #[doc = "USB0_CH13"]
    pub USB0_CH13: USB0_CH13,
    #[doc = "USIC0"]
    pub USIC0: USIC0,
    #[doc = "USIC1"]
    pub USIC1: USIC1,
    #[doc = "USIC0_CH0"]
    pub USIC0_CH0: USIC0_CH0,
    #[doc = "USIC0_CH1"]
    pub USIC0_CH1: USIC0_CH1,
    #[doc = "USIC1_CH0"]
    pub USIC1_CH0: USIC1_CH0,
    #[doc = "USIC1_CH1"]
    pub USIC1_CH1: USIC1_CH1,
    #[doc = "CAN"]
    pub CAN: CAN,
    #[doc = "CAN_NODE0"]
    pub CAN_NODE0: CAN_NODE0,
    #[doc = "CAN_NODE1"]
    pub CAN_NODE1: CAN_NODE1,
    #[doc = "CAN_MO"]
    pub CAN_MO: CAN_MO,
    #[doc = "VADC"]
    pub VADC: VADC,
    #[doc = "VADC_G0"]
    pub VADC_G0: VADC_G0,
    #[doc = "VADC_G1"]
    pub VADC_G1: VADC_G1,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "CCU40"]
    pub CCU40: CCU40,
    #[doc = "CCU41"]
    pub CCU41: CCU41,
    #[doc = "CCU40_CC40"]
    pub CCU40_CC40: CCU40_CC40,
    #[doc = "CCU40_CC41"]
    pub CCU40_CC41: CCU40_CC41,
    #[doc = "CCU40_CC42"]
    pub CCU40_CC42: CCU40_CC42,
    #[doc = "CCU40_CC43"]
    pub CCU40_CC43: CCU40_CC43,
    #[doc = "CCU41_CC40"]
    pub CCU41_CC40: CCU41_CC40,
    #[doc = "CCU41_CC41"]
    pub CCU41_CC41: CCU41_CC41,
    #[doc = "CCU41_CC42"]
    pub CCU41_CC42: CCU41_CC42,
    #[doc = "CCU41_CC43"]
    pub CCU41_CC43: CCU41_CC43,
    #[doc = "CCU80"]
    pub CCU80: CCU80,
    #[doc = "CCU80_CC80"]
    pub CCU80_CC80: CCU80_CC80,
    #[doc = "CCU80_CC81"]
    pub CCU80_CC81: CCU80_CC81,
    #[doc = "CCU80_CC82"]
    pub CCU80_CC82: CCU80_CC82,
    #[doc = "CCU80_CC83"]
    pub CCU80_CC83: CCU80_CC83,
    #[doc = "PORT0"]
    pub PORT0: PORT0,
    #[doc = "PORT1"]
    pub PORT1: PORT1,
    #[doc = "PORT2"]
    pub PORT2: PORT2,
    #[doc = "PORT3"]
    pub PORT3: PORT3,
    #[doc = "PORT4"]
    pub PORT4: PORT4,
    #[doc = "PORT5"]
    pub PORT5: PORT5,
    #[doc = "PORT14"]
    pub PORT14: PORT14,
    #[doc = "PORT15"]
    pub PORT15: PORT15,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| if unsafe { DEVICE_PERIPHERALS } { None } else { Some(unsafe { Peripherals::steal() }) })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PPB: PPB { _marker: PhantomData },
            DLR: DLR { _marker: PhantomData },
            ERU0: ERU0 { _marker: PhantomData },
            ERU1: ERU1 { _marker: PhantomData },
            GPDMA0: GPDMA0 { _marker: PhantomData },
            GPDMA0_CH0: GPDMA0_CH0 { _marker: PhantomData },
            GPDMA0_CH1: GPDMA0_CH1 { _marker: PhantomData },
            GPDMA0_CH2: GPDMA0_CH2 { _marker: PhantomData },
            GPDMA0_CH3: GPDMA0_CH3 { _marker: PhantomData },
            GPDMA0_CH4: GPDMA0_CH4 { _marker: PhantomData },
            GPDMA0_CH5: GPDMA0_CH5 { _marker: PhantomData },
            GPDMA0_CH6: GPDMA0_CH6 { _marker: PhantomData },
            GPDMA0_CH7: GPDMA0_CH7 { _marker: PhantomData },
            FCE: FCE { _marker: PhantomData },
            FCE_KE0: FCE_KE0 { _marker: PhantomData },
            FCE_KE1: FCE_KE1 { _marker: PhantomData },
            FCE_KE2: FCE_KE2 { _marker: PhantomData },
            FCE_KE3: FCE_KE3 { _marker: PhantomData },
            PBA0: PBA0 { _marker: PhantomData },
            PBA1: PBA1 { _marker: PhantomData },
            FLASH0: FLASH0 { _marker: PhantomData },
            PREF: PREF { _marker: PhantomData },
            PMU0: PMU0 { _marker: PhantomData },
            WDT: WDT { _marker: PhantomData },
            RTC: RTC { _marker: PhantomData },
            SCU_CLK: SCU_CLK { _marker: PhantomData },
            SCU_OSC: SCU_OSC { _marker: PhantomData },
            SCU_PLL: SCU_PLL { _marker: PhantomData },
            SCU_GENERAL: SCU_GENERAL { _marker: PhantomData },
            SCU_INTERRUPT: SCU_INTERRUPT { _marker: PhantomData },
            SCU_PARITY: SCU_PARITY { _marker: PhantomData },
            SCU_TRAP: SCU_TRAP { _marker: PhantomData },
            SCU_HIBERNATE: SCU_HIBERNATE { _marker: PhantomData },
            SCU_POWER: SCU_POWER { _marker: PhantomData },
            SCU_RESET: SCU_RESET { _marker: PhantomData },
            LEDTS0: LEDTS0 { _marker: PhantomData },
            SDMMC_CON: SDMMC_CON { _marker: PhantomData },
            SDMMC: SDMMC { _marker: PhantomData },
            ETH0_CON: ETH0_CON { _marker: PhantomData },
            ETH0: ETH0 { _marker: PhantomData },
            // ECAT0_CON: ECAT0_CON { _marker: PhantomData },
            // ECAT0: ECAT0 { _marker: PhantomData },
            // ECAT0_FMMU0: ECAT0_FMMU0 { _marker: PhantomData },
            // ECAT0_FMMU1: ECAT0_FMMU1 { _marker: PhantomData },
            // ECAT0_FMMU2: ECAT0_FMMU2 { _marker: PhantomData },
            // ECAT0_FMMU3: ECAT0_FMMU3 { _marker: PhantomData },
            // ECAT0_FMMU4: ECAT0_FMMU4 { _marker: PhantomData },
            // ECAT0_FMMU5: ECAT0_FMMU5 { _marker: PhantomData },
            // ECAT0_FMMU6: ECAT0_FMMU6 { _marker: PhantomData },
            // ECAT0_FMMU7: ECAT0_FMMU7 { _marker: PhantomData },
            // ECAT0_SM0: ECAT0_SM0 { _marker: PhantomData },
            // ECAT0_SM1: ECAT0_SM1 { _marker: PhantomData },
            // ECAT0_SM2: ECAT0_SM2 { _marker: PhantomData },
            // ECAT0_SM3: ECAT0_SM3 { _marker: PhantomData },
            // ECAT0_SM4: ECAT0_SM4 { _marker: PhantomData },
            // ECAT0_SM5: ECAT0_SM5 { _marker: PhantomData },
            // ECAT0_SM6: ECAT0_SM6 { _marker: PhantomData },
            // ECAT0_SM7: ECAT0_SM7 { _marker: PhantomData },
            USB0: USB0 { _marker: PhantomData },
            USB0_EP0: USB0_EP0 { _marker: PhantomData },
            USB0_EP1: USB0_EP1 { _marker: PhantomData },
            USB0_EP2: USB0_EP2 { _marker: PhantomData },
            USB0_EP3: USB0_EP3 { _marker: PhantomData },
            USB0_EP4: USB0_EP4 { _marker: PhantomData },
            USB0_EP5: USB0_EP5 { _marker: PhantomData },
            USB0_EP6: USB0_EP6 { _marker: PhantomData },
            USB0_CH0: USB0_CH0 { _marker: PhantomData },
            USB0_CH1: USB0_CH1 { _marker: PhantomData },
            USB0_CH2: USB0_CH2 { _marker: PhantomData },
            USB0_CH3: USB0_CH3 { _marker: PhantomData },
            USB0_CH4: USB0_CH4 { _marker: PhantomData },
            USB0_CH5: USB0_CH5 { _marker: PhantomData },
            USB0_CH6: USB0_CH6 { _marker: PhantomData },
            USB0_CH7: USB0_CH7 { _marker: PhantomData },
            USB0_CH8: USB0_CH8 { _marker: PhantomData },
            USB0_CH9: USB0_CH9 { _marker: PhantomData },
            USB0_CH10: USB0_CH10 { _marker: PhantomData },
            USB0_CH11: USB0_CH11 { _marker: PhantomData },
            USB0_CH12: USB0_CH12 { _marker: PhantomData },
            USB0_CH13: USB0_CH13 { _marker: PhantomData },
            USIC0: USIC0 { _marker: PhantomData },
            USIC1: USIC1 { _marker: PhantomData },
            USIC0_CH0: USIC0_CH0 { _marker: PhantomData },
            USIC0_CH1: USIC0_CH1 { _marker: PhantomData },
            USIC1_CH0: USIC1_CH0 { _marker: PhantomData },
            USIC1_CH1: USIC1_CH1 { _marker: PhantomData },
            CAN: CAN { _marker: PhantomData },
            CAN_NODE0: CAN_NODE0 { _marker: PhantomData },
            CAN_NODE1: CAN_NODE1 { _marker: PhantomData },
            CAN_MO: CAN_MO { _marker: PhantomData },
            VADC: VADC { _marker: PhantomData },
            VADC_G0: VADC_G0 { _marker: PhantomData },
            VADC_G1: VADC_G1 { _marker: PhantomData },
            DAC: DAC { _marker: PhantomData },
            CCU40: CCU40 { _marker: PhantomData },
            CCU41: CCU41 { _marker: PhantomData },
            CCU40_CC40: CCU40_CC40 { _marker: PhantomData },
            CCU40_CC41: CCU40_CC41 { _marker: PhantomData },
            CCU40_CC42: CCU40_CC42 { _marker: PhantomData },
            CCU40_CC43: CCU40_CC43 { _marker: PhantomData },
            CCU41_CC40: CCU41_CC40 { _marker: PhantomData },
            CCU41_CC41: CCU41_CC41 { _marker: PhantomData },
            CCU41_CC42: CCU41_CC42 { _marker: PhantomData },
            CCU41_CC43: CCU41_CC43 { _marker: PhantomData },
            CCU80: CCU80 { _marker: PhantomData },
            CCU80_CC80: CCU80_CC80 { _marker: PhantomData },
            CCU80_CC81: CCU80_CC81 { _marker: PhantomData },
            CCU80_CC82: CCU80_CC82 { _marker: PhantomData },
            CCU80_CC83: CCU80_CC83 { _marker: PhantomData },
            PORT0: PORT0 { _marker: PhantomData },
            PORT1: PORT1 { _marker: PhantomData },
            PORT2: PORT2 { _marker: PhantomData },
            PORT3: PORT3 { _marker: PhantomData },
            PORT4: PORT4 { _marker: PhantomData },
            PORT5: PORT5 { _marker: PhantomData },
            PORT14: PORT14 { _marker: PhantomData },
            PORT15: PORT15 { _marker: PhantomData },
        }
    }
}
