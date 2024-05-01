#[doc = "Register `CLKSTAT` reader"]
pub type R = crate::R<ClkstatSpec>;
#[doc = "USB Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcst {
    #[doc = "0: Clock disabled"]
    Const0 = 0,
    #[doc = "1: Clock enabled"]
    Const1 = 1,
}
impl From<Usbcst> for bool {
    #[inline(always)]
    fn from(variant: Usbcst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCST` reader - USB Clock Status"]
pub type UsbcstR = crate::BitReader<Usbcst>;
impl UsbcstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbcst {
        match self.bits {
            false => Usbcst::Const0,
            true => Usbcst::Const1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Usbcst::Const0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Usbcst::Const1
    }
}
#[doc = "MMC Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmccst {
    #[doc = "0: Clock disabled"]
    Const0 = 0,
    #[doc = "1: Clock enabled"]
    Const1 = 1,
}
impl From<Mmccst> for bool {
    #[inline(always)]
    fn from(variant: Mmccst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCST` reader - MMC Clock Status"]
pub type MmccstR = crate::BitReader<Mmccst>;
impl MmccstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmccst {
        match self.bits {
            false => Mmccst::Const0,
            true => Mmccst::Const1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Mmccst::Const0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Mmccst::Const1
    }
}
#[doc = "Ethernet Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0cst {
    #[doc = "0: Clock disabled"]
    Const0 = 0,
    #[doc = "1: Clock enabled"]
    Const1 = 1,
}
impl From<Eth0cst> for bool {
    #[inline(always)]
    fn from(variant: Eth0cst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CST` reader - Ethernet Clock Status"]
pub type Eth0cstR = crate::BitReader<Eth0cst>;
impl Eth0cstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eth0cst {
        match self.bits {
            false => Eth0cst::Const0,
            true => Eth0cst::Const1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Eth0cst::Const0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Eth0cst::Const1
    }
}
#[doc = "CCU Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccucst {
    #[doc = "0: Clock disabled"]
    Const0 = 0,
    #[doc = "1: Clock enabled"]
    Const1 = 1,
}
impl From<Ccucst> for bool {
    #[inline(always)]
    fn from(variant: Ccucst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCST` reader - CCU Clock Status"]
pub type CcucstR = crate::BitReader<Ccucst>;
impl CcucstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccucst {
        match self.bits {
            false => Ccucst::Const0,
            true => Ccucst::Const1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ccucst::Const0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ccucst::Const1
    }
}
#[doc = "WDT Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtcst {
    #[doc = "0: Clock disabled"]
    Const0 = 0,
    #[doc = "1: Clock enabled"]
    Const1 = 1,
}
impl From<Wdtcst> for bool {
    #[inline(always)]
    fn from(variant: Wdtcst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCST` reader - WDT Clock Status"]
pub type WdtcstR = crate::BitReader<Wdtcst>;
impl WdtcstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtcst {
        match self.bits {
            false => Wdtcst::Const0,
            true => Wdtcst::Const1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Wdtcst::Const0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Wdtcst::Const1
    }
}
impl R {
    #[doc = "Bit 0 - USB Clock Status"]
    #[inline(always)]
    pub fn usbcst(&self) -> UsbcstR {
        UsbcstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Clock Status"]
    #[inline(always)]
    pub fn mmccst(&self) -> MmccstR {
        MmccstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ethernet Clock Status"]
    #[inline(always)]
    pub fn eth0cst(&self) -> Eth0cstR {
        Eth0cstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CCU Clock Status"]
    #[inline(always)]
    pub fn ccucst(&self) -> CcucstR {
        CcucstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WDT Clock Status"]
    #[inline(always)]
    pub fn wdtcst(&self) -> WdtcstR {
        WdtcstR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Clock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkstatSpec;
impl crate::RegisterSpec for ClkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkstat::R`](R) reader structure"]
impl crate::Readable for ClkstatSpec {}
#[doc = "`reset()` method sets CLKSTAT to value 0"]
impl crate::Resettable for ClkstatSpec {
    const RESET_VALUE: u32 = 0;
}
