#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clkstat: CLKSTAT,
    clkset: CLKSET,
    clkclr: CLKCLR,
    sysclkcr: SYSCLKCR,
    cpuclkcr: CPUCLKCR,
    pbclkcr: PBCLKCR,
    usbclkcr: USBCLKCR,
    _reserved7: [u8; 0x04],
    ccuclkcr: CCUCLKCR,
    wdtclkcr: WDTCLKCR,
    extclkcr: EXTCLKCR,
    mlinkclkcr: MLINKCLKCR,
    sleepcr: SLEEPCR,
    dsleepcr: DSLEEPCR,
    ecatclkcr: ECATCLKCR,
    _reserved14: [u8; 0x04],
    cgatstat0: CGATSTAT0,
    cgatset0: CGATSET0,
    cgatclr0: CGATCLR0,
    cgatstat1: CGATSTAT1,
    cgatset1: CGATSET1,
    cgatclr1: CGATCLR1,
    cgatstat2: CGATSTAT2,
    cgatset2: CGATSET2,
    cgatclr2: CGATCLR2,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Status Register"]
    #[inline(always)]
    pub const fn clkstat(&self) -> &CLKSTAT {
        &self.clkstat
    }
    #[doc = "0x04 - CLK Set Register"]
    #[inline(always)]
    pub const fn clkset(&self) -> &CLKSET {
        &self.clkset
    }
    #[doc = "0x08 - CLK Clear Register"]
    #[inline(always)]
    pub const fn clkclr(&self) -> &CLKCLR {
        &self.clkclr
    }
    #[doc = "0x0c - System Clock Control Register"]
    #[inline(always)]
    pub const fn sysclkcr(&self) -> &SYSCLKCR {
        &self.sysclkcr
    }
    #[doc = "0x10 - CPU Clock Control Register"]
    #[inline(always)]
    pub const fn cpuclkcr(&self) -> &CPUCLKCR {
        &self.cpuclkcr
    }
    #[doc = "0x14 - Peripheral Bus Clock Control Register"]
    #[inline(always)]
    pub const fn pbclkcr(&self) -> &PBCLKCR {
        &self.pbclkcr
    }
    #[doc = "0x18 - USB Clock Control Register"]
    #[inline(always)]
    pub const fn usbclkcr(&self) -> &USBCLKCR {
        &self.usbclkcr
    }
    #[doc = "0x20 - CCU Clock Control Register"]
    #[inline(always)]
    pub const fn ccuclkcr(&self) -> &CCUCLKCR {
        &self.ccuclkcr
    }
    #[doc = "0x24 - WDT Clock Control Register"]
    #[inline(always)]
    pub const fn wdtclkcr(&self) -> &WDTCLKCR {
        &self.wdtclkcr
    }
    #[doc = "0x28 - External Clock Control"]
    #[inline(always)]
    pub const fn extclkcr(&self) -> &EXTCLKCR {
        &self.extclkcr
    }
    #[doc = "0x2c - Multi-Link Clock Control"]
    #[inline(always)]
    pub const fn mlinkclkcr(&self) -> &MLINKCLKCR {
        &self.mlinkclkcr
    }
    #[doc = "0x30 - Sleep Control Register"]
    #[inline(always)]
    pub const fn sleepcr(&self) -> &SLEEPCR {
        &self.sleepcr
    }
    #[doc = "0x34 - Deep Sleep Control Register"]
    #[inline(always)]
    pub const fn dsleepcr(&self) -> &DSLEEPCR {
        &self.dsleepcr
    }
    #[doc = "0x38 - EtherCAT Clock Control Register"]
    #[inline(always)]
    pub const fn ecatclkcr(&self) -> &ECATCLKCR {
        &self.ecatclkcr
    }
    #[doc = "0x40 - Peripheral 0 Clock Gating Status"]
    #[inline(always)]
    pub const fn cgatstat0(&self) -> &CGATSTAT0 {
        &self.cgatstat0
    }
    #[doc = "0x44 - Peripheral 0 Clock Gating Set"]
    #[inline(always)]
    pub const fn cgatset0(&self) -> &CGATSET0 {
        &self.cgatset0
    }
    #[doc = "0x48 - Peripheral 0 Clock Gating Clear"]
    #[inline(always)]
    pub const fn cgatclr0(&self) -> &CGATCLR0 {
        &self.cgatclr0
    }
    #[doc = "0x4c - Peripheral 1 Clock Gating Status"]
    #[inline(always)]
    pub const fn cgatstat1(&self) -> &CGATSTAT1 {
        &self.cgatstat1
    }
    #[doc = "0x50 - Peripheral 1 Clock Gating Set"]
    #[inline(always)]
    pub const fn cgatset1(&self) -> &CGATSET1 {
        &self.cgatset1
    }
    #[doc = "0x54 - Peripheral 1 Clock Gating Clear"]
    #[inline(always)]
    pub const fn cgatclr1(&self) -> &CGATCLR1 {
        &self.cgatclr1
    }
    #[doc = "0x58 - Peripheral 2 Clock Gating Status"]
    #[inline(always)]
    pub const fn cgatstat2(&self) -> &CGATSTAT2 {
        &self.cgatstat2
    }
    #[doc = "0x5c - Peripheral 2 Clock Gating Set"]
    #[inline(always)]
    pub const fn cgatset2(&self) -> &CGATSET2 {
        &self.cgatset2
    }
    #[doc = "0x60 - Peripheral 2 Clock Gating Clear"]
    #[inline(always)]
    pub const fn cgatclr2(&self) -> &CGATCLR2 {
        &self.cgatclr2
    }
}
#[doc = "CLKSTAT (r) register accessor: Clock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkstat`]
module"]
pub type CLKSTAT = crate::Reg<clkstat::CLKSTAT_SPEC>;
#[doc = "Clock Status Register"]
pub mod clkstat;
#[doc = "CLKSET (w) register accessor: CLK Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkset`]
module"]
pub type CLKSET = crate::Reg<clkset::CLKSET_SPEC>;
#[doc = "CLK Set Register"]
pub mod clkset;
#[doc = "CLKCLR (w) register accessor: CLK Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkclr`]
module"]
pub type CLKCLR = crate::Reg<clkclr::CLKCLR_SPEC>;
#[doc = "CLK Clear Register"]
pub mod clkclr;
#[doc = "SYSCLKCR (rw) register accessor: System Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclkcr`]
module"]
pub type SYSCLKCR = crate::Reg<sysclkcr::SYSCLKCR_SPEC>;
#[doc = "System Clock Control Register"]
pub mod sysclkcr;
#[doc = "CPUCLKCR (rw) register accessor: CPU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuclkcr`]
module"]
pub type CPUCLKCR = crate::Reg<cpuclkcr::CPUCLKCR_SPEC>;
#[doc = "CPU Clock Control Register"]
pub mod cpuclkcr;
#[doc = "PBCLKCR (rw) register accessor: Peripheral Bus Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pbclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbclkcr`]
module"]
pub type PBCLKCR = crate::Reg<pbclkcr::PBCLKCR_SPEC>;
#[doc = "Peripheral Bus Clock Control Register"]
pub mod pbclkcr;
#[doc = "USBCLKCR (rw) register accessor: USB Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkcr`]
module"]
pub type USBCLKCR = crate::Reg<usbclkcr::USBCLKCR_SPEC>;
#[doc = "USB Clock Control Register"]
pub mod usbclkcr;
#[doc = "CCUCLKCR (rw) register accessor: CCU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccuclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccuclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccuclkcr`]
module"]
pub type CCUCLKCR = crate::Reg<ccuclkcr::CCUCLKCR_SPEC>;
#[doc = "CCU Clock Control Register"]
pub mod ccuclkcr;
#[doc = "WDTCLKCR (rw) register accessor: WDT Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclkcr`]
module"]
pub type WDTCLKCR = crate::Reg<wdtclkcr::WDTCLKCR_SPEC>;
#[doc = "WDT Clock Control Register"]
pub mod wdtclkcr;
#[doc = "EXTCLKCR (rw) register accessor: External Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extclkcr`]
module"]
pub type EXTCLKCR = crate::Reg<extclkcr::EXTCLKCR_SPEC>;
#[doc = "External Clock Control"]
pub mod extclkcr;
#[doc = "MLINKCLKCR (rw) register accessor: Multi-Link Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mlinkclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mlinkclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mlinkclkcr`]
module"]
pub type MLINKCLKCR = crate::Reg<mlinkclkcr::MLINKCLKCR_SPEC>;
#[doc = "Multi-Link Clock Control"]
pub mod mlinkclkcr;
#[doc = "SLEEPCR (rw) register accessor: Sleep Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleepcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleepcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepcr`]
module"]
pub type SLEEPCR = crate::Reg<sleepcr::SLEEPCR_SPEC>;
#[doc = "Sleep Control Register"]
pub mod sleepcr;
#[doc = "DSLEEPCR (rw) register accessor: Deep Sleep Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsleepcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsleepcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsleepcr`]
module"]
pub type DSLEEPCR = crate::Reg<dsleepcr::DSLEEPCR_SPEC>;
#[doc = "Deep Sleep Control Register"]
pub mod dsleepcr;
#[doc = "ECATCLKCR (rw) register accessor: EtherCAT Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecatclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecatclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecatclkcr`]
module"]
pub type ECATCLKCR = crate::Reg<ecatclkcr::ECATCLKCR_SPEC>;
#[doc = "EtherCAT Clock Control Register"]
pub mod ecatclkcr;
#[doc = "CGATSTAT0 (r) register accessor: Peripheral 0 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatstat0`]
module"]
pub type CGATSTAT0 = crate::Reg<cgatstat0::CGATSTAT0_SPEC>;
#[doc = "Peripheral 0 Clock Gating Status"]
pub mod cgatstat0;
#[doc = "CGATSET0 (w) register accessor: Peripheral 0 Clock Gating Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatset0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatset0`]
module"]
pub type CGATSET0 = crate::Reg<cgatset0::CGATSET0_SPEC>;
#[doc = "Peripheral 0 Clock Gating Set"]
pub mod cgatset0;
#[doc = "CGATCLR0 (w) register accessor: Peripheral 0 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatclr0`]
module"]
pub type CGATCLR0 = crate::Reg<cgatclr0::CGATCLR0_SPEC>;
#[doc = "Peripheral 0 Clock Gating Clear"]
pub mod cgatclr0;
#[doc = "CGATSTAT1 (r) register accessor: Peripheral 1 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatstat1`]
module"]
pub type CGATSTAT1 = crate::Reg<cgatstat1::CGATSTAT1_SPEC>;
#[doc = "Peripheral 1 Clock Gating Status"]
pub mod cgatstat1;
#[doc = "CGATSET1 (w) register accessor: Peripheral 1 Clock Gating Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatset1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatset1`]
module"]
pub type CGATSET1 = crate::Reg<cgatset1::CGATSET1_SPEC>;
#[doc = "Peripheral 1 Clock Gating Set"]
pub mod cgatset1;
#[doc = "CGATCLR1 (w) register accessor: Peripheral 1 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatclr1`]
module"]
pub type CGATCLR1 = crate::Reg<cgatclr1::CGATCLR1_SPEC>;
#[doc = "Peripheral 1 Clock Gating Clear"]
pub mod cgatclr1;
#[doc = "CGATSTAT2 (r) register accessor: Peripheral 2 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatstat2`]
module"]
pub type CGATSTAT2 = crate::Reg<cgatstat2::CGATSTAT2_SPEC>;
#[doc = "Peripheral 2 Clock Gating Status"]
pub mod cgatstat2;
#[doc = "CGATSET2 (w) register accessor: Peripheral 2 Clock Gating Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatset2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatset2`]
module"]
pub type CGATSET2 = crate::Reg<cgatset2::CGATSET2_SPEC>;
#[doc = "Peripheral 2 Clock Gating Set"]
pub mod cgatset2;
#[doc = "CGATCLR2 (w) register accessor: Peripheral 2 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgatclr2`]
module"]
pub type CGATCLR2 = crate::Reg<cgatclr2::CGATCLR2_SPEC>;
#[doc = "Peripheral 2 Clock Gating Clear"]
pub mod cgatclr2;
