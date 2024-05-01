#[doc = "Register `PRSTAT1` reader"]
pub type R = crate::R<Prstat1Spec>;
#[doc = "LEDTS Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ledtscu0rs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Ledtscu0rs> for bool {
    #[inline(always)]
    fn from(variant: Ledtscu0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0RS` reader - LEDTS Reset Status"]
pub type Ledtscu0rsR = crate::BitReader<Ledtscu0rs>;
impl Ledtscu0rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ledtscu0rs {
        match self.bits {
            false => Ledtscu0rs::Const0,
            true => Ledtscu0rs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ledtscu0rs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ledtscu0rs::Const1
    }
}
#[doc = "MultiCAN Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcan0rs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Mcan0rs> for bool {
    #[inline(always)]
    fn from(variant: Mcan0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0RS` reader - MultiCAN Reset Status"]
pub type Mcan0rsR = crate::BitReader<Mcan0rs>;
impl Mcan0rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcan0rs {
        match self.bits {
            false => Mcan0rs::Const0,
            true => Mcan0rs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Mcan0rs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Mcan0rs::Const1
    }
}
#[doc = "DAC Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacrs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Dacrs> for bool {
    #[inline(always)]
    fn from(variant: Dacrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRS` reader - DAC Reset Status"]
pub type DacrsR = crate::BitReader<Dacrs>;
impl DacrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacrs {
        match self.bits {
            false => Dacrs::Const0,
            true => Dacrs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Dacrs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Dacrs::Const1
    }
}
#[doc = "MMC Interface Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmcirs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Mmcirs> for bool {
    #[inline(always)]
    fn from(variant: Mmcirs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCIRS` reader - MMC Interface Reset Status"]
pub type MmcirsR = crate::BitReader<Mmcirs>;
impl MmcirsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmcirs {
        match self.bits {
            false => Mmcirs::Const0,
            true => Mmcirs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Mmcirs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Mmcirs::Const1
    }
}
#[doc = "USIC1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic1rs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Usic1rs> for bool {
    #[inline(always)]
    fn from(variant: Usic1rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1RS` reader - USIC1 Reset Status"]
pub type Usic1rsR = crate::BitReader<Usic1rs>;
impl Usic1rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usic1rs {
        match self.bits {
            false => Usic1rs::Const0,
            true => Usic1rs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Usic1rs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Usic1rs::Const1
    }
}
#[doc = "PORTS Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pportsrs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Pportsrs> for bool {
    #[inline(always)]
    fn from(variant: Pportsrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTSRS` reader - PORTS Reset Status"]
pub type PportsrsR = crate::BitReader<Pportsrs>;
impl PportsrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pportsrs {
        match self.bits {
            false => Pportsrs::Const0,
            true => Pportsrs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pportsrs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pportsrs::Const1
    }
}
impl R {
    #[doc = "Bit 3 - LEDTS Reset Status"]
    #[inline(always)]
    pub fn ledtscu0rs(&self) -> Ledtscu0rsR {
        Ledtscu0rsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MultiCAN Reset Status"]
    #[inline(always)]
    pub fn mcan0rs(&self) -> Mcan0rsR {
        Mcan0rsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DAC Reset Status"]
    #[inline(always)]
    pub fn dacrs(&self) -> DacrsR {
        DacrsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Interface Reset Status"]
    #[inline(always)]
    pub fn mmcirs(&self) -> MmcirsR {
        MmcirsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USIC1 Reset Status"]
    #[inline(always)]
    pub fn usic1rs(&self) -> Usic1rsR {
        Usic1rsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - PORTS Reset Status"]
    #[inline(always)]
    pub fn pportsrs(&self) -> PportsrsR {
        PportsrsR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "RCU Peripheral 1 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstat1Spec;
impl crate::RegisterSpec for Prstat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstat1::R`](R) reader structure"]
impl crate::Readable for Prstat1Spec {}
#[doc = "`reset()` method sets PRSTAT1 to value 0x01f9"]
impl crate::Resettable for Prstat1Spec {
    const RESET_VALUE: u32 = 0x01f9;
}
