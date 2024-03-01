#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gotgctl: Gotgctl,
    gotgint: Gotgint,
    gahbcfg: Gahbcfg,
    gusbcfg: Gusbcfg,
    grstctl: Grstctl,
    _reserved_5_gintsts: [u8; 0x04],
    _reserved_6_gintmsk: [u8; 0x04],
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    grxfsiz: Grxfsiz,
    _reserved_10_gnptxfsiz: [u8; 0x04],
    gnptxsts: Gnptxsts,
    _reserved12: [u8; 0x0c],
    guid: Guid,
    _reserved13: [u8; 0x1c],
    gdfifocfg: Gdfifocfg,
    _reserved14: [u8; 0xa0],
    hptxfsiz: Hptxfsiz,
    dieptxf1: Dieptxf1,
    dieptxf2: Dieptxf2,
    dieptxf3: Dieptxf3,
    dieptxf4: Dieptxf4,
    dieptxf5: Dieptxf5,
    dieptxf6: Dieptxf6,
    _reserved21: [u8; 0x02e4],
    hcfg: Hcfg,
    hfir: Hfir,
    hfnum: Hfnum,
    _reserved24: [u8; 0x04],
    hptxsts: Hptxsts,
    haint: Haint,
    haintmsk: Haintmsk,
    hflbaddr: Hflbaddr,
    _reserved28: [u8; 0x20],
    hprt: Hprt,
    _reserved29: [u8; 0x03bc],
    dcfg: Dcfg,
    dctl: Dctl,
    dsts: Dsts,
    _reserved32: [u8; 0x04],
    diepmsk: Diepmsk,
    doepmsk: Doepmsk,
    daint: Daint,
    daintmsk: Daintmsk,
    _reserved36: [u8; 0x08],
    dvbusdis: Dvbusdis,
    dvbuspulse: Dvbuspulse,
    _reserved38: [u8; 0x04],
    diepempmsk: Diepempmsk,
    _reserved39: [u8; 0x05c8],
    pcgcctl: Pcgcctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and Status Register"]
    #[inline(always)]
    pub const fn gotgctl(&self) -> &Gotgctl {
        &self.gotgctl
    }
    #[doc = "0x04 - OTG Interrupt Register"]
    #[inline(always)]
    pub const fn gotgint(&self) -> &Gotgint {
        &self.gotgint
    }
    #[doc = "0x08 - AHB Configuration Register"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &Gahbcfg {
        &self.gahbcfg
    }
    #[doc = "0x0c - USB Configuration Register"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &Gusbcfg {
        &self.gusbcfg
    }
    #[doc = "0x10 - Reset Register"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &Grstctl {
        &self.grstctl
    }
    #[doc = "0x14 - Interrupt Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub const fn gintsts_devicemode(&self) -> &GintstsDevicemode {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Interrupt Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub const fn gintsts_hostmode(&self) -> &GintstsHostmode {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - Interrupt Mask Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub const fn gintmsk_devicemode(&self) -> &GintmskDevicemode {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - Interrupt Mask Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub const fn gintmsk_hostmode(&self) -> &GintmskHostmode {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - Receive Status Debug Read Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub const fn grxstsr_devicemode(&self) -> &GrxstsrDevicemode {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Receive Status Debug Read Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub const fn grxstsr_hostmode(&self) -> &GrxstsrHostmode {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - Receive Status Read and Pop Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub const fn grxstsp_hostmode(&self) -> &GrxstspHostmode {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub const fn grxstsp_devicemode(&self) -> &GrxstspDevicemode {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - Receive FIFO Size Register"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &Grxfsiz {
        &self.grxfsiz
    }
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub const fn gnptxfsiz_devicemode(&self) -> &GnptxfsizDevicemode {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub const fn gnptxfsiz_hostmode(&self) -> &GnptxfsizHostmode {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - Non-Periodic Transmit FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn gnptxsts(&self) -> &Gnptxsts {
        &self.gnptxsts
    }
    #[doc = "0x3c - USB Module Identification Register"]
    #[inline(always)]
    pub const fn guid(&self) -> &Guid {
        &self.guid
    }
    #[doc = "0x5c - Global DFIFO Software Config Register"]
    #[inline(always)]
    pub const fn gdfifocfg(&self) -> &Gdfifocfg {
        &self.gdfifocfg
    }
    #[doc = "0x100 - Host Periodic Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &Hptxfsiz {
        &self.hptxfsiz
    }
    #[doc = "0x104 - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &Dieptxf1 {
        &self.dieptxf1
    }
    #[doc = "0x108 - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &Dieptxf2 {
        &self.dieptxf2
    }
    #[doc = "0x10c - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &Dieptxf3 {
        &self.dieptxf3
    }
    #[doc = "0x110 - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &Dieptxf4 {
        &self.dieptxf4
    }
    #[doc = "0x114 - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &Dieptxf5 {
        &self.dieptxf5
    }
    #[doc = "0x118 - Device IN Endpoint Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf6(&self) -> &Dieptxf6 {
        &self.dieptxf6
    }
    #[doc = "0x400 - Host Configuration Register"]
    #[inline(always)]
    pub const fn hcfg(&self) -> &Hcfg {
        &self.hcfg
    }
    #[doc = "0x404 - Host Frame Interval Register"]
    #[inline(always)]
    pub const fn hfir(&self) -> &Hfir {
        &self.hfir
    }
    #[doc = "0x408 - Host Frame Number/Frame Time Remaining Register"]
    #[inline(always)]
    pub const fn hfnum(&self) -> &Hfnum {
        &self.hfnum
    }
    #[doc = "0x410 - Host Periodic Transmit FIFO/ Queue Status Register"]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &Hptxsts {
        &self.hptxsts
    }
    #[doc = "0x414 - Host All Channels Interrupt Register"]
    #[inline(always)]
    pub const fn haint(&self) -> &Haint {
        &self.haint
    }
    #[doc = "0x418 - Host All Channels Interrupt Mask Register"]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &Haintmsk {
        &self.haintmsk
    }
    #[doc = "0x41c - Host Frame List Base Address Register"]
    #[inline(always)]
    pub const fn hflbaddr(&self) -> &Hflbaddr {
        &self.hflbaddr
    }
    #[doc = "0x440 - Host Port Control and Status Register"]
    #[inline(always)]
    pub const fn hprt(&self) -> &Hprt {
        &self.hprt
    }
    #[doc = "0x800 - Device Configuration Register"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &Dcfg {
        &self.dcfg
    }
    #[doc = "0x804 - Device Control Register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &Dctl {
        &self.dctl
    }
    #[doc = "0x808 - Device Status Register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &Dsts {
        &self.dsts
    }
    #[doc = "0x810 - Device IN Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &Diepmsk {
        &self.diepmsk
    }
    #[doc = "0x814 - Device OUT Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &Doepmsk {
        &self.doepmsk
    }
    #[doc = "0x818 - Device All Endpoints Interrupt Register"]
    #[inline(always)]
    pub const fn daint(&self) -> &Daint {
        &self.daint
    }
    #[doc = "0x81c - Device All Endpoints Interrupt Mask Register"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &Daintmsk {
        &self.daintmsk
    }
    #[doc = "0x828 - Device VBUS Discharge Time Register"]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &Dvbusdis {
        &self.dvbusdis
    }
    #[doc = "0x82c - Device VBUS Pulsing Time Register"]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &Dvbuspulse {
        &self.dvbuspulse
    }
    #[doc = "0x834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &Diepempmsk {
        &self.diepempmsk
    }
    #[doc = "0xe00 - Power and Clock Gating Control Register"]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &Pcgcctl {
        &self.pcgcctl
    }
}
#[doc = "GOTGCTL (rw) register accessor: Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`]
module"]
#[doc(alias = "GOTGCTL")]
pub type Gotgctl = crate::Reg<gotgctl::GotgctlSpec>;
#[doc = "Control and Status Register"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: OTG Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`]
module"]
#[doc(alias = "GOTGINT")]
pub type Gotgint = crate::Reg<gotgint::GotgintSpec>;
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: AHB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`]
module"]
#[doc(alias = "GAHBCFG")]
pub type Gahbcfg = crate::Reg<gahbcfg::GahbcfgSpec>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: USB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`]
module"]
#[doc(alias = "GUSBCFG")]
pub type Gusbcfg = crate::Reg<gusbcfg::GusbcfgSpec>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`]
module"]
#[doc(alias = "GRSTCTL")]
pub type Grstctl = crate::Reg<grstctl::GrstctlSpec>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS_HOSTMODE (rw) register accessor: Interrupt Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts_hostmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts_hostmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts_hostmode`]
module"]
#[doc(alias = "GINTSTS_HOSTMODE")]
pub type GintstsHostmode = crate::Reg<gintsts_hostmode::GintstsHostmodeSpec>;
#[doc = "Interrupt Register \\[HOSTMODE\\]"]
pub mod gintsts_hostmode;
#[doc = "GINTSTS_DEVICEMODE (rw) register accessor: Interrupt Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts_devicemode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts_devicemode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts_devicemode`]
module"]
#[doc(alias = "GINTSTS_DEVICEMODE")]
pub type GintstsDevicemode = crate::Reg<gintsts_devicemode::GintstsDevicemodeSpec>;
#[doc = "Interrupt Register \\[DEVICEMODE\\]"]
pub mod gintsts_devicemode;
#[doc = "GINTMSK_HOSTMODE (rw) register accessor: Interrupt Mask Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk_hostmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk_hostmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk_hostmode`]
module"]
#[doc(alias = "GINTMSK_HOSTMODE")]
pub type GintmskHostmode = crate::Reg<gintmsk_hostmode::GintmskHostmodeSpec>;
#[doc = "Interrupt Mask Register \\[HOSTMODE\\]"]
pub mod gintmsk_hostmode;
#[doc = "GINTMSK_DEVICEMODE (rw) register accessor: Interrupt Mask Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk_devicemode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk_devicemode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk_devicemode`]
module"]
#[doc(alias = "GINTMSK_DEVICEMODE")]
pub type GintmskDevicemode = crate::Reg<gintmsk_devicemode::GintmskDevicemodeSpec>;
#[doc = "Interrupt Mask Register \\[DEVICEMODE\\]"]
pub mod gintmsk_devicemode;
#[doc = "GRXSTSR_HOSTMODE (r) register accessor: Receive Status Debug Read Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_hostmode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_hostmode`]
module"]
#[doc(alias = "GRXSTSR_HOSTMODE")]
pub type GrxstsrHostmode = crate::Reg<grxstsr_hostmode::GrxstsrHostmodeSpec>;
#[doc = "Receive Status Debug Read Register \\[HOSTMODE\\]"]
pub mod grxstsr_hostmode;
#[doc = "GRXSTSR_DEVICEMODE (r) register accessor: Receive Status Debug Read Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_devicemode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_devicemode`]
module"]
#[doc(alias = "GRXSTSR_DEVICEMODE")]
pub type GrxstsrDevicemode = crate::Reg<grxstsr_devicemode::GrxstsrDevicemodeSpec>;
#[doc = "Receive Status Debug Read Register \\[DEVICEMODE\\]"]
pub mod grxstsr_devicemode;
#[doc = "GRXSTSP_DEVICEMODE (r) register accessor: Receive Status Read and Pop Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_devicemode::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_devicemode`]
module"]
#[doc(alias = "GRXSTSP_DEVICEMODE")]
pub type GrxstspDevicemode = crate::Reg<grxstsp_devicemode::GrxstspDevicemodeSpec>;
#[doc = "Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
pub mod grxstsp_devicemode;
#[doc = "GRXSTSP_HOSTMODE (r) register accessor: Receive Status Read and Pop Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_hostmode::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_hostmode`]
module"]
#[doc(alias = "GRXSTSP_HOSTMODE")]
pub type GrxstspHostmode = crate::Reg<grxstsp_hostmode::GrxstspHostmodeSpec>;
#[doc = "Receive Status Read and Pop Register \\[HOSTMODE\\]"]
pub mod grxstsp_hostmode;
#[doc = "GRXFSIZ (rw) register accessor: Receive FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`]
module"]
#[doc(alias = "GRXFSIZ")]
pub type Grxfsiz = crate::Reg<grxfsiz::GrxfsizSpec>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ_HOSTMODE (rw) register accessor: Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz_hostmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz_hostmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz_hostmode`]
module"]
#[doc(alias = "GNPTXFSIZ_HOSTMODE")]
pub type GnptxfsizHostmode = crate::Reg<gnptxfsiz_hostmode::GnptxfsizHostmodeSpec>;
#[doc = "Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
pub mod gnptxfsiz_hostmode;
#[doc = "GNPTXFSIZ_DEVICEMODE (rw) register accessor: Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz_devicemode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz_devicemode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz_devicemode`]
module"]
#[doc(alias = "GNPTXFSIZ_DEVICEMODE")]
pub type GnptxfsizDevicemode = crate::Reg<gnptxfsiz_devicemode::GnptxfsizDevicemodeSpec>;
#[doc = "Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]"]
pub mod gnptxfsiz_devicemode;
#[doc = "GNPTXSTS (r) register accessor: Non-Periodic Transmit FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxsts`]
module"]
#[doc(alias = "GNPTXSTS")]
pub type Gnptxsts = crate::Reg<gnptxsts::GnptxstsSpec>;
#[doc = "Non-Periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "GUID (rw) register accessor: USB Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@guid`]
module"]
#[doc(alias = "GUID")]
pub type Guid = crate::Reg<guid::GuidSpec>;
#[doc = "USB Module Identification Register"]
pub mod guid;
#[doc = "GDFIFOCFG (rw) register accessor: Global DFIFO Software Config Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdfifocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdfifocfg`]
module"]
#[doc(alias = "GDFIFOCFG")]
pub type Gdfifocfg = crate::Reg<gdfifocfg::GdfifocfgSpec>;
#[doc = "Global DFIFO Software Config Register"]
pub mod gdfifocfg;
#[doc = "HPTXFSIZ (rw) register accessor: Host Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`]
module"]
#[doc(alias = "HPTXFSIZ")]
pub type Hptxfsiz = crate::Reg<hptxfsiz::HptxfsizSpec>;
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`]
module"]
#[doc(alias = "DIEPTXF1")]
pub type Dieptxf1 = crate::Reg<dieptxf1::Dieptxf1Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`]
module"]
#[doc(alias = "DIEPTXF2")]
pub type Dieptxf2 = crate::Reg<dieptxf2::Dieptxf2Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`]
module"]
#[doc(alias = "DIEPTXF3")]
pub type Dieptxf3 = crate::Reg<dieptxf3::Dieptxf3Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf4`]
module"]
#[doc(alias = "DIEPTXF4")]
pub type Dieptxf4 = crate::Reg<dieptxf4::Dieptxf4Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf5`]
module"]
#[doc(alias = "DIEPTXF5")]
pub type Dieptxf5 = crate::Reg<dieptxf5::Dieptxf5Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf6`]
module"]
#[doc(alias = "DIEPTXF6")]
pub type Dieptxf6 = crate::Reg<dieptxf6::Dieptxf6Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf6;
#[doc = "HCFG (rw) register accessor: Host Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`]
module"]
#[doc(alias = "HCFG")]
pub type Hcfg = crate::Reg<hcfg::HcfgSpec>;
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: Host Frame Interval Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`]
module"]
#[doc(alias = "HFIR")]
pub type Hfir = crate::Reg<hfir::HfirSpec>;
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "HFNUM (rw) register accessor: Host Frame Number/Frame Time Remaining Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfnum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`]
module"]
#[doc(alias = "HFNUM")]
pub type Hfnum = crate::Reg<hfnum::HfnumSpec>;
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "HPTXSTS (rw) register accessor: Host Periodic Transmit FIFO/ Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`]
module"]
#[doc(alias = "HPTXSTS")]
pub type Hptxsts = crate::Reg<hptxsts::HptxstsSpec>;
#[doc = "Host Periodic Transmit FIFO/ Queue Status Register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: Host All Channels Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`]
module"]
#[doc(alias = "HAINT")]
pub type Haint = crate::Reg<haint::HaintSpec>;
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: Host All Channels Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`]
module"]
#[doc(alias = "HAINTMSK")]
pub type Haintmsk = crate::Reg<haintmsk::HaintmskSpec>;
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "HFLBADDR (rw) register accessor: Host Frame List Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hflbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hflbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hflbaddr`]
module"]
#[doc(alias = "HFLBADDR")]
pub type Hflbaddr = crate::Reg<hflbaddr::HflbaddrSpec>;
#[doc = "Host Frame List Base Address Register"]
pub mod hflbaddr;
#[doc = "HPRT (rw) register accessor: Host Port Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`]
module"]
#[doc(alias = "HPRT")]
pub type Hprt = crate::Reg<hprt::HprtSpec>;
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "DCFG (rw) register accessor: Device Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
#[doc(alias = "DCFG")]
pub type Dcfg = crate::Reg<dcfg::DcfgSpec>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
#[doc(alias = "DCTL")]
pub type Dctl = crate::Reg<dctl::DctlSpec>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: Device Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
#[doc(alias = "DSTS")]
pub type Dsts = crate::Reg<dsts::DstsSpec>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: Device IN Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
#[doc(alias = "DIEPMSK")]
pub type Diepmsk = crate::Reg<diepmsk::DiepmskSpec>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: Device OUT Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
#[doc(alias = "DOEPMSK")]
pub type Doepmsk = crate::Reg<doepmsk::DoepmskSpec>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
#[doc(alias = "DAINT")]
pub type Daint = crate::Reg<daint::DaintSpec>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: Device All Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
#[doc(alias = "DAINTMSK")]
pub type Daintmsk = crate::Reg<daintmsk::DaintmskSpec>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: Device VBUS Discharge Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`]
module"]
#[doc(alias = "DVBUSDIS")]
pub type Dvbusdis = crate::Reg<dvbusdis::DvbusdisSpec>;
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: Device VBUS Pulsing Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`]
module"]
#[doc(alias = "DVBUSPULSE")]
pub type Dvbuspulse = crate::Reg<dvbuspulse::DvbuspulseSpec>;
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "DIEPEMPMSK (rw) register accessor: Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
#[doc(alias = "DIEPEMPMSK")]
pub type Diepempmsk = crate::Reg<diepempmsk::DiepempmskSpec>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "PCGCCTL (rw) register accessor: Power and Clock Gating Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`]
module"]
#[doc(alias = "PCGCCTL")]
pub type Pcgcctl = crate::Reg<pcgcctl::PcgcctlSpec>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
