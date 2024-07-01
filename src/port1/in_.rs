#[doc = "Register `IN` reader"]
pub type R = crate::R<IN_SPEC>;
#[doc = "Port n Input Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P0_A> for bool {
    #[inline(always)]
    fn from(variant: P0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0` reader - Port n Input Bit 0\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P0_R = crate::BitReader<P0_A>;
impl P0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_A {
        match self.bits {
            false => P0_A::CONST_0,
            true => P0_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P0_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P0_A::CONST_1
    }
}
#[doc = "Port n Input Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P1_A> for bool {
    #[inline(always)]
    fn from(variant: P1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1` reader - Port n Input Bit 1\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P1_R = crate::BitReader<P1_A>;
impl P1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_A {
        match self.bits {
            false => P1_A::CONST_0,
            true => P1_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P1_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P1_A::CONST_1
    }
}
#[doc = "Port n Input Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P2_A> for bool {
    #[inline(always)]
    fn from(variant: P2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2` reader - Port n Input Bit 2\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P2_R = crate::BitReader<P2_A>;
impl P2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_A {
        match self.bits {
            false => P2_A::CONST_0,
            true => P2_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P2_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P2_A::CONST_1
    }
}
#[doc = "Port n Input Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P3_A> for bool {
    #[inline(always)]
    fn from(variant: P3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P3` reader - Port n Input Bit 3\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P3_R = crate::BitReader<P3_A>;
impl P3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3_A {
        match self.bits {
            false => P3_A::CONST_0,
            true => P3_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P3_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P3_A::CONST_1
    }
}
#[doc = "Port n Input Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P4_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P4_A> for bool {
    #[inline(always)]
    fn from(variant: P4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P4` reader - Port n Input Bit 4\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P4_R = crate::BitReader<P4_A>;
impl P4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P4_A {
        match self.bits {
            false => P4_A::CONST_0,
            true => P4_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P4_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P4_A::CONST_1
    }
}
#[doc = "Port n Input Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P5_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P5_A> for bool {
    #[inline(always)]
    fn from(variant: P5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P5` reader - Port n Input Bit 5\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P5_R = crate::BitReader<P5_A>;
impl P5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P5_A {
        match self.bits {
            false => P5_A::CONST_0,
            true => P5_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P5_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P5_A::CONST_1
    }
}
#[doc = "Port n Input Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P6_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P6_A> for bool {
    #[inline(always)]
    fn from(variant: P6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P6` reader - Port n Input Bit 6\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P6_R = crate::BitReader<P6_A>;
impl P6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P6_A {
        match self.bits {
            false => P6_A::CONST_0,
            true => P6_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P6_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P6_A::CONST_1
    }
}
#[doc = "Port n Input Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P7_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P7_A> for bool {
    #[inline(always)]
    fn from(variant: P7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P7` reader - Port n Input Bit 7\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P7_R = crate::BitReader<P7_A>;
impl P7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P7_A {
        match self.bits {
            false => P7_A::CONST_0,
            true => P7_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P7_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P7_A::CONST_1
    }
}
#[doc = "Port n Input Bit 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P8_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P8_A> for bool {
    #[inline(always)]
    fn from(variant: P8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P8` reader - Port n Input Bit 8\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P8_R = crate::BitReader<P8_A>;
impl P8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P8_A {
        match self.bits {
            false => P8_A::CONST_0,
            true => P8_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P8_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P8_A::CONST_1
    }
}
#[doc = "Port n Input Bit 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P9_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P9_A> for bool {
    #[inline(always)]
    fn from(variant: P9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P9` reader - Port n Input Bit 9\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P9_R = crate::BitReader<P9_A>;
impl P9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P9_A {
        match self.bits {
            false => P9_A::CONST_0,
            true => P9_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P9_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P9_A::CONST_1
    }
}
#[doc = "Port n Input Bit 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P10_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P10_A> for bool {
    #[inline(always)]
    fn from(variant: P10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P10` reader - Port n Input Bit 10\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P10_R = crate::BitReader<P10_A>;
impl P10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P10_A {
        match self.bits {
            false => P10_A::CONST_0,
            true => P10_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P10_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P10_A::CONST_1
    }
}
#[doc = "Port n Input Bit 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P11_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P11_A> for bool {
    #[inline(always)]
    fn from(variant: P11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P11` reader - Port n Input Bit 11\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P11_R = crate::BitReader<P11_A>;
impl P11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P11_A {
        match self.bits {
            false => P11_A::CONST_0,
            true => P11_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P11_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P11_A::CONST_1
    }
}
#[doc = "Port n Input Bit 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P12_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P12_A> for bool {
    #[inline(always)]
    fn from(variant: P12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P12` reader - Port n Input Bit 12\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P12_R = crate::BitReader<P12_A>;
impl P12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P12_A {
        match self.bits {
            false => P12_A::CONST_0,
            true => P12_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P12_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P12_A::CONST_1
    }
}
#[doc = "Port n Input Bit 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P13_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P13_A> for bool {
    #[inline(always)]
    fn from(variant: P13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P13` reader - Port n Input Bit 13\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P13_R = crate::BitReader<P13_A>;
impl P13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P13_A {
        match self.bits {
            false => P13_A::CONST_0,
            true => P13_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P13_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P13_A::CONST_1
    }
}
#[doc = "Port n Input Bit 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P14_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P14_A> for bool {
    #[inline(always)]
    fn from(variant: P14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P14` reader - Port n Input Bit 14\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P14_R = crate::BitReader<P14_A>;
impl P14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P14_A {
        match self.bits {
            false => P14_A::CONST_0,
            true => P14_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P14_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P14_A::CONST_1
    }
}
#[doc = "Port n Input Bit 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P15_A {
    #[doc = "0: The input level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The input level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P15_A> for bool {
    #[inline(always)]
    fn from(variant: P15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P15` reader - Port n Input Bit 15\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P15_R = crate::BitReader<P15_A>;
impl P15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P15_A {
        match self.bits {
            false => P15_A::CONST_0,
            true => P15_A::CONST_1,
        }
    }
    #[doc = "The input level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P15_A::CONST_0
    }
    #[doc = "The input level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P15_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - Port n Input Bit 0"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port n Input Bit 1"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port n Input Bit 2"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port n Input Bit 3"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port n Input Bit 4"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port n Input Bit 5"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port n Input Bit 6"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port n Input Bit 7"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port n Input Bit 8"]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port n Input Bit 9"]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port n Input Bit 10"]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port n Input Bit 11"]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port n Input Bit 12"]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port n Input Bit 13"]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port n Input Bit 14"]
    #[inline(always)]
    pub fn p14(&self) -> P14_R {
        P14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port n Input Bit 15"]
    #[inline(always)]
    pub fn p15(&self) -> P15_R {
        P15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Port 1 Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for IN_SPEC {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for IN_SPEC {
    const RESET_VALUE: u32 = 0;
}
