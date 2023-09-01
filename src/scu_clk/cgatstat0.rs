#[doc = "Register `CGATSTAT0` reader"]
pub type R = crate::R<CGATSTAT0_SPEC>;
#[doc = "Field `VADC` reader - VADC Gating Status"]
pub type VADC_R = crate::BitReader<VADC_A>;
#[doc = "VADC Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VADC_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<VADC_A> for bool {
    #[inline(always)]
    fn from(variant: VADC_A) -> Self {
        variant as u8 != 0
    }
}
impl VADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VADC_A {
        match self.bits {
            false => VADC_A::CONST_0,
            true => VADC_A::CONST_1,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VADC_A::CONST_0
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VADC_A::CONST_1
    }
}
#[doc = "Field `CCU40` reader - CCU40 Gating Status"]
pub type CCU40_R = crate::BitReader<CCU40_A>;
#[doc = "CCU40 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU40_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<CCU40_A> for bool {
    #[inline(always)]
    fn from(variant: CCU40_A) -> Self {
        variant as u8 != 0
    }
}
impl CCU40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU40_A {
        match self.bits {
            false => CCU40_A::CONST_0,
            true => CCU40_A::CONST_1,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == CCU40_A::CONST_0
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CCU40_A::CONST_1
    }
}
#[doc = "Field `CCU41` reader - CCU41 Gating Status"]
pub type CCU41_R = crate::BitReader<CCU41_A>;
#[doc = "CCU41 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU41_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<CCU41_A> for bool {
    #[inline(always)]
    fn from(variant: CCU41_A) -> Self {
        variant as u8 != 0
    }
}
impl CCU41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU41_A {
        match self.bits {
            false => CCU41_A::CONST_0,
            true => CCU41_A::CONST_1,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == CCU41_A::CONST_0
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CCU41_A::CONST_1
    }
}
#[doc = "Field `CCU80` reader - CCU80 Gating Status"]
pub type CCU80_R = crate::BitReader<CCU80_A>;
#[doc = "CCU80 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU80_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<CCU80_A> for bool {
    #[inline(always)]
    fn from(variant: CCU80_A) -> Self {
        variant as u8 != 0
    }
}
impl CCU80_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU80_A {
        match self.bits {
            false => CCU80_A::CONST_0,
            true => CCU80_A::CONST_1,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == CCU80_A::CONST_0
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CCU80_A::CONST_1
    }
}
#[doc = "Field `POSIF0` reader - POSIF0 Gating Status"]
pub type POSIF0_R = crate::BitReader<POSIF0_A>;
#[doc = "POSIF0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSIF0_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<POSIF0_A> for bool {
    #[inline(always)]
    fn from(variant: POSIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl POSIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSIF0_A {
        match self.bits {
            false => POSIF0_A::CONST_0,
            true => POSIF0_A::CONST_1,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == POSIF0_A::CONST_0
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == POSIF0_A::CONST_1
    }
}
#[doc = "Field `USIC0` reader - USIC0 Gating Status"]
pub type USIC0_R = crate::BitReader<USIC0_A>;
#[doc = "USIC0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<USIC0_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0_A) -> Self {
        variant as u8 != 0
    }
}
impl USIC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC0_A {
        match self.bits {
            false => USIC0_A::CONST_0,
            true => USIC0_A::CONST_1,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == USIC0_A::CONST_0
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == USIC0_A::CONST_1
    }
}
#[doc = "Field `ERU1` reader - ERU1 Gating Status"]
pub type ERU1_R = crate::BitReader<ERU1_A>;
#[doc = "ERU1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU1_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<ERU1_A> for bool {
    #[inline(always)]
    fn from(variant: ERU1_A) -> Self {
        variant as u8 != 0
    }
}
impl ERU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU1_A {
        match self.bits {
            false => ERU1_A::CONST_0,
            true => ERU1_A::CONST_1,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ERU1_A::CONST_0
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ERU1_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - VADC Gating Status"]
    #[inline(always)]
    pub fn vadc(&self) -> VADC_R {
        VADC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CCU40 Gating Status"]
    #[inline(always)]
    pub fn ccu40(&self) -> CCU40_R {
        CCU40_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCU41 Gating Status"]
    #[inline(always)]
    pub fn ccu41(&self) -> CCU41_R {
        CCU41_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - CCU80 Gating Status"]
    #[inline(always)]
    pub fn ccu80(&self) -> CCU80_R {
        CCU80_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - POSIF0 Gating Status"]
    #[inline(always)]
    pub fn posif0(&self) -> POSIF0_R {
        POSIF0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - USIC0 Gating Status"]
    #[inline(always)]
    pub fn usic0(&self) -> USIC0_R {
        USIC0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ERU1 Gating Status"]
    #[inline(always)]
    pub fn eru1(&self) -> ERU1_R {
        ERU1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Peripheral 0 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATSTAT0_SPEC;
impl crate::RegisterSpec for CGATSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgatstat0::R`](R) reader structure"]
impl crate::Readable for CGATSTAT0_SPEC {}
#[doc = "`reset()` method sets CGATSTAT0 to value 0"]
impl crate::Resettable for CGATSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
