#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    diepctl0: Diepctl0,
    _reserved1: [u8; 0x04],
    diepint0: Diepint0,
    _reserved2: [u8; 0x04],
    dieptsiz0: Dieptsiz0,
    diepdma0: Diepdma0,
    dtxfsts0: Dtxfsts0,
    diepdmab0: Diepdmab0,
    _reserved6: [u8; 0x01e0],
    doepctl0: Doepctl0,
    _reserved7: [u8; 0x04],
    doepint0: Doepint0,
    _reserved8: [u8; 0x04],
    doeptsiz0: Doeptsiz0,
    doepdma0: Doepdma0,
    _reserved10: [u8; 0x04],
    doepdmab0: Doepdmab0,
}
impl RegisterBlock {
    #[doc = "0x00 - Device Control IN Endpoint Control Register"]
    #[inline(always)]
    pub const fn diepctl0(&self) -> &Diepctl0 {
        &self.diepctl0
    }
    #[doc = "0x08 - Device Endpoint Interrupt Register"]
    #[inline(always)]
    pub const fn diepint0(&self) -> &Diepint0 {
        &self.diepint0
    }
    #[doc = "0x10 - Device IN Endpoint Transfer Size Register"]
    #[inline(always)]
    pub const fn dieptsiz0(&self) -> &Dieptsiz0 {
        &self.dieptsiz0
    }
    #[doc = "0x14 - Device Endpoint DMA Address Register"]
    #[inline(always)]
    pub const fn diepdma0(&self) -> &Diepdma0 {
        &self.diepdma0
    }
    #[doc = "0x18 - Device IN Endpoint Transmit FIFO Status Register"]
    #[inline(always)]
    pub const fn dtxfsts0(&self) -> &Dtxfsts0 {
        &self.dtxfsts0
    }
    #[doc = "0x1c - Device Endpoint DMA Buffer Address Register"]
    #[inline(always)]
    pub const fn diepdmab0(&self) -> &Diepdmab0 {
        &self.diepdmab0
    }
    #[doc = "0x200 - Device Control OUT Endpoint Control Register"]
    #[inline(always)]
    pub const fn doepctl0(&self) -> &Doepctl0 {
        &self.doepctl0
    }
    #[doc = "0x208 - Device Endpoint Interrupt Register"]
    #[inline(always)]
    pub const fn doepint0(&self) -> &Doepint0 {
        &self.doepint0
    }
    #[doc = "0x210 - Device OUT Endpoint Transfer Size Register"]
    #[inline(always)]
    pub const fn doeptsiz0(&self) -> &Doeptsiz0 {
        &self.doeptsiz0
    }
    #[doc = "0x214 - Device Endpoint DMA Address Register"]
    #[inline(always)]
    pub const fn doepdma0(&self) -> &Doepdma0 {
        &self.doepdma0
    }
    #[doc = "0x21c - Device Endpoint DMA Buffer Address Register"]
    #[inline(always)]
    pub const fn doepdmab0(&self) -> &Doepdmab0 {
        &self.doepdmab0
    }
}
#[doc = "DIEPCTL0 (rw) register accessor: Device Control IN Endpoint Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl0`]
module"]
#[doc(alias = "DIEPCTL0")]
pub type Diepctl0 = crate::Reg<diepctl0::Diepctl0Spec>;
#[doc = "Device Control IN Endpoint Control Register"]
pub mod diepctl0;
#[doc = "DIEPINT0 (rw) register accessor: Device Endpoint Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint0`]
module"]
#[doc(alias = "DIEPINT0")]
pub type Diepint0 = crate::Reg<diepint0::Diepint0Spec>;
#[doc = "Device Endpoint Interrupt Register"]
pub mod diepint0;
#[doc = "DIEPTSIZ0 (rw) register accessor: Device IN Endpoint Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz0`]
module"]
#[doc(alias = "DIEPTSIZ0")]
pub type Dieptsiz0 = crate::Reg<dieptsiz0::Dieptsiz0Spec>;
#[doc = "Device IN Endpoint Transfer Size Register"]
pub mod dieptsiz0;
#[doc = "DIEPDMA0 (rw) register accessor: Device Endpoint DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma0::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma0`]
module"]
#[doc(alias = "DIEPDMA0")]
pub type Diepdma0 = crate::Reg<diepdma0::Diepdma0Spec>;
#[doc = "Device Endpoint DMA Address Register"]
pub mod diepdma0;
#[doc = "DTXFSTS0 (r) register accessor: Device IN Endpoint Transmit FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts0`]
module"]
#[doc(alias = "DTXFSTS0")]
pub type Dtxfsts0 = crate::Reg<dtxfsts0::Dtxfsts0Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register"]
pub mod dtxfsts0;
#[doc = "DIEPDMAB0 (r) register accessor: Device Endpoint DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab0`]
module"]
#[doc(alias = "DIEPDMAB0")]
pub type Diepdmab0 = crate::Reg<diepdmab0::Diepdmab0Spec>;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod diepdmab0;
#[doc = "DOEPCTL0 (rw) register accessor: Device Control OUT Endpoint Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl0`]
module"]
#[doc(alias = "DOEPCTL0")]
pub type Doepctl0 = crate::Reg<doepctl0::Doepctl0Spec>;
#[doc = "Device Control OUT Endpoint Control Register"]
pub mod doepctl0;
#[doc = "DOEPINT0 (rw) register accessor: Device Endpoint Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint0`]
module"]
#[doc(alias = "DOEPINT0")]
pub type Doepint0 = crate::Reg<doepint0::Doepint0Spec>;
#[doc = "Device Endpoint Interrupt Register"]
pub mod doepint0;
#[doc = "DOEPTSIZ0 (rw) register accessor: Device OUT Endpoint Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz0`]
module"]
#[doc(alias = "DOEPTSIZ0")]
pub type Doeptsiz0 = crate::Reg<doeptsiz0::Doeptsiz0Spec>;
#[doc = "Device OUT Endpoint Transfer Size Register"]
pub mod doeptsiz0;
#[doc = "DOEPDMA0 (rw) register accessor: Device Endpoint DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma0::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma0`]
module"]
#[doc(alias = "DOEPDMA0")]
pub type Doepdma0 = crate::Reg<doepdma0::Doepdma0Spec>;
#[doc = "Device Endpoint DMA Address Register"]
pub mod doepdma0;
#[doc = "DOEPDMAB0 (r) register accessor: Device Endpoint DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab0`]
module"]
#[doc(alias = "DOEPDMAB0")]
pub type Doepdmab0 = crate::Reg<doepdmab0::Doepdmab0Spec>;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod doepdmab0;
