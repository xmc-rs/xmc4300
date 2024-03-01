#[doc = "Register `PDISC` reader"]
pub type R = crate::R<PdiscSpec>;
#[doc = "Pad Disable for Port n Pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis0 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis0> for bool {
    #[inline(always)]
    fn from(variant: Pdis0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS0` reader - Pad Disable for Port n Pin 0"]
pub type Pdis0R = crate::BitReader<Pdis0>;
impl Pdis0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis0 {
        match self.bits {
            false => Pdis0::Const0,
            true => Pdis0::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis0::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis0::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis1 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis1> for bool {
    #[inline(always)]
    fn from(variant: Pdis1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS1` reader - Pad Disable for Port n Pin 1"]
pub type Pdis1R = crate::BitReader<Pdis1>;
impl Pdis1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis1 {
        match self.bits {
            false => Pdis1::Const0,
            true => Pdis1::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis1::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis1::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis2 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis2> for bool {
    #[inline(always)]
    fn from(variant: Pdis2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS2` reader - Pad Disable for Port n Pin 2"]
pub type Pdis2R = crate::BitReader<Pdis2>;
impl Pdis2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis2 {
        match self.bits {
            false => Pdis2::Const0,
            true => Pdis2::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis2::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis2::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis3 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis3> for bool {
    #[inline(always)]
    fn from(variant: Pdis3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS3` reader - Pad Disable for Port n Pin 3"]
pub type Pdis3R = crate::BitReader<Pdis3>;
impl Pdis3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis3 {
        match self.bits {
            false => Pdis3::Const0,
            true => Pdis3::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis3::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis3::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis4 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis4> for bool {
    #[inline(always)]
    fn from(variant: Pdis4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS4` reader - Pad Disable for Port n Pin 4"]
pub type Pdis4R = crate::BitReader<Pdis4>;
impl Pdis4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis4 {
        match self.bits {
            false => Pdis4::Const0,
            true => Pdis4::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis4::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis4::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis5 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis5> for bool {
    #[inline(always)]
    fn from(variant: Pdis5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS5` reader - Pad Disable for Port n Pin 5"]
pub type Pdis5R = crate::BitReader<Pdis5>;
impl Pdis5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis5 {
        match self.bits {
            false => Pdis5::Const0,
            true => Pdis5::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis5::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis5::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis6 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis6> for bool {
    #[inline(always)]
    fn from(variant: Pdis6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS6` reader - Pad Disable for Port n Pin 6"]
pub type Pdis6R = crate::BitReader<Pdis6>;
impl Pdis6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis6 {
        match self.bits {
            false => Pdis6::Const0,
            true => Pdis6::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis6::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis6::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis7 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis7> for bool {
    #[inline(always)]
    fn from(variant: Pdis7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS7` reader - Pad Disable for Port n Pin 7"]
pub type Pdis7R = crate::BitReader<Pdis7>;
impl Pdis7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis7 {
        match self.bits {
            false => Pdis7::Const0,
            true => Pdis7::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis7::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis7::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis8 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis8> for bool {
    #[inline(always)]
    fn from(variant: Pdis8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS8` reader - Pad Disable for Port n Pin 8"]
pub type Pdis8R = crate::BitReader<Pdis8>;
impl Pdis8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis8 {
        match self.bits {
            false => Pdis8::Const0,
            true => Pdis8::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis8::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis8::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis9 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis9> for bool {
    #[inline(always)]
    fn from(variant: Pdis9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS9` reader - Pad Disable for Port n Pin 9"]
pub type Pdis9R = crate::BitReader<Pdis9>;
impl Pdis9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis9 {
        match self.bits {
            false => Pdis9::Const0,
            true => Pdis9::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis9::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis9::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis10 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis10> for bool {
    #[inline(always)]
    fn from(variant: Pdis10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS10` reader - Pad Disable for Port n Pin 10"]
pub type Pdis10R = crate::BitReader<Pdis10>;
impl Pdis10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis10 {
        match self.bits {
            false => Pdis10::Const0,
            true => Pdis10::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis10::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis10::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis11 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis11> for bool {
    #[inline(always)]
    fn from(variant: Pdis11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS11` reader - Pad Disable for Port n Pin 11"]
pub type Pdis11R = crate::BitReader<Pdis11>;
impl Pdis11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis11 {
        match self.bits {
            false => Pdis11::Const0,
            true => Pdis11::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis11::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis11::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis12 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis12> for bool {
    #[inline(always)]
    fn from(variant: Pdis12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS12` reader - Pad Disable for Port n Pin 12"]
pub type Pdis12R = crate::BitReader<Pdis12>;
impl Pdis12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis12 {
        match self.bits {
            false => Pdis12::Const0,
            true => Pdis12::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis12::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis12::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis13 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis13> for bool {
    #[inline(always)]
    fn from(variant: Pdis13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS13` reader - Pad Disable for Port n Pin 13"]
pub type Pdis13R = crate::BitReader<Pdis13>;
impl Pdis13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis13 {
        match self.bits {
            false => Pdis13::Const0,
            true => Pdis13::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis13::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis13::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis14 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis14> for bool {
    #[inline(always)]
    fn from(variant: Pdis14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS14` reader - Pad Disable for Port n Pin 14"]
pub type Pdis14R = crate::BitReader<Pdis14>;
impl Pdis14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis14 {
        match self.bits {
            false => Pdis14::Const0,
            true => Pdis14::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis14::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis14::Const1
    }
}
#[doc = "Pad Disable for Port n Pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis15 {
    #[doc = "0: Pad Pn.x is enabled."]
    Const0 = 0,
    #[doc = "1: Pad Pn.x is disabled."]
    Const1 = 1,
}
impl From<Pdis15> for bool {
    #[inline(always)]
    fn from(variant: Pdis15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS15` reader - Pad Disable for Port n Pin 15"]
pub type Pdis15R = crate::BitReader<Pdis15>;
impl Pdis15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis15 {
        match self.bits {
            false => Pdis15::Const0,
            true => Pdis15::Const1,
        }
    }
    #[doc = "Pad Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pdis15::Const0
    }
    #[doc = "Pad Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pdis15::Const1
    }
}
impl R {
    #[doc = "Bit 0 - Pad Disable for Port n Pin 0"]
    #[inline(always)]
    pub fn pdis0(&self) -> Pdis0R {
        Pdis0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pad Disable for Port n Pin 1"]
    #[inline(always)]
    pub fn pdis1(&self) -> Pdis1R {
        Pdis1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pad Disable for Port n Pin 2"]
    #[inline(always)]
    pub fn pdis2(&self) -> Pdis2R {
        Pdis2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pad Disable for Port n Pin 3"]
    #[inline(always)]
    pub fn pdis3(&self) -> Pdis3R {
        Pdis3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pad Disable for Port n Pin 4"]
    #[inline(always)]
    pub fn pdis4(&self) -> Pdis4R {
        Pdis4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pad Disable for Port n Pin 5"]
    #[inline(always)]
    pub fn pdis5(&self) -> Pdis5R {
        Pdis5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pad Disable for Port n Pin 6"]
    #[inline(always)]
    pub fn pdis6(&self) -> Pdis6R {
        Pdis6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pad Disable for Port n Pin 7"]
    #[inline(always)]
    pub fn pdis7(&self) -> Pdis7R {
        Pdis7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pad Disable for Port n Pin 8"]
    #[inline(always)]
    pub fn pdis8(&self) -> Pdis8R {
        Pdis8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pad Disable for Port n Pin 9"]
    #[inline(always)]
    pub fn pdis9(&self) -> Pdis9R {
        Pdis9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pad Disable for Port n Pin 10"]
    #[inline(always)]
    pub fn pdis10(&self) -> Pdis10R {
        Pdis10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pad Disable for Port n Pin 11"]
    #[inline(always)]
    pub fn pdis11(&self) -> Pdis11R {
        Pdis11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pad Disable for Port n Pin 12"]
    #[inline(always)]
    pub fn pdis12(&self) -> Pdis12R {
        Pdis12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pad Disable for Port n Pin 13"]
    #[inline(always)]
    pub fn pdis13(&self) -> Pdis13R {
        Pdis13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pad Disable for Port n Pin 14"]
    #[inline(always)]
    pub fn pdis14(&self) -> Pdis14R {
        Pdis14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pad Disable for Port n Pin 15"]
    #[inline(always)]
    pub fn pdis15(&self) -> Pdis15R {
        Pdis15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Port 5 Pin Function Decision Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdisc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdiscSpec;
impl crate::RegisterSpec for PdiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdisc::R`](R) reader structure"]
impl crate::Readable for PdiscSpec {}
#[doc = "`reset()` method sets PDISC to value 0"]
impl crate::Resettable for PdiscSpec {
    const RESET_VALUE: u32 = 0;
}
