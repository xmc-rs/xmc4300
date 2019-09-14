#[doc = r"Register block"]
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
    _reserved_5_gintsts: [u8; 4usize],
    _reserved_6_gintmsk: [u8; 4usize],
    _reserved_7_grxstsr: [u8; 4usize],
    _reserved_8_grxstsp: [u8; 4usize],
    #[doc = "0x24 - Receive FIFO Size Register"]
    pub grxfsiz: GRXFSIZ,
    _reserved_10_gnptxfsiz: [u8; 4usize],
    #[doc = "0x2c - Non-Periodic Transmit FIFO/Queue Status Register"]
    pub gnptxsts: GNPTXSTS,
    _reserved12: [u8; 12usize],
    #[doc = "0x3c - USB Module Identification Register"]
    pub guid: GUID,
    _reserved13: [u8; 28usize],
    #[doc = "0x5c - Global DFIFO Software Config Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved14: [u8; 160usize],
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
    _reserved21: [u8; 740usize],
    #[doc = "0x400 - Host Configuration Register"]
    pub hcfg: HCFG,
    #[doc = "0x404 - Host Frame Interval Register"]
    pub hfir: HFIR,
    #[doc = "0x408 - Host Frame Number/Frame Time Remaining Register"]
    pub hfnum: HFNUM,
    _reserved24: [u8; 4usize],
    #[doc = "0x410 - Host Periodic Transmit FIFO/ Queue Status Register"]
    pub hptxsts: HPTXSTS,
    #[doc = "0x414 - Host All Channels Interrupt Register"]
    pub haint: HAINT,
    #[doc = "0x418 - Host All Channels Interrupt Mask Register"]
    pub haintmsk: HAINTMSK,
    #[doc = "0x41c - Host Frame List Base Address Register"]
    pub hflbaddr: HFLBADDR,
    _reserved28: [u8; 32usize],
    #[doc = "0x440 - Host Port Control and Status Register"]
    pub hprt: HPRT,
    _reserved29: [u8; 956usize],
    #[doc = "0x800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0x804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0x808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved32: [u8; 4usize],
    #[doc = "0x810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0x81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved36: [u8; 8usize],
    #[doc = "0x828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: DVBUSPULSE,
    _reserved38: [u8; 4usize],
    #[doc = "0x834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved39: [u8; 1480usize],
    #[doc = "0xe00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    #[doc = "0x14 - Interrupt Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn gintsts_devicemode(&self) -> &GINTSTS_DEVICEMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const GINTSTS_DEVICEMODE) }
    }
    #[doc = "0x14 - Interrupt Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn gintsts_devicemode_mut(&self) -> &mut GINTSTS_DEVICEMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut GINTSTS_DEVICEMODE) }
    }
    #[doc = "0x14 - Interrupt Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn gintsts_hostmode(&self) -> &GINTSTS_HOSTMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const GINTSTS_HOSTMODE) }
    }
    #[doc = "0x14 - Interrupt Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn gintsts_hostmode_mut(&self) -> &mut GINTSTS_HOSTMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut GINTSTS_HOSTMODE) }
    }
    #[doc = "0x18 - Interrupt Mask Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn gintmsk_devicemode(&self) -> &GINTMSK_DEVICEMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const GINTMSK_DEVICEMODE) }
    }
    #[doc = "0x18 - Interrupt Mask Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn gintmsk_devicemode_mut(&self) -> &mut GINTMSK_DEVICEMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut GINTMSK_DEVICEMODE) }
    }
    #[doc = "0x18 - Interrupt Mask Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn gintmsk_hostmode(&self) -> &GINTMSK_HOSTMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const GINTMSK_HOSTMODE) }
    }
    #[doc = "0x18 - Interrupt Mask Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn gintmsk_hostmode_mut(&self) -> &mut GINTMSK_HOSTMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut GINTMSK_HOSTMODE) }
    }
    #[doc = "0x1c - Receive Status Debug Read Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn grxstsr_devicemode(&self) -> &GRXSTSR_DEVICEMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const GRXSTSR_DEVICEMODE) }
    }
    #[doc = "0x1c - Receive Status Debug Read Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn grxstsr_devicemode_mut(&self) -> &mut GRXSTSR_DEVICEMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut GRXSTSR_DEVICEMODE) }
    }
    #[doc = "0x1c - Receive Status Debug Read Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn grxstsr_hostmode(&self) -> &GRXSTSR_HOSTMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const GRXSTSR_HOSTMODE) }
    }
    #[doc = "0x1c - Receive Status Debug Read Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn grxstsr_hostmode_mut(&self) -> &mut GRXSTSR_HOSTMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut GRXSTSR_HOSTMODE) }
    }
    #[doc = "0x20 - Receive Status Read and Pop Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn grxstsp_hostmode(&self) -> &GRXSTSP_HOSTMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const GRXSTSP_HOSTMODE) }
    }
    #[doc = "0x20 - Receive Status Read and Pop Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn grxstsp_hostmode_mut(&self) -> &mut GRXSTSP_HOSTMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut GRXSTSP_HOSTMODE) }
    }
    #[doc = "0x20 - Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn grxstsp_devicemode(&self) -> &GRXSTSP_DEVICEMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const GRXSTSP_DEVICEMODE) }
    }
    #[doc = "0x20 - Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn grxstsp_devicemode_mut(&self) -> &mut GRXSTSP_DEVICEMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut GRXSTSP_DEVICEMODE) }
    }
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn gnptxfsiz_devicemode(&self) -> &GNPTXFSIZ_DEVICEMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const GNPTXFSIZ_DEVICEMODE) }
    }
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]"]
    #[inline(always)]
    pub fn gnptxfsiz_devicemode_mut(&self) -> &mut GNPTXFSIZ_DEVICEMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut GNPTXFSIZ_DEVICEMODE) }
    }
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn gnptxfsiz_hostmode(&self) -> &GNPTXFSIZ_HOSTMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const GNPTXFSIZ_HOSTMODE) }
    }
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
    #[inline(always)]
    pub fn gnptxfsiz_hostmode_mut(&self) -> &mut GNPTXFSIZ_HOSTMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut GNPTXFSIZ_HOSTMODE) }
    }
}
#[doc = "Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gotgctl](gotgctl) module"]
pub type GOTGCTL = crate::Reg<u32, _GOTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GOTGCTL;
#[doc = "`read()` method returns [gotgctl::R](gotgctl::R) reader structure"]
impl crate::Readable for GOTGCTL {}
#[doc = "`write(|w| ..)` method takes [gotgctl::W](gotgctl::W) writer structure"]
impl crate::Writable for GOTGCTL {}
#[doc = "Control and Status Register"]
pub mod gotgctl;
#[doc = "OTG Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gotgint](gotgint) module"]
pub type GOTGINT = crate::Reg<u32, _GOTGINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GOTGINT;
#[doc = "`read()` method returns [gotgint::R](gotgint::R) reader structure"]
impl crate::Readable for GOTGINT {}
#[doc = "`write(|w| ..)` method takes [gotgint::W](gotgint::W) writer structure"]
impl crate::Writable for GOTGINT {}
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "AHB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gahbcfg](gahbcfg) module"]
pub type GAHBCFG = crate::Reg<u32, _GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAHBCFG;
#[doc = "`read()` method returns [gahbcfg::R](gahbcfg::R) reader structure"]
impl crate::Readable for GAHBCFG {}
#[doc = "`write(|w| ..)` method takes [gahbcfg::W](gahbcfg::W) writer structure"]
impl crate::Writable for GAHBCFG {}
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "USB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gusbcfg](gusbcfg) module"]
pub type GUSBCFG = crate::Reg<u32, _GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GUSBCFG;
#[doc = "`read()` method returns [gusbcfg::R](gusbcfg::R) reader structure"]
impl crate::Readable for GUSBCFG {}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](gusbcfg::W) writer structure"]
impl crate::Writable for GUSBCFG {}
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grstctl](grstctl) module"]
pub type GRSTCTL = crate::Reg<u32, _GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSTCTL;
#[doc = "`read()` method returns [grstctl::R](grstctl::R) reader structure"]
impl crate::Readable for GRSTCTL {}
#[doc = "`write(|w| ..)` method takes [grstctl::W](grstctl::W) writer structure"]
impl crate::Writable for GRSTCTL {}
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "Interrupt Register \\[HOSTMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gintsts_hostmode](gintsts_hostmode) module"]
pub type GINTSTS_HOSTMODE = crate::Reg<u32, _GINTSTS_HOSTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTSTS_HOSTMODE;
#[doc = "`read()` method returns [gintsts_hostmode::R](gintsts_hostmode::R) reader structure"]
impl crate::Readable for GINTSTS_HOSTMODE {}
#[doc = "`write(|w| ..)` method takes [gintsts_hostmode::W](gintsts_hostmode::W) writer structure"]
impl crate::Writable for GINTSTS_HOSTMODE {}
#[doc = "Interrupt Register \\[HOSTMODE\\]"]
pub mod gintsts_hostmode;
#[doc = "Interrupt Register \\[DEVICEMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gintsts_devicemode](gintsts_devicemode) module"]
pub type GINTSTS_DEVICEMODE = crate::Reg<u32, _GINTSTS_DEVICEMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTSTS_DEVICEMODE;
#[doc = "`read()` method returns [gintsts_devicemode::R](gintsts_devicemode::R) reader structure"]
impl crate::Readable for GINTSTS_DEVICEMODE {}
#[doc = "`write(|w| ..)` method takes [gintsts_devicemode::W](gintsts_devicemode::W) writer structure"]
impl crate::Writable for GINTSTS_DEVICEMODE {}
#[doc = "Interrupt Register \\[DEVICEMODE\\]"]
pub mod gintsts_devicemode;
#[doc = "Interrupt Mask Register \\[HOSTMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gintmsk_hostmode](gintmsk_hostmode) module"]
pub type GINTMSK_HOSTMODE = crate::Reg<u32, _GINTMSK_HOSTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTMSK_HOSTMODE;
#[doc = "`read()` method returns [gintmsk_hostmode::R](gintmsk_hostmode::R) reader structure"]
impl crate::Readable for GINTMSK_HOSTMODE {}
#[doc = "`write(|w| ..)` method takes [gintmsk_hostmode::W](gintmsk_hostmode::W) writer structure"]
impl crate::Writable for GINTMSK_HOSTMODE {}
#[doc = "Interrupt Mask Register \\[HOSTMODE\\]"]
pub mod gintmsk_hostmode;
#[doc = "Interrupt Mask Register \\[DEVICEMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gintmsk_devicemode](gintmsk_devicemode) module"]
pub type GINTMSK_DEVICEMODE = crate::Reg<u32, _GINTMSK_DEVICEMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTMSK_DEVICEMODE;
#[doc = "`read()` method returns [gintmsk_devicemode::R](gintmsk_devicemode::R) reader structure"]
impl crate::Readable for GINTMSK_DEVICEMODE {}
#[doc = "`write(|w| ..)` method takes [gintmsk_devicemode::W](gintmsk_devicemode::W) writer structure"]
impl crate::Writable for GINTMSK_DEVICEMODE {}
#[doc = "Interrupt Mask Register \\[DEVICEMODE\\]"]
pub mod gintmsk_devicemode;
#[doc = "Receive Status Debug Read Register \\[HOSTMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grxstsr_hostmode](grxstsr_hostmode) module"]
pub type GRXSTSR_HOSTMODE = crate::Reg<u32, _GRXSTSR_HOSTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSR_HOSTMODE;
#[doc = "`read()` method returns [grxstsr_hostmode::R](grxstsr_hostmode::R) reader structure"]
impl crate::Readable for GRXSTSR_HOSTMODE {}
#[doc = "Receive Status Debug Read Register \\[HOSTMODE\\]"]
pub mod grxstsr_hostmode;
#[doc = "Receive Status Debug Read Register \\[DEVICEMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grxstsr_devicemode](grxstsr_devicemode) module"]
pub type GRXSTSR_DEVICEMODE = crate::Reg<u32, _GRXSTSR_DEVICEMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSR_DEVICEMODE;
#[doc = "`read()` method returns [grxstsr_devicemode::R](grxstsr_devicemode::R) reader structure"]
impl crate::Readable for GRXSTSR_DEVICEMODE {}
#[doc = "Receive Status Debug Read Register \\[DEVICEMODE\\]"]
pub mod grxstsr_devicemode;
#[doc = "Receive Status Read and Pop Register \\[DEVICEMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grxstsp_devicemode](grxstsp_devicemode) module"]
pub type GRXSTSP_DEVICEMODE = crate::Reg<u32, _GRXSTSP_DEVICEMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSP_DEVICEMODE;
#[doc = "`read()` method returns [grxstsp_devicemode::R](grxstsp_devicemode::R) reader structure"]
impl crate::Readable for GRXSTSP_DEVICEMODE {}
#[doc = "Receive Status Read and Pop Register \\[DEVICEMODE\\]"]
pub mod grxstsp_devicemode;
#[doc = "Receive Status Read and Pop Register \\[HOSTMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grxstsp_hostmode](grxstsp_hostmode) module"]
pub type GRXSTSP_HOSTMODE = crate::Reg<u32, _GRXSTSP_HOSTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSP_HOSTMODE;
#[doc = "`read()` method returns [grxstsp_hostmode::R](grxstsp_hostmode::R) reader structure"]
impl crate::Readable for GRXSTSP_HOSTMODE {}
#[doc = "Receive Status Read and Pop Register \\[HOSTMODE\\]"]
pub mod grxstsp_hostmode;
#[doc = "Receive FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grxfsiz](grxfsiz) module"]
pub type GRXFSIZ = crate::Reg<u32, _GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXFSIZ;
#[doc = "`read()` method returns [grxfsiz::R](grxfsiz::R) reader structure"]
impl crate::Readable for GRXFSIZ {}
#[doc = "`write(|w| ..)` method takes [grxfsiz::W](grxfsiz::W) writer structure"]
impl crate::Writable for GRXFSIZ {}
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gnptxfsiz_hostmode](gnptxfsiz_hostmode) module"]
pub type GNPTXFSIZ_HOSTMODE = crate::Reg<u32, _GNPTXFSIZ_HOSTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GNPTXFSIZ_HOSTMODE;
#[doc = "`read()` method returns [gnptxfsiz_hostmode::R](gnptxfsiz_hostmode::R) reader structure"]
impl crate::Readable for GNPTXFSIZ_HOSTMODE {}
#[doc = "`write(|w| ..)` method takes [gnptxfsiz_hostmode::W](gnptxfsiz_hostmode::W) writer structure"]
impl crate::Writable for GNPTXFSIZ_HOSTMODE {}
#[doc = "Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]"]
pub mod gnptxfsiz_hostmode;
#[doc = "Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gnptxfsiz_devicemode](gnptxfsiz_devicemode) module"]
pub type GNPTXFSIZ_DEVICEMODE = crate::Reg<u32, _GNPTXFSIZ_DEVICEMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GNPTXFSIZ_DEVICEMODE;
#[doc = "`read()` method returns [gnptxfsiz_devicemode::R](gnptxfsiz_devicemode::R) reader structure"]
impl crate::Readable for GNPTXFSIZ_DEVICEMODE {}
#[doc = "`write(|w| ..)` method takes [gnptxfsiz_devicemode::W](gnptxfsiz_devicemode::W) writer structure"]
impl crate::Writable for GNPTXFSIZ_DEVICEMODE {}
#[doc = "Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]"]
pub mod gnptxfsiz_devicemode;
#[doc = "Non-Periodic Transmit FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gnptxsts](gnptxsts) module"]
pub type GNPTXSTS = crate::Reg<u32, _GNPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GNPTXSTS;
#[doc = "`read()` method returns [gnptxsts::R](gnptxsts::R) reader structure"]
impl crate::Readable for GNPTXSTS {}
#[doc = "Non-Periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "USB Module Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [guid](guid) module"]
pub type GUID = crate::Reg<u32, _GUID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GUID;
#[doc = "`read()` method returns [guid::R](guid::R) reader structure"]
impl crate::Readable for GUID {}
#[doc = "`write(|w| ..)` method takes [guid::W](guid::W) writer structure"]
impl crate::Writable for GUID {}
#[doc = "USB Module Identification Register"]
pub mod guid;
#[doc = "Global DFIFO Software Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gdfifocfg](gdfifocfg) module"]
pub type GDFIFOCFG = crate::Reg<u32, _GDFIFOCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GDFIFOCFG;
#[doc = "`read()` method returns [gdfifocfg::R](gdfifocfg::R) reader structure"]
impl crate::Readable for GDFIFOCFG {}
#[doc = "`write(|w| ..)` method takes [gdfifocfg::W](gdfifocfg::W) writer structure"]
impl crate::Writable for GDFIFOCFG {}
#[doc = "Global DFIFO Software Config Register"]
pub mod gdfifocfg;
#[doc = "Host Periodic Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hptxfsiz](hptxfsiz) module"]
pub type HPTXFSIZ = crate::Reg<u32, _HPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTXFSIZ;
#[doc = "`read()` method returns [hptxfsiz::R](hptxfsiz::R) reader structure"]
impl crate::Readable for HPTXFSIZ {}
#[doc = "`write(|w| ..)` method takes [hptxfsiz::W](hptxfsiz::W) writer structure"]
impl crate::Writable for HPTXFSIZ {}
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf1](dieptxf1) module"]
pub type DIEPTXF1 = crate::Reg<u32, _DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF1;
#[doc = "`read()` method returns [dieptxf1::R](dieptxf1::R) reader structure"]
impl crate::Readable for DIEPTXF1 {}
#[doc = "`write(|w| ..)` method takes [dieptxf1::W](dieptxf1::W) writer structure"]
impl crate::Writable for DIEPTXF1 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf1;
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf2](dieptxf2) module"]
pub type DIEPTXF2 = crate::Reg<u32, _DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF2;
#[doc = "`read()` method returns [dieptxf2::R](dieptxf2::R) reader structure"]
impl crate::Readable for DIEPTXF2 {}
#[doc = "`write(|w| ..)` method takes [dieptxf2::W](dieptxf2::W) writer structure"]
impl crate::Writable for DIEPTXF2 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf2;
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf3](dieptxf3) module"]
pub type DIEPTXF3 = crate::Reg<u32, _DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF3;
#[doc = "`read()` method returns [dieptxf3::R](dieptxf3::R) reader structure"]
impl crate::Readable for DIEPTXF3 {}
#[doc = "`write(|w| ..)` method takes [dieptxf3::W](dieptxf3::W) writer structure"]
impl crate::Writable for DIEPTXF3 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf3;
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf4](dieptxf4) module"]
pub type DIEPTXF4 = crate::Reg<u32, _DIEPTXF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF4;
#[doc = "`read()` method returns [dieptxf4::R](dieptxf4::R) reader structure"]
impl crate::Readable for DIEPTXF4 {}
#[doc = "`write(|w| ..)` method takes [dieptxf4::W](dieptxf4::W) writer structure"]
impl crate::Writable for DIEPTXF4 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf4;
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf5](dieptxf5) module"]
pub type DIEPTXF5 = crate::Reg<u32, _DIEPTXF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF5;
#[doc = "`read()` method returns [dieptxf5::R](dieptxf5::R) reader structure"]
impl crate::Readable for DIEPTXF5 {}
#[doc = "`write(|w| ..)` method takes [dieptxf5::W](dieptxf5::W) writer structure"]
impl crate::Writable for DIEPTXF5 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf5;
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf6](dieptxf6) module"]
pub type DIEPTXF6 = crate::Reg<u32, _DIEPTXF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF6;
#[doc = "`read()` method returns [dieptxf6::R](dieptxf6::R) reader structure"]
impl crate::Readable for DIEPTXF6 {}
#[doc = "`write(|w| ..)` method takes [dieptxf6::W](dieptxf6::W) writer structure"]
impl crate::Writable for DIEPTXF6 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register"]
pub mod dieptxf6;
#[doc = "Host Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hcfg](hcfg) module"]
pub type HCFG = crate::Reg<u32, _HCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCFG;
#[doc = "`read()` method returns [hcfg::R](hcfg::R) reader structure"]
impl crate::Readable for HCFG {}
#[doc = "`write(|w| ..)` method takes [hcfg::W](hcfg::W) writer structure"]
impl crate::Writable for HCFG {}
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "Host Frame Interval Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfir](hfir) module"]
pub type HFIR = crate::Reg<u32, _HFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFIR;
#[doc = "`read()` method returns [hfir::R](hfir::R) reader structure"]
impl crate::Readable for HFIR {}
#[doc = "`write(|w| ..)` method takes [hfir::W](hfir::W) writer structure"]
impl crate::Writable for HFIR {}
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "Host Frame Number/Frame Time Remaining Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfnum](hfnum) module"]
pub type HFNUM = crate::Reg<u32, _HFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFNUM;
#[doc = "`read()` method returns [hfnum::R](hfnum::R) reader structure"]
impl crate::Readable for HFNUM {}
#[doc = "`write(|w| ..)` method takes [hfnum::W](hfnum::W) writer structure"]
impl crate::Writable for HFNUM {}
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "Host Periodic Transmit FIFO/ Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hptxsts](hptxsts) module"]
pub type HPTXSTS = crate::Reg<u32, _HPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTXSTS;
#[doc = "`read()` method returns [hptxsts::R](hptxsts::R) reader structure"]
impl crate::Readable for HPTXSTS {}
#[doc = "`write(|w| ..)` method takes [hptxsts::W](hptxsts::W) writer structure"]
impl crate::Writable for HPTXSTS {}
#[doc = "Host Periodic Transmit FIFO/ Queue Status Register"]
pub mod hptxsts;
#[doc = "Host All Channels Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [haint](haint) module"]
pub type HAINT = crate::Reg<u32, _HAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HAINT;
#[doc = "`read()` method returns [haint::R](haint::R) reader structure"]
impl crate::Readable for HAINT {}
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "Host All Channels Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [haintmsk](haintmsk) module"]
pub type HAINTMSK = crate::Reg<u32, _HAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HAINTMSK;
#[doc = "`read()` method returns [haintmsk::R](haintmsk::R) reader structure"]
impl crate::Readable for HAINTMSK {}
#[doc = "`write(|w| ..)` method takes [haintmsk::W](haintmsk::W) writer structure"]
impl crate::Writable for HAINTMSK {}
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "Host Frame List Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hflbaddr](hflbaddr) module"]
pub type HFLBADDR = crate::Reg<u32, _HFLBADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFLBADDR;
#[doc = "`read()` method returns [hflbaddr::R](hflbaddr::R) reader structure"]
impl crate::Readable for HFLBADDR {}
#[doc = "`write(|w| ..)` method takes [hflbaddr::W](hflbaddr::W) writer structure"]
impl crate::Writable for HFLBADDR {}
#[doc = "Host Frame List Base Address Register"]
pub mod hflbaddr;
#[doc = "Host Port Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hprt](hprt) module"]
pub type HPRT = crate::Reg<u32, _HPRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPRT;
#[doc = "`read()` method returns [hprt::R](hprt::R) reader structure"]
impl crate::Readable for HPRT {}
#[doc = "`write(|w| ..)` method takes [hprt::W](hprt::W) writer structure"]
impl crate::Writable for HPRT {}
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "Device Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcfg](dcfg) module"]
pub type DCFG = crate::Reg<u32, _DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFG;
#[doc = "`read()` method returns [dcfg::R](dcfg::R) reader structure"]
impl crate::Readable for DCFG {}
#[doc = "`write(|w| ..)` method takes [dcfg::W](dcfg::W) writer structure"]
impl crate::Writable for DCFG {}
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "Device Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dctl](dctl) module"]
pub type DCTL = crate::Reg<u32, _DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTL;
#[doc = "`read()` method returns [dctl::R](dctl::R) reader structure"]
impl crate::Readable for DCTL {}
#[doc = "`write(|w| ..)` method takes [dctl::W](dctl::W) writer structure"]
impl crate::Writable for DCTL {}
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "Device Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dsts](dsts) module"]
pub type DSTS = crate::Reg<u32, _DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSTS;
#[doc = "`read()` method returns [dsts::R](dsts::R) reader structure"]
impl crate::Readable for DSTS {}
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "Device IN Endpoint Common Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepmsk](diepmsk) module"]
pub type DIEPMSK = crate::Reg<u32, _DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPMSK;
#[doc = "`read()` method returns [diepmsk::R](diepmsk::R) reader structure"]
impl crate::Readable for DIEPMSK {}
#[doc = "`write(|w| ..)` method takes [diepmsk::W](diepmsk::W) writer structure"]
impl crate::Writable for DIEPMSK {}
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepmsk](doepmsk) module"]
pub type DOEPMSK = crate::Reg<u32, _DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPMSK;
#[doc = "`read()` method returns [doepmsk::R](doepmsk::R) reader structure"]
impl crate::Readable for DOEPMSK {}
#[doc = "`write(|w| ..)` method takes [doepmsk::W](doepmsk::W) writer structure"]
impl crate::Writable for DOEPMSK {}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "Device All Endpoints Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daint](daint) module"]
pub type DAINT = crate::Reg<u32, _DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINT;
#[doc = "`read()` method returns [daint::R](daint::R) reader structure"]
impl crate::Readable for DAINT {}
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "Device All Endpoints Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daintmsk](daintmsk) module"]
pub type DAINTMSK = crate::Reg<u32, _DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINTMSK;
#[doc = "`read()` method returns [daintmsk::R](daintmsk::R) reader structure"]
impl crate::Readable for DAINTMSK {}
#[doc = "`write(|w| ..)` method takes [daintmsk::W](daintmsk::W) writer structure"]
impl crate::Writable for DAINTMSK {}
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "Device VBUS Discharge Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dvbusdis](dvbusdis) module"]
pub type DVBUSDIS = crate::Reg<u32, _DVBUSDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSDIS;
#[doc = "`read()` method returns [dvbusdis::R](dvbusdis::R) reader structure"]
impl crate::Readable for DVBUSDIS {}
#[doc = "`write(|w| ..)` method takes [dvbusdis::W](dvbusdis::W) writer structure"]
impl crate::Writable for DVBUSDIS {}
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "Device VBUS Pulsing Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dvbuspulse](dvbuspulse) module"]
pub type DVBUSPULSE = crate::Reg<u32, _DVBUSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSPULSE;
#[doc = "`read()` method returns [dvbuspulse::R](dvbuspulse::R) reader structure"]
impl crate::Readable for DVBUSPULSE {}
#[doc = "`write(|w| ..)` method takes [dvbuspulse::W](dvbuspulse::W) writer structure"]
impl crate::Writable for DVBUSPULSE {}
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepempmsk](diepempmsk) module"]
pub type DIEPEMPMSK = crate::Reg<u32, _DIEPEMPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPEMPMSK;
#[doc = "`read()` method returns [diepempmsk::R](diepempmsk::R) reader structure"]
impl crate::Readable for DIEPEMPMSK {}
#[doc = "`write(|w| ..)` method takes [diepempmsk::W](diepempmsk::W) writer structure"]
impl crate::Writable for DIEPEMPMSK {}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "Power and Clock Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcgcctl](pcgcctl) module"]
pub type PCGCCTL = crate::Reg<u32, _PCGCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCGCCTL;
#[doc = "`read()` method returns [pcgcctl::R](pcgcctl::R) reader structure"]
impl crate::Readable for PCGCCTL {}
#[doc = "`write(|w| ..)` method takes [pcgcctl::W](pcgcctl::W) writer structure"]
impl crate::Writable for PCGCCTL {}
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
