#[doc = "Register `PDISC` reader"]
pub type R = crate::R<PDISC_SPEC>;
#[doc = "Field `PDIS0` reader - Pad Disable for Port n Pin 0"]
pub type PDIS0_R = crate::BitReader<PDIS0_A>;
#[doc = "Pad Disable for Port n Pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS0_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS0_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS0_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS0_A {
        match self.bits {
            false => PDIS0_A::CONST_0,
            true => PDIS0_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS0_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS0_A::CONST_1
    }
}
#[doc = "Field `PDIS1` reader - Pad Disable for Port n Pin 1"]
pub type PDIS1_R = crate::BitReader<PDIS1_A>;
#[doc = "Pad Disable for Port n Pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS1_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS1_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS1_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS1_A {
        match self.bits {
            false => PDIS1_A::CONST_0,
            true => PDIS1_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS1_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS1_A::CONST_1
    }
}
#[doc = "Field `PDIS2` reader - Pad Disable for Port n Pin 2"]
pub type PDIS2_R = crate::BitReader<PDIS2_A>;
#[doc = "Pad Disable for Port n Pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS2_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS2_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS2_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS2_A {
        match self.bits {
            false => PDIS2_A::CONST_0,
            true => PDIS2_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS2_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS2_A::CONST_1
    }
}
#[doc = "Field `PDIS3` reader - Pad Disable for Port n Pin 3"]
pub type PDIS3_R = crate::BitReader<PDIS3_A>;
#[doc = "Pad Disable for Port n Pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS3_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS3_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS3_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS3_A {
        match self.bits {
            false => PDIS3_A::CONST_0,
            true => PDIS3_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS3_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS3_A::CONST_1
    }
}
#[doc = "Field `PDIS4` reader - Pad Disable for Port n Pin 4"]
pub type PDIS4_R = crate::BitReader<PDIS4_A>;
#[doc = "Pad Disable for Port n Pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS4_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS4_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS4_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS4_A {
        match self.bits {
            false => PDIS4_A::CONST_0,
            true => PDIS4_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS4_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS4_A::CONST_1
    }
}
#[doc = "Field `PDIS5` reader - Pad Disable for Port n Pin 5"]
pub type PDIS5_R = crate::BitReader<PDIS5_A>;
#[doc = "Pad Disable for Port n Pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS5_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS5_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS5_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS5_A {
        match self.bits {
            false => PDIS5_A::CONST_0,
            true => PDIS5_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS5_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS5_A::CONST_1
    }
}
#[doc = "Field `PDIS6` reader - Pad Disable for Port n Pin 6"]
pub type PDIS6_R = crate::BitReader<PDIS6_A>;
#[doc = "Pad Disable for Port n Pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS6_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS6_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS6_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS6_A {
        match self.bits {
            false => PDIS6_A::CONST_0,
            true => PDIS6_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS6_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS6_A::CONST_1
    }
}
#[doc = "Field `PDIS7` reader - Pad Disable for Port n Pin 7"]
pub type PDIS7_R = crate::BitReader<PDIS7_A>;
#[doc = "Pad Disable for Port n Pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS7_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS7_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS7_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS7_A {
        match self.bits {
            false => PDIS7_A::CONST_0,
            true => PDIS7_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS7_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS7_A::CONST_1
    }
}
#[doc = "Field `PDIS8` reader - Pad Disable for Port n Pin 8"]
pub type PDIS8_R = crate::BitReader<PDIS8_A>;
#[doc = "Pad Disable for Port n Pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS8_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS8_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS8_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS8_A {
        match self.bits {
            false => PDIS8_A::CONST_0,
            true => PDIS8_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS8_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS8_A::CONST_1
    }
}
#[doc = "Field `PDIS9` reader - Pad Disable for Port n Pin 9"]
pub type PDIS9_R = crate::BitReader<PDIS9_A>;
#[doc = "Pad Disable for Port n Pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS9_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS9_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS9_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS9_A {
        match self.bits {
            false => PDIS9_A::CONST_0,
            true => PDIS9_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS9_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS9_A::CONST_1
    }
}
#[doc = "Field `PDIS10` reader - Pad Disable for Port n Pin 10"]
pub type PDIS10_R = crate::BitReader<PDIS10_A>;
#[doc = "Pad Disable for Port n Pin 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS10_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS10_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS10_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS10_A {
        match self.bits {
            false => PDIS10_A::CONST_0,
            true => PDIS10_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS10_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS10_A::CONST_1
    }
}
#[doc = "Field `PDIS11` reader - Pad Disable for Port n Pin 11"]
pub type PDIS11_R = crate::BitReader<PDIS11_A>;
#[doc = "Pad Disable for Port n Pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS11_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS11_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS11_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS11_A {
        match self.bits {
            false => PDIS11_A::CONST_0,
            true => PDIS11_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS11_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS11_A::CONST_1
    }
}
#[doc = "Field `PDIS12` reader - Pad Disable for Port n Pin 12"]
pub type PDIS12_R = crate::BitReader<PDIS12_A>;
#[doc = "Pad Disable for Port n Pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS12_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS12_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS12_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS12_A {
        match self.bits {
            false => PDIS12_A::CONST_0,
            true => PDIS12_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS12_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS12_A::CONST_1
    }
}
#[doc = "Field `PDIS13` reader - Pad Disable for Port n Pin 13"]
pub type PDIS13_R = crate::BitReader<PDIS13_A>;
#[doc = "Pad Disable for Port n Pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS13_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS13_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS13_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS13_A {
        match self.bits {
            false => PDIS13_A::CONST_0,
            true => PDIS13_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS13_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS13_A::CONST_1
    }
}
#[doc = "Field `PDIS14` reader - Pad Disable for Port n Pin 14"]
pub type PDIS14_R = crate::BitReader<PDIS14_A>;
#[doc = "Pad Disable for Port n Pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS14_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS14_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS14_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS14_A {
        match self.bits {
            false => PDIS14_A::CONST_0,
            true => PDIS14_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS14_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS14_A::CONST_1
    }
}
#[doc = "Field `PDIS15` reader - Pad Disable for Port n Pin 15"]
pub type PDIS15_R = crate::BitReader<PDIS15_A>;
#[doc = "Pad Disable for Port n Pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS15_A {
    #[doc = "0: Pad Pn.x is enabled."]
    CONST_0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    CONST_1 = 1,
}
impl From<PDIS15_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS15_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS15_A {
        match self.bits {
            false => PDIS15_A::CONST_0,
            true => PDIS15_A::CONST_1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS15_A::CONST_0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS15_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - Pad Disable for Port n Pin 0"]
    #[inline(always)]
    pub fn pdis0(&self) -> PDIS0_R {
        PDIS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pad Disable for Port n Pin 1"]
    #[inline(always)]
    pub fn pdis1(&self) -> PDIS1_R {
        PDIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pad Disable for Port n Pin 2"]
    #[inline(always)]
    pub fn pdis2(&self) -> PDIS2_R {
        PDIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pad Disable for Port n Pin 3"]
    #[inline(always)]
    pub fn pdis3(&self) -> PDIS3_R {
        PDIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pad Disable for Port n Pin 4"]
    #[inline(always)]
    pub fn pdis4(&self) -> PDIS4_R {
        PDIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pad Disable for Port n Pin 5"]
    #[inline(always)]
    pub fn pdis5(&self) -> PDIS5_R {
        PDIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pad Disable for Port n Pin 6"]
    #[inline(always)]
    pub fn pdis6(&self) -> PDIS6_R {
        PDIS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pad Disable for Port n Pin 7"]
    #[inline(always)]
    pub fn pdis7(&self) -> PDIS7_R {
        PDIS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pad Disable for Port n Pin 8"]
    #[inline(always)]
    pub fn pdis8(&self) -> PDIS8_R {
        PDIS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pad Disable for Port n Pin 9"]
    #[inline(always)]
    pub fn pdis9(&self) -> PDIS9_R {
        PDIS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pad Disable for Port n Pin 10"]
    #[inline(always)]
    pub fn pdis10(&self) -> PDIS10_R {
        PDIS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pad Disable for Port n Pin 11"]
    #[inline(always)]
    pub fn pdis11(&self) -> PDIS11_R {
        PDIS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pad Disable for Port n Pin 12"]
    #[inline(always)]
    pub fn pdis12(&self) -> PDIS12_R {
        PDIS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pad Disable for Port n Pin 13"]
    #[inline(always)]
    pub fn pdis13(&self) -> PDIS13_R {
        PDIS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pad Disable for Port n Pin 14"]
    #[inline(always)]
    pub fn pdis14(&self) -> PDIS14_R {
        PDIS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pad Disable for Port n Pin 15"]
    #[inline(always)]
    pub fn pdis15(&self) -> PDIS15_R {
        PDIS15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Port 5 Pin Function Decision Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdisc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDISC_SPEC;
impl crate::RegisterSpec for PDISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdisc::R`](R) reader structure"]
impl crate::Readable for PDISC_SPEC {}
#[doc = "`reset()` method sets PDISC to value 0"]
impl crate::Resettable for PDISC_SPEC {
    const RESET_VALUE: u32 = 0;
}
