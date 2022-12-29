#[doc = "Register `PROCON2` reader"]
pub struct R(crate::R<PROCON2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROCON2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROCON2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROCON2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `S0ROM` reader - Sector 0 Locked Forever by User 2"]
pub type S0ROM_R = crate::BitReader<S0ROM_A>;
#[doc = "Sector 0 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1 = 1,
}
impl From<S0ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S0ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S0ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0ROM_A {
        match self.bits {
            false => S0ROM_A::CONST_0,
            true => S0ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S0ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S0ROM_A::CONST_1
    }
}
#[doc = "Field `S1ROM` reader - Sector 1 Locked Forever by User 2"]
pub type S1ROM_R = crate::BitReader<S1ROM_A>;
#[doc = "Sector 1 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1 = 1,
}
impl From<S1ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S1ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S1ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1ROM_A {
        match self.bits {
            false => S1ROM_A::CONST_0,
            true => S1ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S1ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S1ROM_A::CONST_1
    }
}
#[doc = "Field `S2ROM` reader - Sector 2 Locked Forever by User 2"]
pub type S2ROM_R = crate::BitReader<S2ROM_A>;
#[doc = "Sector 2 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1 = 1,
}
impl From<S2ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S2ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S2ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2ROM_A {
        match self.bits {
            false => S2ROM_A::CONST_0,
            true => S2ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S2ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S2ROM_A::CONST_1
    }
}
#[doc = "Field `S3ROM` reader - Sector 3 Locked Forever by User 2"]
pub type S3ROM_R = crate::BitReader<S3ROM_A>;
#[doc = "Sector 3 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1 = 1,
}
impl From<S3ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S3ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S3ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3ROM_A {
        match self.bits {
            false => S3ROM_A::CONST_0,
            true => S3ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S3ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S3ROM_A::CONST_1
    }
}
#[doc = "Field `S4ROM` reader - Sector 4 Locked Forever by User 2"]
pub type S4ROM_R = crate::BitReader<S4ROM_A>;
#[doc = "Sector 4 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S4ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1 = 1,
}
impl From<S4ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S4ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S4ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S4ROM_A {
        match self.bits {
            false => S4ROM_A::CONST_0,
            true => S4ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S4ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S4ROM_A::CONST_1
    }
}
#[doc = "Field `S5ROM` reader - Sector 5 Locked Forever by User 2"]
pub type S5ROM_R = crate::BitReader<S5ROM_A>;
#[doc = "Sector 5 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S5ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1 = 1,
}
impl From<S5ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S5ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S5ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S5ROM_A {
        match self.bits {
            false => S5ROM_A::CONST_0,
            true => S5ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S5ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S5ROM_A::CONST_1
    }
}
#[doc = "Field `S6ROM` reader - Sector 6 Locked Forever by User 2"]
pub type S6ROM_R = crate::BitReader<S6ROM_A>;
#[doc = "Sector 6 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S6ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1 = 1,
}
impl From<S6ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S6ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S6ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S6ROM_A {
        match self.bits {
            false => S6ROM_A::CONST_0,
            true => S6ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S6ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S6ROM_A::CONST_1
    }
}
#[doc = "Field `S7ROM` reader - Sector 7 Locked Forever by User 2"]
pub type S7ROM_R = crate::BitReader<S7ROM_A>;
#[doc = "Sector 7 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S7ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1 = 1,
}
impl From<S7ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S7ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S7ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S7ROM_A {
        match self.bits {
            false => S7ROM_A::CONST_0,
            true => S7ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S7ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S7ROM_A::CONST_1
    }
}
#[doc = "Field `S8ROM` reader - Sector 8 Locked Forever by User 2"]
pub type S8ROM_R = crate::BitReader<S8ROM_A>;
#[doc = "Sector 8 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S8ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1 = 1,
}
impl From<S8ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S8ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S8ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S8ROM_A {
        match self.bits {
            false => S8ROM_A::CONST_0,
            true => S8ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S8ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S8ROM_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - Sector 0 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s0rom(&self) -> S0ROM_R {
        S0ROM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector 1 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s1rom(&self) -> S1ROM_R {
        S1ROM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sector 2 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s2rom(&self) -> S2ROM_R {
        S2ROM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sector 3 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s3rom(&self) -> S3ROM_R {
        S3ROM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sector 4 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s4rom(&self) -> S4ROM_R {
        S4ROM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sector 5 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s5rom(&self) -> S5ROM_R {
        S5ROM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sector 6 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s6rom(&self) -> S6ROM_R {
        S6ROM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sector 7 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s7rom(&self) -> S7ROM_R {
        S7ROM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sector 8 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s8rom(&self) -> S8ROM_R {
        S8ROM_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Flash Protection Configuration Register User 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [procon2](index.html) module"]
pub struct PROCON2_SPEC;
impl crate::RegisterSpec for PROCON2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [procon2::R](R) reader structure"]
impl crate::Readable for PROCON2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PROCON2 to value 0"]
impl crate::Resettable for PROCON2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
