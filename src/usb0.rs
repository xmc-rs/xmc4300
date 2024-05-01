#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gotgctl: GOTGCTL,
    gotgint: GOTGINT,
    gahbcfg: GAHBCFG,
    gusbcfg: GUSBCFG,
    grstctl: GRSTCTL,
    _reserved_5_gintsts: [u8; 0x04],
    _reserved_6_gintmsk: [u8; 0x04],
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    grxfsiz: GRXFSIZ,
    _reserved_10_gnptxfsiz: [u8; 0x04],
    gnptxsts: GNPTXSTS,
    _reserved12: [u8; 0x0c],
    guid: GUID,
    _reserved13: [u8; 0x1c],
    gdfifocfg: GDFIFOCFG,
    _reserved14: [u8; 0xa0],
    hptxfsiz: HPTXFSIZ,
    dieptxf1: DIEPTXF1,
    dieptxf2: DIEPTXF2,
    dieptxf3: DIEPTXF3,
    dieptxf4: DIEPTXF4,
    dieptxf5: DIEPTXF5,
    dieptxf6: DIEPTXF6,
    _reserved21: [u8; 0x02e4],
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved24: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    hflbaddr: HFLBADDR,
    _reserved28: [u8; 0x20],
    hprt: HPRT,
    _reserved29: [u8; 0x03bc],
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved32: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved36: [u8; 0x08],
    dvbusdis: DVBUSDIS,
    dvbuspulse: DVBUSPULSE,
    _reserved38: [u8; 0x04],
    diepempmsk: DIEPEMPMSK,
    _reserved39: [u8; 0x05c8],
    pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and Status Register"]
    #[inline(always)]
    pub const fn gotgctl(&self) -> &GOTGCTL {
        &self.gotgctl
    }
    #[doc = "0x04 - OTG Interrupt Register"]
    #[inline(always)]
    pub const fn gotgint(&self) -> &GOTGINT {
        &self.gotgint
    }
    #[doc = "0x08 - AHB Configuration Register"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    #[doc = "0x0c - USB Configuration Register"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    #[doc = "0x10 - Reset Register"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    #[doc = "0x14 - Interrupt Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub const fn gintsts_devicemode(&self) -> &GINTSTS_DEVICEMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Interrupt Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub const fn gintsts_hostmode(&self) -> &GINTSTS_HOSTMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - Interrupt Mask Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub const fn gintmsk_devicemode(&self) -> &GINTMSK_DEVICEMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - Interrupt Mask Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub const fn gintmsk_hostmode(&self) -> &GINTMSK_HOSTMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - Receive Status Debug Read Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub const fn grxstsr_devicemode(&self) -> &GRXSTSR_DEVICEMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Receive Status Debug Read Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub const fn grxstsr_hostmode(&self) -> &GRXSTSR_HOSTMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - Receive Status Read and Pop Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub const fn grxstsp_hostmode(&self) -> &GRXSTSP_HOSTMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub const fn grxstsp_devicemode(&self) -> &GRXSTSP_DEVICEMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - Receive FIFO Size Register"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub const fn gnptxfsiz_devicemode(&self) -> &GNPTXFSIZ_DEVICEMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub const fn gnptxfsiz_hostmode(&self) -> &GNPTXFSIZ_HOSTMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - Non-Periodic Transmit FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn gnptxsts(&self) -> &GNPTXSTS {
        &self.gnptxsts
    }
    #[doc = "0x3c - USB Module Identification Register"]
    #[inline(always)]
    pub const fn guid(&self) -> &GUID {
        &self.guid
    }
    #[doc = "0x5c - Global DFIFO Software Config Register"]
    #[inline(always)]
    pub const fn gdfifocfg(&self) -> &GDFIFOCFG {
        &self.gdfifocfg
    }
    #[doc = "0x100 - Host Periodic Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &HPTXFSIZ {
        &self.hptxfsiz
    }
    #[doc = "0x104 - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF1 {
        &self.dieptxf1
    }
    #[doc = "0x108 - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF2 {
        &self.dieptxf2
    }
    #[doc = "0x10c - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF3 {
        &self.dieptxf3
    }
    #[doc = "0x110 - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF4 {
        &self.dieptxf4
    }
    #[doc = "0x114 - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &DIEPTXF5 {
        &self.dieptxf5
    }
    #[doc = "0x118 - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf6(&self) -> &DIEPTXF6 {
        &self.dieptxf6
    }
    #[doc = "0x400 - Host Configuration Register"]
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    #[doc = "0x404 - Host Frame Interval Register"]
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    #[doc = "0x408 - Host Frame Number/Frame Time Remaining Register"]
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    #[doc = "0x410 - Host Periodic Transmit FIFO/ Queue Status Register"]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    #[doc = "0x414 - Host All Channels Interrupt Register"]
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    #[doc = "0x418 - Host All Channels Interrupt Mask Register"]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    #[doc = "0x41c - Host Frame List Base Address Register"]
    #[inline(always)]
    pub const fn hflbaddr(&self) -> &HFLBADDR {
        &self.hflbaddr
    }
    #[doc = "0x440 - Host Port Control and Status Register"]
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    #[doc = "0x800 - Device Configuration Register"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    #[doc = "0x804 - Device Control Register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    #[doc = "0x808 - Device Status Register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    #[doc = "0x810 - Device IN Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    #[doc = "0x814 - Device OUT Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    #[doc = "0x818 - Device All Endpoints Interrupt Register"]
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    #[doc = "0x81c - Device All Endpoints Interrupt Mask Register"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    #[doc = "0x828 - Device VBUS Discharge Time Register"]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &DVBUSDIS {
        &self.dvbusdis
    }
    #[doc = "0x82c - Device VBUS Pulsing Time Register"]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &DVBUSPULSE {
        &self.dvbuspulse
    }
    #[doc = "0x834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    #[doc = "0xe00 - Power and Clock Gating Control Register"]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
}
#[doc = "GOTGCTL (rw) register accessor: Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`]
module"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = "Control and Status Register"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: OTG Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`]
module"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: AHB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`]
module"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: USB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`]
module"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`]
module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS_HOSTMODE (rw) register accessor: Interrupt Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts_hostmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts_hostmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts_hostmode`]
module"]
pub type GINTSTS_HOSTMODE = crate::Reg<gintsts_hostmode::GINTSTS_HOSTMODE_SPEC>;
#[doc = "Interrupt Register \\[HOSTMODE\\]"]
pub mod gintsts_hostmode;
#[doc = "GINTSTS_DEVICEMODE (rw) register accessor: Interrupt Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts_devicemode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts_devicemode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts_devicemode`]
module"]
pub type GINTSTS_DEVICEMODE = crate::Reg<gintsts_devicemode::GINTSTS_DEVICEMODE_SPEC>;
#[doc = "Interrupt Register \\[DEVICEMODE\\]"]
pub mod gintsts_devicemode;
#[doc = "GINTMSK_HOSTMODE (rw) register accessor: Interrupt Mask Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk_hostmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk_hostmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk_hostmode`]
module"]
pub type GINTMSK_HOSTMODE = crate::Reg<gintmsk_hostmode::GINTMSK_HOSTMODE_SPEC>;
#[doc = "Interrupt Mask Register \\[HOSTMODE\\]"]
pub mod gintmsk_hostmode;
#[doc = "GINTMSK_DEVICEMODE (rw) register accessor: Interrupt Mask Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk_devicemode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk_devicemode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk_devicemode`]
module"]
pub type GINTMSK_DEVICEMODE = crate::Reg<gintmsk_devicemode::GINTMSK_DEVICEMODE_SPEC>;
#[doc = "Interrupt Mask Register \\[DEVICEMODE\\]"]
pub mod gintmsk_devicemode;
#[doc = "GRXSTSR_HOSTMODE (r) register accessor: Receive Status Debug Read Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_hostmode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_hostmode`]
module"]
pub type GRXSTSR_HOSTMODE = crate::Reg<grxstsr_hostmode::GRXSTSR_HOSTMODE_SPEC>;
#[doc = "Receive Status Debug Read Register \\[HOSTMODE\\]"]
pub mod grxstsr_hostmode;
#[doc = "GRXSTSR_DEVICEMODE (r) register accessor: Receive Status Debug Read Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_devicemode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_devicemode`]
module"]
pub type GRXSTSR_DEVICEMODE = crate::Reg<grxstsr_devicemode::GRXSTSR_DEVICEMODE_SPEC>;
#[doc = "Receive Status Debug Read Register \\[DEVICEMODE\\]"]
pub mod grxstsr_devicemode;
#[doc = "GRXSTSP_DEVICEMODE (r) register accessor: Receive Status Read and Pop Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_devicemode::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_devicemode`]
module"]
pub type GRXSTSP_DEVICEMODE = crate::Reg<grxstsp_devicemode::GRXSTSP_DEVICEMODE_SPEC>;
#[doc = "Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
pub mod grxstsp_devicemode;
#[doc = "GRXSTSP_HOSTMODE (r) register accessor: Receive Status Read and Pop Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_hostmode::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_hostmode`]
module"]
pub type GRXSTSP_HOSTMODE = crate::Reg<grxstsp_hostmode::GRXSTSP_HOSTMODE_SPEC>;
#[doc = "Receive Status Read and Pop Register \\[HOSTMODE\\]"]
pub mod grxstsp_hostmode;
#[doc = "GRXFSIZ (rw) register accessor: Receive FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`]
module"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ_HOSTMODE (rw) register accessor: Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz_hostmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz_hostmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz_hostmode`]
module"]
pub type GNPTXFSIZ_HOSTMODE = crate::Reg<gnptxfsiz_hostmode::GNPTXFSIZ_HOSTMODE_SPEC>;
#[doc = "Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
pub mod gnptxfsiz_hostmode;
#[doc = "GNPTXFSIZ_DEVICEMODE (rw) register accessor: Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz_devicemode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz_devicemode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz_devicemode`]
module"]
pub type GNPTXFSIZ_DEVICEMODE = crate::Reg<gnptxfsiz_devicemode::GNPTXFSIZ_DEVICEMODE_SPEC>;
#[doc = "Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]"]
pub mod gnptxfsiz_devicemode;
#[doc = "GNPTXSTS (r) register accessor: Non-Periodic Transmit FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxsts`]
module"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = "Non-Periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "GUID (rw) register accessor: USB Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@guid`]
module"]
pub type GUID = crate::Reg<guid::GUID_SPEC>;
#[doc = "USB Module Identification Register"]
pub mod guid;
#[doc = "GDFIFOCFG (rw) register accessor: Global DFIFO Software Config Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdfifocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdfifocfg`]
module"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = "Global DFIFO Software Config Register"]
pub mod gdfifocfg;
#[doc = "HPTXFSIZ (rw) register accessor: Host Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`]
module"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`]
module"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`]
module"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`]
module"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf4`]
module"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf5`]
module"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf6`]
module"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf6;
#[doc = "HCFG (rw) register accessor: Host Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`]
module"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: Host Frame Interval Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`]
module"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "HFNUM (rw) register accessor: Host Frame Number/Frame Time Remaining Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfnum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`]
module"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "HPTXSTS (rw) register accessor: Host Periodic Transmit FIFO/ Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`]
module"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "Host Periodic Transmit FIFO/ Queue Status Register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: Host All Channels Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`]
module"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: Host All Channels Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`]
module"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "HFLBADDR (rw) register accessor: Host Frame List Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hflbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hflbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hflbaddr`]
module"]
pub type HFLBADDR = crate::Reg<hflbaddr::HFLBADDR_SPEC>;
#[doc = "Host Frame List Base Address Register"]
pub mod hflbaddr;
#[doc = "HPRT (rw) register accessor: Host Port Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`]
module"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "DCFG (rw) register accessor: Device Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: Device Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: Device IN Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: Device OUT Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: Device All Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: Device VBUS Discharge Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`]
module"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: Device VBUS Pulsing Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`]
module"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "DIEPEMPMSK (rw) register accessor: Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "PCGCCTL (rw) register accessor: Power and Clock Gating Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`]
module"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
