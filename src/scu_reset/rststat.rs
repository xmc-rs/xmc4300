#[doc = "Register `RSTSTAT` reader"]
pub type R = crate::R<RststatSpec>;
#[doc = "Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rststat {
    #[doc = "1: PORST reset"]
    Const00000001 = 1,
    #[doc = "2: SWD reset"]
    Const00000010 = 2,
    #[doc = "4: PV reset"]
    Const00000100 = 4,
    #[doc = "8: CPU system reset"]
    Const00001000 = 8,
    #[doc = "16: CPU lockup reset"]
    Const00010000 = 16,
    #[doc = "32: WDT reset"]
    Const00100000 = 32,
    #[doc = "128: Parity Error reset"]
    Const10000000 = 128,
}
impl From<Rststat> for u8 {
    #[inline(always)]
    fn from(variant: Rststat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rststat {
    type Ux = u8;
}
impl crate::IsEnum for Rststat {}
#[doc = "Field `RSTSTAT` reader - Reset Status Information"]
pub type RststatR = crate::FieldReader<Rststat>;
impl RststatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rststat> {
        match self.bits {
            1 => Some(Rststat::Const00000001),
            2 => Some(Rststat::Const00000010),
            4 => Some(Rststat::Const00000100),
            8 => Some(Rststat::Const00001000),
            16 => Some(Rststat::Const00010000),
            32 => Some(Rststat::Const00100000),
            128 => Some(Rststat::Const10000000),
            _ => None,
        }
    }
    #[doc = "PORST reset"]
    #[inline(always)]
    pub fn is_const_00000001(&self) -> bool {
        *self == Rststat::Const00000001
    }
    #[doc = "SWD reset"]
    #[inline(always)]
    pub fn is_const_00000010(&self) -> bool {
        *self == Rststat::Const00000010
    }
    #[doc = "PV reset"]
    #[inline(always)]
    pub fn is_const_00000100(&self) -> bool {
        *self == Rststat::Const00000100
    }
    #[doc = "CPU system reset"]
    #[inline(always)]
    pub fn is_const_00001000(&self) -> bool {
        *self == Rststat::Const00001000
    }
    #[doc = "CPU lockup reset"]
    #[inline(always)]
    pub fn is_const_00010000(&self) -> bool {
        *self == Rststat::Const00010000
    }
    #[doc = "WDT reset"]
    #[inline(always)]
    pub fn is_const_00100000(&self) -> bool {
        *self == Rststat::Const00100000
    }
    #[doc = "Parity Error reset"]
    #[inline(always)]
    pub fn is_const_10000000(&self) -> bool {
        *self == Rststat::Const10000000
    }
}
#[doc = "Hibernate Wake-up Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibwk {
    #[doc = "0: No Wake-up"]
    Const0 = 0,
    #[doc = "1: Wake-up event"]
    Const1 = 1,
}
impl From<Hibwk> for bool {
    #[inline(always)]
    fn from(variant: Hibwk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBWK` reader - Hibernate Wake-up Status"]
pub type HibwkR = crate::BitReader<Hibwk>;
impl HibwkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hibwk {
        match self.bits {
            false => Hibwk::Const0,
            true => Hibwk::Const1,
        }
    }
    #[doc = "No Wake-up"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Hibwk::Const0
    }
    #[doc = "Wake-up event"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Hibwk::Const1
    }
}
#[doc = "Hibernate Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibrs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Hibrs> for bool {
    #[inline(always)]
    fn from(variant: Hibrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBRS` reader - Hibernate Reset Status"]
pub type HibrsR = crate::BitReader<Hibrs>;
impl HibrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hibrs {
        match self.bits {
            false => Hibrs::Const0,
            true => Hibrs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Hibrs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Hibrs::Const1
    }
}
#[doc = "Enable Lockup Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcken {
    #[doc = "0: Reset by Lockup disabled"]
    Const0 = 0,
    #[doc = "1: Reset by Lockup enabled"]
    Const1 = 1,
}
impl From<Lcken> for bool {
    #[inline(always)]
    fn from(variant: Lcken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCKEN` reader - Enable Lockup Status"]
pub type LckenR = crate::BitReader<Lcken>;
impl LckenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcken {
        match self.bits {
            false => Lcken::Const0,
            true => Lcken::Const1,
        }
    }
    #[doc = "Reset by Lockup disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Lcken::Const0
    }
    #[doc = "Reset by Lockup enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Lcken::Const1
    }
}
#[doc = "ECAT0 Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecat0rs {
    #[doc = "0: Reset did not occur"]
    Const0 = 0,
    #[doc = "1: Reset occurred"]
    Const1 = 1,
}
impl From<Ecat0rs> for bool {
    #[inline(always)]
    fn from(variant: Ecat0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RS` reader - ECAT0 Reset Status Information"]
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
    #[doc = "Reset did not occur"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ecat0rs::Const0
    }
    #[doc = "Reset occurred"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ecat0rs::Const1
    }
}
impl R {
    #[doc = "Bits 0:7 - Reset Status Information"]
    #[inline(always)]
    pub fn rststat(&self) -> RststatR {
        RststatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Hibernate Wake-up Status"]
    #[inline(always)]
    pub fn hibwk(&self) -> HibwkR {
        HibwkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Hibernate Reset Status"]
    #[inline(always)]
    pub fn hibrs(&self) -> HibrsR {
        HibrsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Lockup Status"]
    #[inline(always)]
    pub fn lcken(&self) -> LckenR {
        LckenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline(always)]
    pub fn ecat0rs(&self) -> Ecat0rsR {
        Ecat0rsR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "RCU Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rststat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RststatSpec;
impl crate::RegisterSpec for RststatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rststat::R`](R) reader structure"]
impl crate::Readable for RststatSpec {}
#[doc = "`reset()` method sets RSTSTAT to value 0"]
impl crate::Resettable for RststatSpec {
    const RESET_VALUE: u32 = 0;
}
