#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clkstat: Clkstat,
    clkset: Clkset,
    clkclr: Clkclr,
    sysclkcr: Sysclkcr,
    cpuclkcr: Cpuclkcr,
    pbclkcr: Pbclkcr,
    usbclkcr: Usbclkcr,
    _reserved7: [u8; 0x04],
    ccuclkcr: Ccuclkcr,
    wdtclkcr: Wdtclkcr,
    extclkcr: Extclkcr,
    mlinkclkcr: Mlinkclkcr,
    sleepcr: Sleepcr,
    dsleepcr: Dsleepcr,
    ecatclkcr: Ecatclkcr,
    _reserved14: [u8; 0x04],
    cgatstat0: Cgatstat0,
    cgatset0: Cgatset0,
    cgatclr0: Cgatclr0,
    cgatstat1: Cgatstat1,
    cgatset1: Cgatset1,
    cgatclr1: Cgatclr1,
    cgatstat2: Cgatstat2,
    cgatset2: Cgatset2,
    cgatclr2: Cgatclr2,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Status Register"]
    #[inline(always)]
    pub const fn clkstat(&self) -> &Clkstat {
        &self.clkstat
    }
    #[doc = "0x04 - CLK Set Register"]
    #[inline(always)]
    pub const fn clkset(&self) -> &Clkset {
        &self.clkset
    }
    #[doc = "0x08 - CLK Clear Register"]
    #[inline(always)]
    pub const fn clkclr(&self) -> &Clkclr {
        &self.clkclr
    }
    #[doc = "0x0c - System Clock Control Register"]
    #[inline(always)]
    pub const fn sysclkcr(&self) -> &Sysclkcr {
        &self.sysclkcr
    }
    #[doc = "0x10 - CPU Clock Control Register"]
    #[inline(always)]
    pub const fn cpuclkcr(&self) -> &Cpuclkcr {
        &self.cpuclkcr
    }
    #[doc = "0x14 - Peripheral Bus Clock Control Register"]
    #[inline(always)]
    pub const fn pbclkcr(&self) -> &Pbclkcr {
        &self.pbclkcr
    }
    #[doc = "0x18 - USB Clock Control Register"]
    #[inline(always)]
    pub const fn usbclkcr(&self) -> &Usbclkcr {
        &self.usbclkcr
    }
    #[doc = "0x20 - CCU Clock Control Register"]
    #[inline(always)]
    pub const fn ccuclkcr(&self) -> &Ccuclkcr {
        &self.ccuclkcr
    }
    #[doc = "0x24 - WDT Clock Control Register"]
    #[inline(always)]
    pub const fn wdtclkcr(&self) -> &Wdtclkcr {
        &self.wdtclkcr
    }
    #[doc = "0x28 - External Clock Control"]
    #[inline(always)]
    pub const fn extclkcr(&self) -> &Extclkcr {
        &self.extclkcr
    }
    #[doc = "0x2c - Multi-Link Clock Control"]
    #[inline(always)]
    pub const fn mlinkclkcr(&self) -> &Mlinkclkcr {
        &self.mlinkclkcr
    }
    #[doc = "0x30 - Sleep Control Register"]
    #[inline(always)]
    pub const fn sleepcr(&self) -> &Sleepcr {
        &self.sleepcr
    }
    #[doc = "0x34 - Deep Sleep Control Register"]
    #[inline(always)]
    pub const fn dsleepcr(&self) -> &Dsleepcr {
        &self.dsleepcr
    }
    #[doc = "0x38 - EtherCAT Clock Control Register"]
    #[inline(always)]
    pub const fn ecatclkcr(&self) -> &Ecatclkcr {
        &self.ecatclkcr
    }
    #[doc = "0x40 - Peripheral 0 Clock Gating Status"]
    #[inline(always)]
    pub const fn cgatstat0(&self) -> &Cgatstat0 {
        &self.cgatstat0
    }
    #[doc = "0x44 - Peripheral 0 Clock Gating Set"]
    #[inline(always)]
    pub const fn cgatset0(&self) -> &Cgatset0 {
        &self.cgatset0
    }
    #[doc = "0x48 - Peripheral 0 Clock Gating Clear"]
    #[inline(always)]
    pub const fn cgatclr0(&self) -> &Cgatclr0 {
        &self.cgatclr0
    }
    #[doc = "0x4c - Peripheral 1 Clock Gating Status"]
    #[inline(always)]
    pub const fn cgatstat1(&self) -> &Cgatstat1 {
        &self.cgatstat1
    }
    #[doc = "0x50 - Peripheral 1 Clock Gating Set"]
    #[inline(always)]
    pub const fn cgatset1(&self) -> &Cgatset1 {
        &self.cgatset1
    }
    #[doc = "0x54 - Peripheral 1 Clock Gating Clear"]
    #[inline(always)]
    pub const fn cgatclr1(&self) -> &Cgatclr1 {
        &self.cgatclr1
    }
    #[doc = "0x58 - Peripheral 2 Clock Gating Status"]
    #[inline(always)]
    pub const fn cgatstat2(&self) -> &Cgatstat2 {
        &self.cgatstat2
    }
    #[doc = "0x5c - Peripheral 2 Clock Gating Set"]
    #[inline(always)]
    pub const fn cgatset2(&self) -> &Cgatset2 {
        &self.cgatset2
    }
    #[doc = "0x60 - Peripheral 2 Clock Gating Clear"]
    #[inline(always)]
    pub const fn cgatclr2(&self) -> &Cgatclr2 {
        &self.cgatclr2
    }
}
#[doc = "CLKSTAT (r) register accessor: Clock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkstat`]
module"]
#[doc(alias = "CLKSTAT")]
pub type Clkstat = crate::Reg<clkstat::ClkstatSpec>;
#[doc = "Clock Status Register"]
pub mod clkstat;
#[doc = "CLKSET (w) register accessor: CLK Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkset`]
module"]
#[doc(alias = "CLKSET")]
pub type Clkset = crate::Reg<clkset::ClksetSpec>;
#[doc = "CLK Set Register"]
pub mod clkset;
#[doc = "CLKCLR (w) register accessor: CLK Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkclr`]
module"]
#[doc(alias = "CLKCLR")]
pub type Clkclr = crate::Reg<clkclr::ClkclrSpec>;
#[doc = "CLK Clear Register"]
pub mod clkclr;
#[doc = "SYSCLKCR (rw) register accessor: System Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclkcr`]
module"]
#[doc(alias = "SYSCLKCR")]
pub type Sysclkcr = crate::Reg<sysclkcr::SysclkcrSpec>;
#[doc = "System Clock Control Register"]
pub mod sysclkcr;
#[doc = "CPUCLKCR (rw) register accessor: CPU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuclkcr`]
module"]
#[doc(alias = "CPUCLKCR")]
pub type Cpuclkcr = crate::Reg<cpuclkcr::CpuclkcrSpec>;
#[doc = "CPU Clock Control Register"]
pub mod cpuclkcr;
#[doc = "PBCLKCR (rw) register accessor: Peripheral Bus Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pbclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbclkcr`]
module"]
#[doc(alias = "PBCLKCR")]
pub type Pbclkcr = crate::Reg<pbclkcr::PbclkcrSpec>;
#[doc = "Peripheral Bus Clock Control Register"]
pub mod pbclkcr;
#[doc = "USBCLKCR (rw) register accessor: USB Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkcr`]
module"]
#[doc(alias = "USBCLKCR")]
pub type Usbclkcr = crate::Reg<usbclkcr::UsbclkcrSpec>;
#[doc = "USB Clock Control Register"]
pub mod usbclkcr;
#[doc = "CCUCLKCR (rw) register accessor: CCU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccuclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccuclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccuclkcr`]
module"]
#[doc(alias = "CCUCLKCR")]
pub type Ccuclkcr = crate::Reg<ccuclkcr::CcuclkcrSpec>;
#[doc = "CCU Clock Control Register"]
pub mod ccuclkcr;
#[doc = "WDTCLKCR (rw) register accessor: WDT Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclkcr`]
module"]
#[doc(alias = "WDTCLKCR")]
pub type Wdtclkcr = crate::Reg<wdtclkcr::WdtclkcrSpec>;
#[doc = "WDT Clock Control Register"]
pub mod wdtclkcr;
#[doc = "EXTCLKCR (rw) register accessor: External Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extclkcr`]
module"]
#[doc(alias = "EXTCLKCR")]
pub type Extclkcr = crate::Reg<extclkcr::ExtclkcrSpec>;
#[doc = "External Clock Control"]
pub mod extclkcr;
#[doc = "MLINKCLKCR (rw) register accessor: Multi-Link Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mlinkclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mlinkclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mlinkclkcr`]
module"]
#[doc(alias = "MLINKCLKCR")]
pub type Mlinkclkcr = crate::Reg<mlinkclkcr::MlinkclkcrSpec>;
#[doc = "Multi-Link Clock Control"]
pub mod mlinkclkcr;
#[doc = "SLEEPCR (rw) register accessor: Sleep Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleepcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleepcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepcr`]
module"]
#[doc(alias = "SLEEPCR")]
pub type Sleepcr = crate::Reg<sleepcr::SleepcrSpec>;
#[doc = "Sleep Control Register"]
pub mod sleepcr;
#[doc = "DSLEEPCR (rw) register accessor: Deep Sleep Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsleepcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsleepcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsleepcr`]
module"]
#[doc(alias = "DSLEEPCR")]
pub type Dsleepcr = crate::Reg<dsleepcr::DsleepcrSpec>;
#[doc = "Deep Sleep Control Register"]
pub mod dsleepcr;
#[doc = "ECATCLKCR (rw) register accessor: EtherCAT Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecatclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecatclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecatclkcr`]
module"]
#[doc(alias = "ECATCLKCR")]
pub type Ecatclkcr = crate::Reg<ecatclkcr::EcatclkcrSpec>;
#[doc = "EtherCAT Clock Control Register"]
pub mod ecatclkcr;
#[doc = "CGATSTAT0 (r) register accessor: Peripheral 0 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatstat0`]
module"]
#[doc(alias = "CGATSTAT0")]
pub type Cgatstat0 = crate::Reg<cgatstat0::Cgatstat0Spec>;
#[doc = "Peripheral 0 Clock Gating Status"]
pub mod cgatstat0;
#[doc = "CGATSET0 (w) register accessor: Peripheral 0 Clock Gating Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatset0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatset0`]
module"]
#[doc(alias = "CGATSET0")]
pub type Cgatset0 = crate::Reg<cgatset0::Cgatset0Spec>;
#[doc = "Peripheral 0 Clock Gating Set"]
pub mod cgatset0;
#[doc = "CGATCLR0 (w) register accessor: Peripheral 0 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatclr0`]
module"]
#[doc(alias = "CGATCLR0")]
pub type Cgatclr0 = crate::Reg<cgatclr0::Cgatclr0Spec>;
#[doc = "Peripheral 0 Clock Gating Clear"]
pub mod cgatclr0;
#[doc = "CGATSTAT1 (r) register accessor: Peripheral 1 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatstat1`]
module"]
#[doc(alias = "CGATSTAT1")]
pub type Cgatstat1 = crate::Reg<cgatstat1::Cgatstat1Spec>;
#[doc = "Peripheral 1 Clock Gating Status"]
pub mod cgatstat1;
#[doc = "CGATSET1 (w) register accessor: Peripheral 1 Clock Gating Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatset1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatset1`]
module"]
#[doc(alias = "CGATSET1")]
pub type Cgatset1 = crate::Reg<cgatset1::Cgatset1Spec>;
#[doc = "Peripheral 1 Clock Gating Set"]
pub mod cgatset1;
#[doc = "CGATCLR1 (w) register accessor: Peripheral 1 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatclr1`]
module"]
#[doc(alias = "CGATCLR1")]
pub type Cgatclr1 = crate::Reg<cgatclr1::Cgatclr1Spec>;
#[doc = "Peripheral 1 Clock Gating Clear"]
pub mod cgatclr1;
#[doc = "CGATSTAT2 (r) register accessor: Peripheral 2 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatstat2`]
module"]
#[doc(alias = "CGATSTAT2")]
pub type Cgatstat2 = crate::Reg<cgatstat2::Cgatstat2Spec>;
#[doc = "Peripheral 2 Clock Gating Status"]
pub mod cgatstat2;
#[doc = "CGATSET2 (w) register accessor: Peripheral 2 Clock Gating Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatset2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatset2`]
module"]
#[doc(alias = "CGATSET2")]
pub type Cgatset2 = crate::Reg<cgatset2::Cgatset2Spec>;
#[doc = "Peripheral 2 Clock Gating Set"]
pub mod cgatset2;
#[doc = "CGATCLR2 (w) register accessor: Peripheral 2 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatclr2`]
module"]
#[doc(alias = "CGATCLR2")]
pub type Cgatclr2 = crate::Reg<cgatclr2::Cgatclr2Spec>;
#[doc = "Peripheral 2 Clock Gating Clear"]
pub mod cgatclr2;
