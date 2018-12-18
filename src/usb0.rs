#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control and Status Register"]
    pub gotgctl: GOTGCTL,
    #[doc = "0x04 - OTG Interrupt Register"]
    pub gotgint: GOTGINT,
    #[doc = "0x08 - AHB Configuration Register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x0c - USB Configuration Register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x10 - Reset Register"]
    pub grstctl: GRSTCTL,
    #[doc = "0x14 - Interrupt Register \\[HOSTMODE\\]"]
    pub gintsts_hostmode: GINTSTS_HOSTMODE,
    #[doc = "0x18 - Interrupt Mask Register \\[HOSTMODE\\]"]
    pub gintmsk_hostmode: GINTMSK_HOSTMODE,
    #[doc = "0x1c - Receive Status Debug Read Register \\[HOSTMODE\\]"]
    pub grxstsr_hostmode: GRXSTSR_HOSTMODE,
    #[doc = "0x20 - Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
    pub grxstsp_devicemode: GRXSTSP_DEVICEMODE,
    #[doc = "0x24 - Receive FIFO Size Register"]
    pub grxfsiz: GRXFSIZ,
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
    pub gnptxfsiz_hostmode: GNPTXFSIZ_HOSTMODE,
    #[doc = "0x2c - Non-Periodic Transmit FIFO/Queue Status Register"]
    pub gnptxsts: GNPTXSTS,
    _reserved0: [u8; 12usize],
    #[doc = "0x3c - USB Module Identification Register"]
    pub guid: GUID,
    _reserved1: [u8; 28usize],
    #[doc = "0x5c - Global DFIFO Software Config Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved2: [u8; 160usize],
    #[doc = "0x100 - Host Periodic Transmit FIFO Size Register"]
    pub hptxfsiz: HPTXFSIZ,
    #[doc = "0x104 - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x108 - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0x10c - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0x110 - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0x114 - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf5: DIEPTXF5,
    #[doc = "0x118 - Device IN Endpoint Transmit FIFO Size Register"]
    pub dieptxf6: DIEPTXF6,
    _reserved3: [u8; 740usize],
    #[doc = "0x400 - Host Configuration Register"]
    pub hcfg: HCFG,
    #[doc = "0x404 - Host Frame Interval Register"]
    pub hfir: HFIR,
    #[doc = "0x408 - Host Frame Number/Frame Time Remaining Register"]
    pub hfnum: HFNUM,
    _reserved4: [u8; 4usize],
    #[doc = "0x410 - Host Periodic Transmit FIFO/ Queue Status Register"]
    pub hptxsts: HPTXSTS,
    #[doc = "0x414 - Host All Channels Interrupt Register"]
    pub haint: HAINT,
    #[doc = "0x418 - Host All Channels Interrupt Mask Register"]
    pub haintmsk: HAINTMSK,
    #[doc = "0x41c - Host Frame List Base Address Register"]
    pub hflbaddr: HFLBADDR,
    _reserved5: [u8; 32usize],
    #[doc = "0x440 - Host Port Control and Status Register"]
    pub hprt: HPRT,
    _reserved6: [u8; 956usize],
    #[doc = "0x800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0x804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0x808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved7: [u8; 4usize],
    #[doc = "0x810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0x81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved8: [u8; 8usize],
    #[doc = "0x828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: DVBUSPULSE,
    _reserved9: [u8; 4usize],
    #[doc = "0x834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved10: [u8; 1480usize],
    #[doc = "0xe00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "Control and Status Register"]
pub struct GOTGCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control and Status Register"]
pub mod gotgctl;
#[doc = "OTG Interrupt Register"]
pub struct GOTGINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "AHB Configuration Register"]
pub struct GAHBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "USB Configuration Register"]
pub struct GUSBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "Reset Register"]
pub struct GRSTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "Interrupt Register \\[HOSTMODE\\]"]
pub struct GINTSTS_HOSTMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register \\[HOSTMODE\\]"]
pub mod gintsts_hostmode;
#[doc = "Interrupt Register \\[DEVICEMODE\\]"]
pub struct GINTSTS_DEVICEMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register \\[DEVICEMODE\\]"]
pub mod gintsts_devicemode;
#[doc = "Interrupt Mask Register \\[HOSTMODE\\]"]
pub struct GINTMSK_HOSTMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register \\[HOSTMODE\\]"]
pub mod gintmsk_hostmode;
#[doc = "Interrupt Mask Register \\[DEVICEMODE\\]"]
pub struct GINTMSK_DEVICEMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register \\[DEVICEMODE\\]"]
pub mod gintmsk_devicemode;
#[doc = "Receive Status Debug Read Register \\[HOSTMODE\\]"]
pub struct GRXSTSR_HOSTMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Status Debug Read Register \\[HOSTMODE\\]"]
pub mod grxstsr_hostmode;
#[doc = "Receive Status Debug Read Register \\[DEVICEMODE\\]"]
pub struct GRXSTSR_DEVICEMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Status Debug Read Register \\[DEVICEMODE\\]"]
pub mod grxstsr_devicemode;
#[doc = "Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
pub struct GRXSTSP_DEVICEMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
pub mod grxstsp_devicemode;
#[doc = "Receive Status Read and Pop Register \\[HOSTMODE\\]"]
pub struct GRXSTSP_HOSTMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Status Read and Pop Register \\[HOSTMODE\\]"]
pub mod grxstsp_hostmode;
#[doc = "Receive FIFO Size Register"]
pub struct GRXFSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
pub struct GNPTXFSIZ_HOSTMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
pub mod gnptxfsiz_hostmode;
#[doc = "Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]"]
pub struct GNPTXFSIZ_DEVICEMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]"]
pub mod gnptxfsiz_devicemode;
#[doc = "Non-Periodic Transmit FIFO/Queue Status Register"]
pub struct GNPTXSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-Periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "USB Module Identification Register"]
pub struct GUID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Module Identification Register"]
pub mod guid;
#[doc = "Global DFIFO Software Config Register"]
pub struct GDFIFOCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global DFIFO Software Config Register"]
pub mod gdfifocfg;
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub struct HPTXFSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub struct DIEPTXF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf1;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub struct DIEPTXF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf2;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub struct DIEPTXF3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf3;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub struct DIEPTXF4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf4;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub struct DIEPTXF5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf5;
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub struct DIEPTXF6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf6;
#[doc = "Host Configuration Register"]
pub struct HCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "Host Frame Interval Register"]
pub struct HFIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub struct HFNUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "Host Periodic Transmit FIFO/ Queue Status Register"]
pub struct HPTXSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Periodic Transmit FIFO/ Queue Status Register"]
pub mod hptxsts;
#[doc = "Host All Channels Interrupt Register"]
pub struct HAINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "Host All Channels Interrupt Mask Register"]
pub struct HAINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "Host Frame List Base Address Register"]
pub struct HFLBADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Frame List Base Address Register"]
pub mod hflbaddr;
#[doc = "Host Port Control and Status Register"]
pub struct HPRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "Device Configuration Register"]
pub struct DCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "Device Control Register"]
pub struct DCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "Device Status Register"]
pub struct DSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub struct DIEPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub struct DOEPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "Device All Endpoints Interrupt Register"]
pub struct DAINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub struct DAINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "Device VBUS Discharge Time Register"]
pub struct DVBUSDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "Device VBUS Pulsing Time Register"]
pub struct DVBUSPULSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub struct DIEPEMPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "Power and Clock Gating Control Register"]
pub struct PCGCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
