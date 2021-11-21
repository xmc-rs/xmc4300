#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control and Status Register"]
    pub gotgctl: crate::Reg<gotgctl::GOTGCTL_SPEC>,
    #[doc = "0x04 - OTG Interrupt Register"]
    pub gotgint: crate::Reg<gotgint::GOTGINT_SPEC>,
    #[doc = "0x08 - AHB Configuration Register"]
    pub gahbcfg: crate::Reg<gahbcfg::GAHBCFG_SPEC>,
    #[doc = "0x0c - USB Configuration Register"]
    pub gusbcfg: crate::Reg<gusbcfg::GUSBCFG_SPEC>,
    #[doc = "0x10 - Reset Register"]
    pub grstctl: crate::Reg<grstctl::GRSTCTL_SPEC>,
    _reserved_5_gintsts: [u8; 0x04],
    _reserved_6_gintmsk: [u8; 0x04],
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    #[doc = "0x24 - Receive FIFO Size Register"]
    pub grxfsiz: crate::Reg<grxfsiz::GRXFSIZ_SPEC>,
    _reserved_10_gnptxfsiz: [u8; 0x04],
    #[doc = "0x2c - Non-Periodic Transmit FIFO/Queue Status Register"]
    pub gnptxsts: crate::Reg<gnptxsts::GNPTXSTS_SPEC>,
    _reserved12: [u8; 0x0c],
    #[doc = "0x3c - USB Module Identification Register"]
    pub guid: crate::Reg<guid::GUID_SPEC>,
    _reserved13: [u8; 0x1c],
    #[doc = "0x5c - Global DFIFO Software Config Register"]
    pub gdfifocfg: crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>,
    _reserved14: [u8; 0xa0],
    #[doc = "0x100 - Host Periodic Transmit FIFO Size Register"]
    pub hptxfsiz: crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>,
    #[doc = "0x104 - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf1: crate::Reg<dieptxf1::DIEPTXF1_SPEC>,
    #[doc = "0x108 - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf2: crate::Reg<dieptxf2::DIEPTXF2_SPEC>,
    #[doc = "0x10c - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf3: crate::Reg<dieptxf3::DIEPTXF3_SPEC>,
    #[doc = "0x110 - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf4: crate::Reg<dieptxf4::DIEPTXF4_SPEC>,
    #[doc = "0x114 - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf5: crate::Reg<dieptxf5::DIEPTXF5_SPEC>,
    #[doc = "0x118 - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf6: crate::Reg<dieptxf6::DIEPTXF6_SPEC>,
    _reserved21: [u8; 0x02e4],
    #[doc = "0x400 - Host Configuration Register"]
    pub hcfg: crate::Reg<hcfg::HCFG_SPEC>,
    #[doc = "0x404 - Host Frame Interval Register"]
    pub hfir: crate::Reg<hfir::HFIR_SPEC>,
    #[doc = "0x408 - Host Frame Number/Frame Time Remaining Register"]
    pub hfnum: crate::Reg<hfnum::HFNUM_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x410 - Host Periodic Transmit FIFO/ Queue Status Register"]
    pub hptxsts: crate::Reg<hptxsts::HPTXSTS_SPEC>,
    #[doc = "0x414 - Host All Channels Interrupt Register"]
    pub haint: crate::Reg<haint::HAINT_SPEC>,
    #[doc = "0x418 - Host All Channels Interrupt Mask Register"]
    pub haintmsk: crate::Reg<haintmsk::HAINTMSK_SPEC>,
    #[doc = "0x41c - Host Frame List Base Address Register"]
    pub hflbaddr: crate::Reg<hflbaddr::HFLBADDR_SPEC>,
    _reserved28: [u8; 0x20],
    #[doc = "0x440 - Host Port Control and Status Register"]
    pub hprt: crate::Reg<hprt::HPRT_SPEC>,
    _reserved29: [u8; 0x03bc],
    #[doc = "0x800 - Device Configuration Register"]
    pub dcfg: crate::Reg<dcfg::DCFG_SPEC>,
    #[doc = "0x804 - Device Control Register"]
    pub dctl: crate::Reg<dctl::DCTL_SPEC>,
    #[doc = "0x808 - Device Status Register"]
    pub dsts: crate::Reg<dsts::DSTS_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: crate::Reg<diepmsk::DIEPMSK_SPEC>,
    #[doc = "0x814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: crate::Reg<doepmsk::DOEPMSK_SPEC>,
    #[doc = "0x818 - Device All Endpoints Interrupt Register"]
    pub daint: crate::Reg<daint::DAINT_SPEC>,
    #[doc = "0x81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: crate::Reg<daintmsk::DAINTMSK_SPEC>,
    _reserved36: [u8; 0x08],
    #[doc = "0x828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: crate::Reg<dvbusdis::DVBUSDIS_SPEC>,
    #[doc = "0x82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>,
    _reserved38: [u8; 0x04],
    #[doc = "0x834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>,
    _reserved39: [u8; 0x05c8],
    #[doc = "0xe00 - Power and Clock Gating Control Register"]
    pub pcgcctl: crate::Reg<pcgcctl::PCGCCTL_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x14 - Interrupt Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn gintsts_devicemode(&self) -> &crate::Reg<gintsts_devicemode::GINTSTS_DEVICEMODE_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const crate::Reg<gintsts_devicemode::GINTSTS_DEVICEMODE_SPEC>) }
    }
    #[doc = "0x14 - Interrupt Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn gintsts_hostmode(&self) -> &crate::Reg<gintsts_hostmode::GINTSTS_HOSTMODE_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const crate::Reg<gintsts_hostmode::GINTSTS_HOSTMODE_SPEC>) }
    }
    #[doc = "0x18 - Interrupt Mask Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn gintmsk_devicemode(&self) -> &crate::Reg<gintmsk_devicemode::GINTMSK_DEVICEMODE_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const crate::Reg<gintmsk_devicemode::GINTMSK_DEVICEMODE_SPEC>) }
    }
    #[doc = "0x18 - Interrupt Mask Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn gintmsk_hostmode(&self) -> &crate::Reg<gintmsk_hostmode::GINTMSK_HOSTMODE_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const crate::Reg<gintmsk_hostmode::GINTMSK_HOSTMODE_SPEC>) }
    }
    #[doc = "0x1c - Receive Status Debug Read Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn grxstsr_devicemode(&self) -> &crate::Reg<grxstsr_devicemode::GRXSTSR_DEVICEMODE_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const crate::Reg<grxstsr_devicemode::GRXSTSR_DEVICEMODE_SPEC>) }
    }
    #[doc = "0x1c - Receive Status Debug Read Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn grxstsr_hostmode(&self) -> &crate::Reg<grxstsr_hostmode::GRXSTSR_HOSTMODE_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const crate::Reg<grxstsr_hostmode::GRXSTSR_HOSTMODE_SPEC>) }
    }
    #[doc = "0x20 - Receive Status Read and Pop Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn grxstsp_hostmode(&self) -> &crate::Reg<grxstsp_hostmode::GRXSTSP_HOSTMODE_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const crate::Reg<grxstsp_hostmode::GRXSTSP_HOSTMODE_SPEC>) }
    }
    #[doc = "0x20 - Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn grxstsp_devicemode(&self) -> &crate::Reg<grxstsp_devicemode::GRXSTSP_DEVICEMODE_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const crate::Reg<grxstsp_devicemode::GRXSTSP_DEVICEMODE_SPEC>) }
    }
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn gnptxfsiz_devicemode(&self) -> &crate::Reg<gnptxfsiz_devicemode::GNPTXFSIZ_DEVICEMODE_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const crate::Reg<gnptxfsiz_devicemode::GNPTXFSIZ_DEVICEMODE_SPEC>) }
    }
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn gnptxfsiz_hostmode(&self) -> &crate::Reg<gnptxfsiz_hostmode::GNPTXFSIZ_HOSTMODE_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const crate::Reg<gnptxfsiz_hostmode::GNPTXFSIZ_HOSTMODE_SPEC>) }
    }
}
#[doc = "GOTGCTL register accessor: an alias for `Reg<GOTGCTL_SPEC>`"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = "Control and Status Register"]
pub mod gotgctl;
#[doc = "GOTGINT register accessor: an alias for `Reg<GOTGINT_SPEC>`"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "GAHBCFG register accessor: an alias for `Reg<GAHBCFG_SPEC>`"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG register accessor: an alias for `Reg<GUSBCFG_SPEC>`"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL register accessor: an alias for `Reg<GRSTCTL_SPEC>`"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS_HOSTMODE register accessor: an alias for `Reg<GINTSTS_HOSTMODE_SPEC>`"]
pub type GINTSTS_HOSTMODE = crate::Reg<gintsts_hostmode::GINTSTS_HOSTMODE_SPEC>;
#[doc = "Interrupt Register \\[HOSTMODE\\]"]
pub mod gintsts_hostmode;
#[doc = "GINTSTS_DEVICEMODE register accessor: an alias for `Reg<GINTSTS_DEVICEMODE_SPEC>`"]
pub type GINTSTS_DEVICEMODE = crate::Reg<gintsts_devicemode::GINTSTS_DEVICEMODE_SPEC>;
#[doc = "Interrupt Register \\[DEVICEMODE\\]"]
pub mod gintsts_devicemode;
#[doc = "GINTMSK_HOSTMODE register accessor: an alias for `Reg<GINTMSK_HOSTMODE_SPEC>`"]
pub type GINTMSK_HOSTMODE = crate::Reg<gintmsk_hostmode::GINTMSK_HOSTMODE_SPEC>;
#[doc = "Interrupt Mask Register \\[HOSTMODE\\]"]
pub mod gintmsk_hostmode;
#[doc = "GINTMSK_DEVICEMODE register accessor: an alias for `Reg<GINTMSK_DEVICEMODE_SPEC>`"]
pub type GINTMSK_DEVICEMODE = crate::Reg<gintmsk_devicemode::GINTMSK_DEVICEMODE_SPEC>;
#[doc = "Interrupt Mask Register \\[DEVICEMODE\\]"]
pub mod gintmsk_devicemode;
#[doc = "GRXSTSR_HOSTMODE register accessor: an alias for `Reg<GRXSTSR_HOSTMODE_SPEC>`"]
pub type GRXSTSR_HOSTMODE = crate::Reg<grxstsr_hostmode::GRXSTSR_HOSTMODE_SPEC>;
#[doc = "Receive Status Debug Read Register \\[HOSTMODE\\]"]
pub mod grxstsr_hostmode;
#[doc = "GRXSTSR_DEVICEMODE register accessor: an alias for `Reg<GRXSTSR_DEVICEMODE_SPEC>`"]
pub type GRXSTSR_DEVICEMODE = crate::Reg<grxstsr_devicemode::GRXSTSR_DEVICEMODE_SPEC>;
#[doc = "Receive Status Debug Read Register \\[DEVICEMODE\\]"]
pub mod grxstsr_devicemode;
#[doc = "GRXSTSP_DEVICEMODE register accessor: an alias for `Reg<GRXSTSP_DEVICEMODE_SPEC>`"]
pub type GRXSTSP_DEVICEMODE = crate::Reg<grxstsp_devicemode::GRXSTSP_DEVICEMODE_SPEC>;
#[doc = "Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
pub mod grxstsp_devicemode;
#[doc = "GRXSTSP_HOSTMODE register accessor: an alias for `Reg<GRXSTSP_HOSTMODE_SPEC>`"]
pub type GRXSTSP_HOSTMODE = crate::Reg<grxstsp_hostmode::GRXSTSP_HOSTMODE_SPEC>;
#[doc = "Receive Status Read and Pop Register \\[HOSTMODE\\]"]
pub mod grxstsp_hostmode;
#[doc = "GRXFSIZ register accessor: an alias for `Reg<GRXFSIZ_SPEC>`"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ_HOSTMODE register accessor: an alias for `Reg<GNPTXFSIZ_HOSTMODE_SPEC>`"]
pub type GNPTXFSIZ_HOSTMODE = crate::Reg<gnptxfsiz_hostmode::GNPTXFSIZ_HOSTMODE_SPEC>;
#[doc = "Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
pub mod gnptxfsiz_hostmode;
#[doc = "GNPTXFSIZ_DEVICEMODE register accessor: an alias for `Reg<GNPTXFSIZ_DEVICEMODE_SPEC>`"]
pub type GNPTXFSIZ_DEVICEMODE = crate::Reg<gnptxfsiz_devicemode::GNPTXFSIZ_DEVICEMODE_SPEC>;
#[doc = "Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]"]
pub mod gnptxfsiz_devicemode;
#[doc = "GNPTXSTS register accessor: an alias for `Reg<GNPTXSTS_SPEC>`"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = "Non-Periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "GUID register accessor: an alias for `Reg<GUID_SPEC>`"]
pub type GUID = crate::Reg<guid::GUID_SPEC>;
#[doc = "USB Module Identification Register"]
pub mod guid;
#[doc = "GDFIFOCFG register accessor: an alias for `Reg<GDFIFOCFG_SPEC>`"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = "Global DFIFO Software Config Register"]
pub mod gdfifocfg;
#[doc = "HPTXFSIZ register accessor: an alias for `Reg<HPTXFSIZ_SPEC>`"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 register accessor: an alias for `Reg<DIEPTXF1_SPEC>`"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 register accessor: an alias for `Reg<DIEPTXF2_SPEC>`"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 register accessor: an alias for `Reg<DIEPTXF3_SPEC>`"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 register accessor: an alias for `Reg<DIEPTXF4_SPEC>`"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 register accessor: an alias for `Reg<DIEPTXF5_SPEC>`"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 register accessor: an alias for `Reg<DIEPTXF6_SPEC>`"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf6;
#[doc = "HCFG register accessor: an alias for `Reg<HCFG_SPEC>`"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "HFIR register accessor: an alias for `Reg<HFIR_SPEC>`"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "HFNUM register accessor: an alias for `Reg<HFNUM_SPEC>`"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "HPTXSTS register accessor: an alias for `Reg<HPTXSTS_SPEC>`"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "Host Periodic Transmit FIFO/ Queue Status Register"]
pub mod hptxsts;
#[doc = "HAINT register accessor: an alias for `Reg<HAINT_SPEC>`"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "HAINTMSK register accessor: an alias for `Reg<HAINTMSK_SPEC>`"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "HFLBADDR register accessor: an alias for `Reg<HFLBADDR_SPEC>`"]
pub type HFLBADDR = crate::Reg<hflbaddr::HFLBADDR_SPEC>;
#[doc = "Host Frame List Base Address Register"]
pub mod hflbaddr;
#[doc = "HPRT register accessor: an alias for `Reg<HPRT_SPEC>`"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "DCFG register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DVBUSDIS register accessor: an alias for `Reg<DVBUSDIS_SPEC>`"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "DIEPEMPMSK register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "PCGCCTL register accessor: an alias for `Reg<PCGCCTL_SPEC>`"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
