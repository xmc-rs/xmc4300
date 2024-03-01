#[doc = "Register `PRSTAT0` reader"]
pub type R = crate::R<Prstat0Spec>;
#[doc = "VADC Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vadcrs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Vadcrs> for bool {
    #[inline(always)]
    fn from(variant: Vadcrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADCRS` reader - VADC Reset Status"]
pub type VadcrsR = crate::BitReader<Vadcrs>;
impl VadcrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vadcrs {
        match self.bits {
            false => Vadcrs::Const0,
            true => Vadcrs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Vadcrs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Vadcrs::Const1
    }
}
#[doc = "CCU40 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu40rs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Ccu40rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu40rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40RS` reader - CCU40 Reset Status"]
pub type Ccu40rsR = crate::BitReader<Ccu40rs>;
impl Ccu40rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu40rs {
        match self.bits {
            false => Ccu40rs::Const0,
            true => Ccu40rs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ccu40rs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ccu40rs::Const1
    }
}
#[doc = "CCU41 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu41rs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Ccu41rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu41rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41RS` reader - CCU41 Reset Status"]
pub type Ccu41rsR = crate::BitReader<Ccu41rs>;
impl Ccu41rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu41rs {
        match self.bits {
            false => Ccu41rs::Const0,
            true => Ccu41rs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ccu41rs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ccu41rs::Const1
    }
}
#[doc = "CCU80 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu80rs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Ccu80rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu80rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80RS` reader - CCU80 Reset Status"]
pub type Ccu80rsR = crate::BitReader<Ccu80rs>;
impl Ccu80rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu80rs {
        match self.bits {
            false => Ccu80rs::Const0,
            true => Ccu80rs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ccu80rs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ccu80rs::Const1
    }
}
#[doc = "USIC0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic0rs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Usic0rs> for bool {
    #[inline(always)]
    fn from(variant: Usic0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0RS` reader - USIC0 Reset Status"]
pub type Usic0rsR = crate::BitReader<Usic0rs>;
impl Usic0rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usic0rs {
        match self.bits {
            false => Usic0rs::Const0,
            true => Usic0rs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Usic0rs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Usic0rs::Const1
    }
}
#[doc = "ERU1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eru1rs {
    #[doc = "0: Reset de-asserted"]
    Const0 = 0,
    #[doc = "1: Reset asserted"]
    Const1 = 1,
}
impl From<Eru1rs> for bool {
    #[inline(always)]
    fn from(variant: Eru1rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1RS` reader - ERU1 Reset Status"]
pub type Eru1rsR = crate::BitReader<Eru1rs>;
impl Eru1rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eru1rs {
        match self.bits {
            false => Eru1rs::Const0,
            true => Eru1rs::Const1,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Eru1rs::Const0
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Eru1rs::Const1
    }
}
impl R {
    #[doc = "Bit 0 - VADC Reset Status"]
    #[inline(always)]
    pub fn vadcrs(&self) -> VadcrsR {
        VadcrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CCU40 Reset Status"]
    #[inline(always)]
    pub fn ccu40rs(&self) -> Ccu40rsR {
        Ccu40rsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCU41 Reset Status"]
    #[inline(always)]
    pub fn ccu41rs(&self) -> Ccu41rsR {
        Ccu41rsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - CCU80 Reset Status"]
    #[inline(always)]
    pub fn ccu80rs(&self) -> Ccu80rsR {
        Ccu80rsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - USIC0 Reset Status"]
    #[inline(always)]
    pub fn usic0rs(&self) -> Usic0rsR {
        Usic0rsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ERU1 Reset Status"]
    #[inline(always)]
    pub fn eru1rs(&self) -> Eru1rsR {
        Eru1rsR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "RCU Peripheral 0 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstat0Spec;
impl crate::RegisterSpec for Prstat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstat0::R`](R) reader structure"]
impl crate::Readable for Prstat0Spec {}
#[doc = "`reset()` method sets PRSTAT0 to value 0x0001_0f9f"]
impl crate::Resettable for Prstat0Spec {
    const RESET_VALUE: u32 = 0x0001_0f9f;
}
