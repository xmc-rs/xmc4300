#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_diepctl: [u8; 0x04],
    _reserved1: [u8; 0x04],
    diepint: Diepint,
    _reserved2: [u8; 0x04],
    dieptsiz: Dieptsiz,
    diepdma: Diepdma,
    dtxfsts: Dtxfsts,
    diepdmab: Diepdmab,
    _reserved6: [u8; 0x01e0],
    _reserved_6_doepctl: [u8; 0x04],
    _reserved7: [u8; 0x04],
    doepint: Doepint,
    _reserved8: [u8; 0x04],
    _reserved_8_doeptsiz: [u8; 0x04],
    doepdma: Doepdma,
    _reserved10: [u8; 0x04],
    doepdmab: Doepdmab,
}
impl RegisterBlock {
    #[doc = "0x00 - Device Endpoint Control Register \\[INTBULK\\]"]
    #[inline(always)]
    pub const fn diepctl_intbulk(&self) -> &DiepctlIntbulk {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Device Endpoint Control Register \\[ISOCONT\\]"]
    #[inline(always)]
    pub const fn diepctl_isocont(&self) -> &DiepctlIsocont {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x08 - Device Endpoint Interrupt Register"]
    #[inline(always)]
    pub const fn diepint(&self) -> &Diepint {
        &self.diepint
    }
    #[doc = "0x10 - Device Endpoint Transfer Size Register"]
    #[inline(always)]
    pub const fn dieptsiz(&self) -> &Dieptsiz {
        &self.dieptsiz
    }
    #[doc = "0x14 - Device Endpoint DMA Address Register"]
    #[inline(always)]
    pub const fn diepdma(&self) -> &Diepdma {
        &self.diepdma
    }
    #[doc = "0x18 - Device IN Endpoint Transmit FIFO Status Register"]
    #[inline(always)]
    pub const fn dtxfsts(&self) -> &Dtxfsts {
        &self.dtxfsts
    }
    #[doc = "0x1c - Device Endpoint DMA Buffer Address Register"]
    #[inline(always)]
    pub const fn diepdmab(&self) -> &Diepdmab {
        &self.diepdmab
    }
    #[doc = "0x200 - Device Endpoint Control Register \\[INTBULK\\]"]
    #[inline(always)]
    pub const fn doepctl_intbulk(&self) -> &DoepctlIntbulk {
        unsafe { &*(self as *const Self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x200 - Device Endpoint Control Register \\[ISOCONT\\]"]
    #[inline(always)]
    pub const fn doepctl_isocont(&self) -> &DoepctlIsocont {
        unsafe { &*(self as *const Self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x208 - Device Endpoint Interrupt Register"]
    #[inline(always)]
    pub const fn doepint(&self) -> &Doepint {
        &self.doepint
    }
    #[doc = "0x210 - Device Endpoint Transfer Size Register \\[CONT\\]"]
    #[inline(always)]
    pub const fn doeptsiz_control(&self) -> &DoeptsizControl {
        unsafe { &*(self as *const Self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x210 - Device Endpoint Transfer Size Register \\[ISO\\]"]
    #[inline(always)]
    pub const fn doeptsiz_iso(&self) -> &DoeptsizIso {
        unsafe { &*(self as *const Self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x214 - Device Endpoint DMA Address Register"]
    #[inline(always)]
    pub const fn doepdma(&self) -> &Doepdma {
        &self.doepdma
    }
    #[doc = "0x21c - Device Endpoint DMA Buffer Address Register"]
    #[inline(always)]
    pub const fn doepdmab(&self) -> &Doepdmab {
        &self.doepdmab
    }
}
#[doc = "DIEPCTL_ISOCONT (rw) register accessor: Device Endpoint Control Register \\[ISOCONT\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl_isocont::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl_isocont::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl_isocont`]
module"]
#[doc(alias = "DIEPCTL_ISOCONT")]
pub type DiepctlIsocont = crate::Reg<diepctl_isocont::DiepctlIsocontSpec>;
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]"]
pub mod diepctl_isocont;
#[doc = "DIEPCTL_INTBULK (rw) register accessor: Device Endpoint Control Register \\[INTBULK\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl_intbulk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl_intbulk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl_intbulk`]
module"]
#[doc(alias = "DIEPCTL_INTBULK")]
pub type DiepctlIntbulk = crate::Reg<diepctl_intbulk::DiepctlIntbulkSpec>;
#[doc = "Device Endpoint Control Register \\[INTBULK\\]"]
pub mod diepctl_intbulk;
#[doc = "DIEPINT (rw) register accessor: Device Endpoint Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint`]
module"]
#[doc(alias = "DIEPINT")]
pub type Diepint = crate::Reg<diepint::DiepintSpec>;
#[doc = "Device Endpoint Interrupt Register"]
pub mod diepint;
#[doc = "DIEPTSIZ (rw) register accessor: Device Endpoint Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz`]
module"]
#[doc(alias = "DIEPTSIZ")]
pub type Dieptsiz = crate::Reg<dieptsiz::DieptsizSpec>;
#[doc = "Device Endpoint Transfer Size Register"]
pub mod dieptsiz;
#[doc = "DIEPDMA (rw) register accessor: Device Endpoint DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma`]
module"]
#[doc(alias = "DIEPDMA")]
pub type Diepdma = crate::Reg<diepdma::DiepdmaSpec>;
#[doc = "Device Endpoint DMA Address Register"]
pub mod diepdma;
#[doc = "DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts`]
module"]
#[doc(alias = "DTXFSTS")]
pub type Dtxfsts = crate::Reg<dtxfsts::DtxfstsSpec>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register"]
pub mod dtxfsts;
#[doc = "DIEPDMAB (r) register accessor: Device Endpoint DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab`]
module"]
#[doc(alias = "DIEPDMAB")]
pub type Diepdmab = crate::Reg<diepdmab::DiepdmabSpec>;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod diepdmab;
#[doc = "DOEPCTL_ISOCONT (rw) register accessor: Device Endpoint Control Register \\[ISOCONT\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl_isocont::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl_isocont::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl_isocont`]
module"]
#[doc(alias = "DOEPCTL_ISOCONT")]
pub type DoepctlIsocont = crate::Reg<doepctl_isocont::DoepctlIsocontSpec>;
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]"]
pub mod doepctl_isocont;
#[doc = "DOEPCTL_INTBULK (rw) register accessor: Device Endpoint Control Register \\[INTBULK\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl_intbulk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl_intbulk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl_intbulk`]
module"]
#[doc(alias = "DOEPCTL_INTBULK")]
pub type DoepctlIntbulk = crate::Reg<doepctl_intbulk::DoepctlIntbulkSpec>;
#[doc = "Device Endpoint Control Register \\[INTBULK\\]"]
pub mod doepctl_intbulk;
#[doc = "DOEPINT (rw) register accessor: Device Endpoint Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint`]
module"]
#[doc(alias = "DOEPINT")]
pub type Doepint = crate::Reg<doepint::DoepintSpec>;
#[doc = "Device Endpoint Interrupt Register"]
pub mod doepint;
#[doc = "DOEPTSIZ_ISO (rw) register accessor: Device Endpoint Transfer Size Register \\[ISO\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz_iso::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz_iso::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz_iso`]
module"]
#[doc(alias = "DOEPTSIZ_ISO")]
pub type DoeptsizIso = crate::Reg<doeptsiz_iso::DoeptsizIsoSpec>;
#[doc = "Device Endpoint Transfer Size Register \\[ISO\\]"]
pub mod doeptsiz_iso;
#[doc = "DOEPTSIZ_CONTROL (rw) register accessor: Device Endpoint Transfer Size Register \\[CONT\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz_control`]
module"]
#[doc(alias = "DOEPTSIZ_CONTROL")]
pub type DoeptsizControl = crate::Reg<doeptsiz_control::DoeptsizControlSpec>;
#[doc = "Device Endpoint Transfer Size Register \\[CONT\\]"]
pub mod doeptsiz_control;
#[doc = "DOEPDMA (rw) register accessor: Device Endpoint DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma`]
module"]
#[doc(alias = "DOEPDMA")]
pub type Doepdma = crate::Reg<doepdma::DoepdmaSpec>;
#[doc = "Device Endpoint DMA Address Register"]
pub mod doepdma;
#[doc = "DOEPDMAB (r) register accessor: Device Endpoint DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab`]
module"]
#[doc(alias = "DOEPDMAB")]
pub type Doepdmab = crate::Reg<doepdmab::DoepdmabSpec>;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod doepdmab;
