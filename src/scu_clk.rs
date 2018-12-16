#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Status Register"]
    pub clkstat: CLKSTAT,
    #[doc = "0x04 - CLK Set Register"]
    pub clkset: CLKSET,
    #[doc = "0x08 - CLK Clear Register"]
    pub clkclr: CLKCLR,
    #[doc = "0x0c - System Clock Control Register"]
    pub sysclkcr: SYSCLKCR,
    #[doc = "0x10 - CPU Clock Control Register"]
    pub cpuclkcr: CPUCLKCR,
    #[doc = "0x14 - Peripheral Bus Clock Control Register"]
    pub pbclkcr: PBCLKCR,
    #[doc = "0x18 - USB Clock Control Register"]
    pub usbclkcr: USBCLKCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - CCU Clock Control Register"]
    pub ccuclkcr: CCUCLKCR,
    #[doc = "0x24 - WDT Clock Control Register"]
    pub wdtclkcr: WDTCLKCR,
    #[doc = "0x28 - External Clock Control"]
    pub extclkcr: EXTCLKCR,
    #[doc = "0x2c - Multi-Link Clock Control"]
    pub mlinkclkcr: MLINKCLKCR,
    #[doc = "0x30 - Sleep Control Register"]
    pub sleepcr: SLEEPCR,
    #[doc = "0x34 - Deep Sleep Control Register"]
    pub dsleepcr: DSLEEPCR,
    #[doc = "0x38 - EtherCAT Clock Control Register"]
    pub ecatclkcr: ECATCLKCR,
    _reserved1: [u8; 4usize],
    #[doc = "0x40 - Peripheral 0 Clock Gating Status"]
    pub cgatstat0: CGATSTAT0,
    #[doc = "0x44 - Peripheral 0 Clock Gating Set"]
    pub cgatset0: CGATSET0,
    #[doc = "0x48 - Peripheral 0 Clock Gating Clear"]
    pub cgatclr0: CGATCLR0,
    #[doc = "0x4c - Peripheral 1 Clock Gating Status"]
    pub cgatstat1: CGATSTAT1,
    #[doc = "0x50 - Peripheral 1 Clock Gating Set"]
    pub cgatset1: CGATSET1,
    #[doc = "0x54 - Peripheral 1 Clock Gating Clear"]
    pub cgatclr1: CGATCLR1,
    #[doc = "0x58 - Peripheral 2 Clock Gating Status"]
    pub cgatstat2: CGATSTAT2,
    #[doc = "0x5c - Peripheral 2 Clock Gating Set"]
    pub cgatset2: CGATSET2,
    #[doc = "0x60 - Peripheral 2 Clock Gating Clear"]
    pub cgatclr2: CGATCLR2,
}
#[doc = "Clock Status Register"]
pub struct CLKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Status Register"]
pub mod clkstat;
#[doc = "CLK Set Register"]
pub struct CLKSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK Set Register"]
pub mod clkset;
#[doc = "CLK Clear Register"]
pub struct CLKCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK Clear Register"]
pub mod clkclr;
#[doc = "System Clock Control Register"]
pub struct SYSCLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Clock Control Register"]
pub mod sysclkcr;
#[doc = "CPU Clock Control Register"]
pub struct CPUCLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU Clock Control Register"]
pub mod cpuclkcr;
#[doc = "Peripheral Bus Clock Control Register"]
pub struct PBCLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Bus Clock Control Register"]
pub mod pbclkcr;
#[doc = "USB Clock Control Register"]
pub struct USBCLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Clock Control Register"]
pub mod usbclkcr;
#[doc = "CCU Clock Control Register"]
pub struct CCUCLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCU Clock Control Register"]
pub mod ccuclkcr;
#[doc = "WDT Clock Control Register"]
pub struct WDTCLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Clock Control Register"]
pub mod wdtclkcr;
#[doc = "External Clock Control"]
pub struct EXTCLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Clock Control"]
pub mod extclkcr;
#[doc = "Multi-Link Clock Control"]
pub struct MLINKCLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-Link Clock Control"]
pub mod mlinkclkcr;
#[doc = "Sleep Control Register"]
pub struct SLEEPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sleep Control Register"]
pub mod sleepcr;
#[doc = "Deep Sleep Control Register"]
pub struct DSLEEPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep Sleep Control Register"]
pub mod dsleepcr;
#[doc = "EtherCAT Clock Control Register"]
pub struct ECATCLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EtherCAT Clock Control Register"]
pub mod ecatclkcr;
#[doc = "Peripheral 0 Clock Gating Status"]
pub struct CGATSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 0 Clock Gating Status"]
pub mod cgatstat0;
#[doc = "Peripheral 0 Clock Gating Set"]
pub struct CGATSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 0 Clock Gating Set"]
pub mod cgatset0;
#[doc = "Peripheral 0 Clock Gating Clear"]
pub struct CGATCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 0 Clock Gating Clear"]
pub mod cgatclr0;
#[doc = "Peripheral 1 Clock Gating Status"]
pub struct CGATSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 1 Clock Gating Status"]
pub mod cgatstat1;
#[doc = "Peripheral 1 Clock Gating Set"]
pub struct CGATSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 1 Clock Gating Set"]
pub mod cgatset1;
#[doc = "Peripheral 1 Clock Gating Clear"]
pub struct CGATCLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 1 Clock Gating Clear"]
pub mod cgatclr1;
#[doc = "Peripheral 2 Clock Gating Status"]
pub struct CGATSTAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 2 Clock Gating Status"]
pub mod cgatstat2;
#[doc = "Peripheral 2 Clock Gating Set"]
pub struct CGATSET2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 2 Clock Gating Set"]
pub mod cgatset2;
#[doc = "Peripheral 2 Clock Gating Clear"]
pub struct CGATCLR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 2 Clock Gating Clear"]
pub mod cgatclr2;
