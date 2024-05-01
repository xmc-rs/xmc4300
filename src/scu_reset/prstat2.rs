#[doc = "Register `PRSTAT2` reader"]
pub type R = crate::R<Prstat2Spec>;
#[doc = "WDT Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtrs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Wdtrs> for bool {
    #[inline(always)]
    fn from(variant: Wdtrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRS` reader - WDT Reset Status"]
pub type WdtrsR = crate::BitReader<Wdtrs>;
impl WdtrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtrs {
        match self.bits {
            false => Wdtrs::Const0,
            true => Wdtrs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Wdtrs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Wdtrs::Const1
    }
}
#[doc = "ETH0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0rs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Eth0rs> for bool {
    #[inline(always)]
    fn from(variant: Eth0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0RS` reader - ETH0 Reset Status"]
pub type Eth0rsR = crate::BitReader<Eth0rs>;
impl Eth0rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eth0rs {
        match self.bits {
            false => Eth0rs::Const0,
            true => Eth0rs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Eth0rs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Eth0rs::Const1
    }
}
#[doc = "DMA0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma0rs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Dma0rs> for bool {
    #[inline(always)]
    fn from(variant: Dma0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0RS` reader - DMA0 Reset Status"]
pub type Dma0rsR = crate::BitReader<Dma0rs>;
impl Dma0rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0rs {
        match self.bits {
            false => Dma0rs::Const0,
            true => Dma0rs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Dma0rs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Dma0rs::Const1
    }
}
#[doc = "FCE Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcers {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Fcers> for bool {
    #[inline(always)]
    fn from(variant: Fcers) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCERS` reader - FCE Reset Status"]
pub type FcersR = crate::BitReader<Fcers>;
impl FcersR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcers {
        match self.bits {
            false => Fcers::Const0,
            true => Fcers::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Fcers::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Fcers::Const1
    }
}
#[doc = "USB Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Usbrs> for bool {
    #[inline(always)]
    fn from(variant: Usbrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRS` reader - USB Reset Status"]
pub type UsbrsR = crate::BitReader<Usbrs>;
impl UsbrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbrs {
        match self.bits {
            false => Usbrs::Const0,
            true => Usbrs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Usbrs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Usbrs::Const1
    }
}
#[doc = "ECAT0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecat0rs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Ecat0rs> for bool {
    #[inline(always)]
    fn from(variant: Ecat0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RS` reader - ECAT0 Reset Status"]
pub type Ecat0rsR = crate::BitReader<Ecat0rs>;
impl Ecat0rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecat0rs {
        match self.bits {
            false => Ecat0rs::Const0,
            true => Ecat0rs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ecat0rs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ecat0rs::Const1
    }
}
impl R {
    #[doc = "Bit 1 - WDT Reset Status"]
    #[inline(always)]
    pub fn wdtrs(&self) -> WdtrsR {
        WdtrsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ETH0 Reset Status"]
    #[inline(always)]
    pub fn eth0rs(&self) -> Eth0rsR {
        Eth0rsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA0 Reset Status"]
    #[inline(always)]
    pub fn dma0rs(&self) -> Dma0rsR {
        Dma0rsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - FCE Reset Status"]
    #[inline(always)]
    pub fn fcers(&self) -> FcersR {
        FcersR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Reset Status"]
    #[inline(always)]
    pub fn usbrs(&self) -> UsbrsR {
        UsbrsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - ECAT0 Reset Status"]
    #[inline(always)]
    pub fn ecat0rs(&self) -> Ecat0rsR {
        Ecat0rsR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "RCU Peripheral 2 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstat2Spec;
impl crate::RegisterSpec for Prstat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstat2::R`](R) reader structure"]
impl crate::Readable for Prstat2Spec {}
#[doc = "`reset()` method sets PRSTAT2 to value 0x04f6"]
impl crate::Resettable for Prstat2Spec {
    const RESET_VALUE: u32 = 0x04f6;
}
