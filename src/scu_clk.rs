#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Status Register"]
    pub clkstat: crate::Reg<clkstat::CLKSTAT_SPEC>,
    #[doc = "0x04 - CLK Set Register"]
    pub clkset: crate::Reg<clkset::CLKSET_SPEC>,
    #[doc = "0x08 - CLK Clear Register"]
    pub clkclr: crate::Reg<clkclr::CLKCLR_SPEC>,
    #[doc = "0x0c - System Clock Control Register"]
    pub sysclkcr: crate::Reg<sysclkcr::SYSCLKCR_SPEC>,
    #[doc = "0x10 - CPU Clock Control Register"]
    pub cpuclkcr: crate::Reg<cpuclkcr::CPUCLKCR_SPEC>,
    #[doc = "0x14 - Peripheral Bus Clock Control Register"]
    pub pbclkcr: crate::Reg<pbclkcr::PBCLKCR_SPEC>,
    #[doc = "0x18 - USB Clock Control Register"]
    pub usbclkcr: crate::Reg<usbclkcr::USBCLKCR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - CCU Clock Control Register"]
    pub ccuclkcr: crate::Reg<ccuclkcr::CCUCLKCR_SPEC>,
    #[doc = "0x24 - WDT Clock Control Register"]
    pub wdtclkcr: crate::Reg<wdtclkcr::WDTCLKCR_SPEC>,
    #[doc = "0x28 - External Clock Control"]
    pub extclkcr: crate::Reg<extclkcr::EXTCLKCR_SPEC>,
    #[doc = "0x2c - Multi-Link Clock Control"]
    pub mlinkclkcr: crate::Reg<mlinkclkcr::MLINKCLKCR_SPEC>,
    #[doc = "0x30 - Sleep Control Register"]
    pub sleepcr: crate::Reg<sleepcr::SLEEPCR_SPEC>,
    #[doc = "0x34 - Deep Sleep Control Register"]
    pub dsleepcr: crate::Reg<dsleepcr::DSLEEPCR_SPEC>,
    #[doc = "0x38 - EtherCAT Clock Control Register"]
    pub ecatclkcr: crate::Reg<ecatclkcr::ECATCLKCR_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x40 - Peripheral 0 Clock Gating Status"]
    pub cgatstat0: crate::Reg<cgatstat0::CGATSTAT0_SPEC>,
    #[doc = "0x44 - Peripheral 0 Clock Gating Set"]
    pub cgatset0: crate::Reg<cgatset0::CGATSET0_SPEC>,
    #[doc = "0x48 - Peripheral 0 Clock Gating Clear"]
    pub cgatclr0: crate::Reg<cgatclr0::CGATCLR0_SPEC>,
    #[doc = "0x4c - Peripheral 1 Clock Gating Status"]
    pub cgatstat1: crate::Reg<cgatstat1::CGATSTAT1_SPEC>,
    #[doc = "0x50 - Peripheral 1 Clock Gating Set"]
    pub cgatset1: crate::Reg<cgatset1::CGATSET1_SPEC>,
    #[doc = "0x54 - Peripheral 1 Clock Gating Clear"]
    pub cgatclr1: crate::Reg<cgatclr1::CGATCLR1_SPEC>,
    #[doc = "0x58 - Peripheral 2 Clock Gating Status"]
    pub cgatstat2: crate::Reg<cgatstat2::CGATSTAT2_SPEC>,
    #[doc = "0x5c - Peripheral 2 Clock Gating Set"]
    pub cgatset2: crate::Reg<cgatset2::CGATSET2_SPEC>,
    #[doc = "0x60 - Peripheral 2 Clock Gating Clear"]
    pub cgatclr2: crate::Reg<cgatclr2::CGATCLR2_SPEC>,
}
#[doc = "CLKSTAT register accessor: an alias for `Reg<CLKSTAT_SPEC>`"]
pub type CLKSTAT = crate::Reg<clkstat::CLKSTAT_SPEC>;
#[doc = "Clock Status Register"]
pub mod clkstat;
#[doc = "CLKSET register accessor: an alias for `Reg<CLKSET_SPEC>`"]
pub type CLKSET = crate::Reg<clkset::CLKSET_SPEC>;
#[doc = "CLK Set Register"]
pub mod clkset;
#[doc = "CLKCLR register accessor: an alias for `Reg<CLKCLR_SPEC>`"]
pub type CLKCLR = crate::Reg<clkclr::CLKCLR_SPEC>;
#[doc = "CLK Clear Register"]
pub mod clkclr;
#[doc = "SYSCLKCR register accessor: an alias for `Reg<SYSCLKCR_SPEC>`"]
pub type SYSCLKCR = crate::Reg<sysclkcr::SYSCLKCR_SPEC>;
#[doc = "System Clock Control Register"]
pub mod sysclkcr;
#[doc = "CPUCLKCR register accessor: an alias for `Reg<CPUCLKCR_SPEC>`"]
pub type CPUCLKCR = crate::Reg<cpuclkcr::CPUCLKCR_SPEC>;
#[doc = "CPU Clock Control Register"]
pub mod cpuclkcr;
#[doc = "PBCLKCR register accessor: an alias for `Reg<PBCLKCR_SPEC>`"]
pub type PBCLKCR = crate::Reg<pbclkcr::PBCLKCR_SPEC>;
#[doc = "Peripheral Bus Clock Control Register"]
pub mod pbclkcr;
#[doc = "USBCLKCR register accessor: an alias for `Reg<USBCLKCR_SPEC>`"]
pub type USBCLKCR = crate::Reg<usbclkcr::USBCLKCR_SPEC>;
#[doc = "USB Clock Control Register"]
pub mod usbclkcr;
#[doc = "CCUCLKCR register accessor: an alias for `Reg<CCUCLKCR_SPEC>`"]
pub type CCUCLKCR = crate::Reg<ccuclkcr::CCUCLKCR_SPEC>;
#[doc = "CCU Clock Control Register"]
pub mod ccuclkcr;
#[doc = "WDTCLKCR register accessor: an alias for `Reg<WDTCLKCR_SPEC>`"]
pub type WDTCLKCR = crate::Reg<wdtclkcr::WDTCLKCR_SPEC>;
#[doc = "WDT Clock Control Register"]
pub mod wdtclkcr;
#[doc = "EXTCLKCR register accessor: an alias for `Reg<EXTCLKCR_SPEC>`"]
pub type EXTCLKCR = crate::Reg<extclkcr::EXTCLKCR_SPEC>;
#[doc = "External Clock Control"]
pub mod extclkcr;
#[doc = "MLINKCLKCR register accessor: an alias for `Reg<MLINKCLKCR_SPEC>`"]
pub type MLINKCLKCR = crate::Reg<mlinkclkcr::MLINKCLKCR_SPEC>;
#[doc = "Multi-Link Clock Control"]
pub mod mlinkclkcr;
#[doc = "SLEEPCR register accessor: an alias for `Reg<SLEEPCR_SPEC>`"]
pub type SLEEPCR = crate::Reg<sleepcr::SLEEPCR_SPEC>;
#[doc = "Sleep Control Register"]
pub mod sleepcr;
#[doc = "DSLEEPCR register accessor: an alias for `Reg<DSLEEPCR_SPEC>`"]
pub type DSLEEPCR = crate::Reg<dsleepcr::DSLEEPCR_SPEC>;
#[doc = "Deep Sleep Control Register"]
pub mod dsleepcr;
#[doc = "ECATCLKCR register accessor: an alias for `Reg<ECATCLKCR_SPEC>`"]
pub type ECATCLKCR = crate::Reg<ecatclkcr::ECATCLKCR_SPEC>;
#[doc = "EtherCAT Clock Control Register"]
pub mod ecatclkcr;
#[doc = "CGATSTAT0 register accessor: an alias for `Reg<CGATSTAT0_SPEC>`"]
pub type CGATSTAT0 = crate::Reg<cgatstat0::CGATSTAT0_SPEC>;
#[doc = "Peripheral 0 Clock Gating Status"]
pub mod cgatstat0;
#[doc = "CGATSET0 register accessor: an alias for `Reg<CGATSET0_SPEC>`"]
pub type CGATSET0 = crate::Reg<cgatset0::CGATSET0_SPEC>;
#[doc = "Peripheral 0 Clock Gating Set"]
pub mod cgatset0;
#[doc = "CGATCLR0 register accessor: an alias for `Reg<CGATCLR0_SPEC>`"]
pub type CGATCLR0 = crate::Reg<cgatclr0::CGATCLR0_SPEC>;
#[doc = "Peripheral 0 Clock Gating Clear"]
pub mod cgatclr0;
#[doc = "CGATSTAT1 register accessor: an alias for `Reg<CGATSTAT1_SPEC>`"]
pub type CGATSTAT1 = crate::Reg<cgatstat1::CGATSTAT1_SPEC>;
#[doc = "Peripheral 1 Clock Gating Status"]
pub mod cgatstat1;
#[doc = "CGATSET1 register accessor: an alias for `Reg<CGATSET1_SPEC>`"]
pub type CGATSET1 = crate::Reg<cgatset1::CGATSET1_SPEC>;
#[doc = "Peripheral 1 Clock Gating Set"]
pub mod cgatset1;
#[doc = "CGATCLR1 register accessor: an alias for `Reg<CGATCLR1_SPEC>`"]
pub type CGATCLR1 = crate::Reg<cgatclr1::CGATCLR1_SPEC>;
#[doc = "Peripheral 1 Clock Gating Clear"]
pub mod cgatclr1;
#[doc = "CGATSTAT2 register accessor: an alias for `Reg<CGATSTAT2_SPEC>`"]
pub type CGATSTAT2 = crate::Reg<cgatstat2::CGATSTAT2_SPEC>;
#[doc = "Peripheral 2 Clock Gating Status"]
pub mod cgatstat2;
#[doc = "CGATSET2 register accessor: an alias for `Reg<CGATSET2_SPEC>`"]
pub type CGATSET2 = crate::Reg<cgatset2::CGATSET2_SPEC>;
#[doc = "Peripheral 2 Clock Gating Set"]
pub mod cgatset2;
#[doc = "CGATCLR2 register accessor: an alias for `Reg<CGATCLR2_SPEC>`"]
pub type CGATCLR2 = crate::Reg<cgatclr2::CGATCLR2_SPEC>;
#[doc = "Peripheral 2 Clock Gating Clear"]
pub mod cgatclr2;
