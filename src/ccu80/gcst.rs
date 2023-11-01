#[doc = "Register `GCST` reader"]
pub type R = crate::R<GCST_SPEC>;
#[doc = "Field `S0SS` reader - Slice 0 shadow transfer status"]
pub type S0SS_R = crate::BitReader<S0SS_A>;
#[doc = "Slice 0 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S0SS_A> for bool {
    #[inline(always)]
    fn from(variant: S0SS_A) -> Self {
        variant as u8 != 0
    }
}
impl S0SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0SS_A {
        match self.bits {
            false => S0SS_A::VALUE1,
            true => S0SS_A::VALUE2,
        }
    }
    #[doc = "Shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0SS_A::VALUE1
    }
    #[doc = "Shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0SS_A::VALUE2
    }
}
#[doc = "Field `S0DSS` reader - Slice 0 Dither shadow transfer status"]
pub type S0DSS_R = crate::BitReader<S0DSS_A>;
#[doc = "Slice 0 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S0DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S0DSS_A) -> Self {
        variant as u8 != 0
    }
}
impl S0DSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0DSS_A {
        match self.bits {
            false => S0DSS_A::VALUE1,
            true => S0DSS_A::VALUE2,
        }
    }
    #[doc = "Dither shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0DSS_A::VALUE1
    }
    #[doc = "Dither shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0DSS_A::VALUE2
    }
}
#[doc = "Field `S0PSS` reader - Slice 0 Prescaler shadow transfer status"]
pub type S0PSS_R = crate::BitReader<S0PSS_A>;
#[doc = "Slice 0 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S0PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S0PSS_A) -> Self {
        variant as u8 != 0
    }
}
impl S0PSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0PSS_A {
        match self.bits {
            false => S0PSS_A::VALUE1,
            true => S0PSS_A::VALUE2,
        }
    }
    #[doc = "Prescaler shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0PSS_A::VALUE1
    }
    #[doc = "Prescaler shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0PSS_A::VALUE2
    }
}
#[doc = "Field `S1SS` reader - Slice 1 shadow transfer status"]
pub type S1SS_R = crate::BitReader<S1SS_A>;
#[doc = "Slice 1 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S1SS_A> for bool {
    #[inline(always)]
    fn from(variant: S1SS_A) -> Self {
        variant as u8 != 0
    }
}
impl S1SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1SS_A {
        match self.bits {
            false => S1SS_A::VALUE1,
            true => S1SS_A::VALUE2,
        }
    }
    #[doc = "Shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1SS_A::VALUE1
    }
    #[doc = "Shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1SS_A::VALUE2
    }
}
#[doc = "Field `S1DSS` reader - Slice 1 Dither shadow transfer status"]
pub type S1DSS_R = crate::BitReader<S1DSS_A>;
#[doc = "Slice 1 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S1DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S1DSS_A) -> Self {
        variant as u8 != 0
    }
}
impl S1DSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1DSS_A {
        match self.bits {
            false => S1DSS_A::VALUE1,
            true => S1DSS_A::VALUE2,
        }
    }
    #[doc = "Dither shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1DSS_A::VALUE1
    }
    #[doc = "Dither shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1DSS_A::VALUE2
    }
}
#[doc = "Field `S1PSS` reader - Slice 1 Prescaler shadow transfer status"]
pub type S1PSS_R = crate::BitReader<S1PSS_A>;
#[doc = "Slice 1 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S1PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S1PSS_A) -> Self {
        variant as u8 != 0
    }
}
impl S1PSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1PSS_A {
        match self.bits {
            false => S1PSS_A::VALUE1,
            true => S1PSS_A::VALUE2,
        }
    }
    #[doc = "Prescaler shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1PSS_A::VALUE1
    }
    #[doc = "Prescaler shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1PSS_A::VALUE2
    }
}
#[doc = "Field `S2SS` reader - Slice 2 shadow transfer status"]
pub type S2SS_R = crate::BitReader<S2SS_A>;
#[doc = "Slice 2 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S2SS_A> for bool {
    #[inline(always)]
    fn from(variant: S2SS_A) -> Self {
        variant as u8 != 0
    }
}
impl S2SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2SS_A {
        match self.bits {
            false => S2SS_A::VALUE1,
            true => S2SS_A::VALUE2,
        }
    }
    #[doc = "Shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2SS_A::VALUE1
    }
    #[doc = "Shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2SS_A::VALUE2
    }
}
#[doc = "Field `S2DSS` reader - Slice 2 Dither shadow transfer status"]
pub type S2DSS_R = crate::BitReader<S2DSS_A>;
#[doc = "Slice 2 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S2DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S2DSS_A) -> Self {
        variant as u8 != 0
    }
}
impl S2DSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2DSS_A {
        match self.bits {
            false => S2DSS_A::VALUE1,
            true => S2DSS_A::VALUE2,
        }
    }
    #[doc = "Dither shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2DSS_A::VALUE1
    }
    #[doc = "Dither shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2DSS_A::VALUE2
    }
}
#[doc = "Field `S2PSS` reader - Slice 2 Prescaler shadow transfer status"]
pub type S2PSS_R = crate::BitReader<S2PSS_A>;
#[doc = "Slice 2 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S2PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S2PSS_A) -> Self {
        variant as u8 != 0
    }
}
impl S2PSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2PSS_A {
        match self.bits {
            false => S2PSS_A::VALUE1,
            true => S2PSS_A::VALUE2,
        }
    }
    #[doc = "Prescaler shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2PSS_A::VALUE1
    }
    #[doc = "Prescaler shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2PSS_A::VALUE2
    }
}
#[doc = "Field `S3SS` reader - Slice 3 shadow transfer status"]
pub type S3SS_R = crate::BitReader<S3SS_A>;
#[doc = "Slice 3 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S3SS_A> for bool {
    #[inline(always)]
    fn from(variant: S3SS_A) -> Self {
        variant as u8 != 0
    }
}
impl S3SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3SS_A {
        match self.bits {
            false => S3SS_A::VALUE1,
            true => S3SS_A::VALUE2,
        }
    }
    #[doc = "Shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3SS_A::VALUE1
    }
    #[doc = "Shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3SS_A::VALUE2
    }
}
#[doc = "Field `S3DSS` reader - Slice 3 Dither shadow transfer status"]
pub type S3DSS_R = crate::BitReader<S3DSS_A>;
#[doc = "Slice 3 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S3DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S3DSS_A) -> Self {
        variant as u8 != 0
    }
}
impl S3DSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3DSS_A {
        match self.bits {
            false => S3DSS_A::VALUE1,
            true => S3DSS_A::VALUE2,
        }
    }
    #[doc = "Dither shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3DSS_A::VALUE1
    }
    #[doc = "Dither shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3DSS_A::VALUE2
    }
}
#[doc = "Field `S3PSS` reader - Slice 3 Prescaler shadow transfer status"]
pub type S3PSS_R = crate::BitReader<S3PSS_A>;
#[doc = "Slice 3 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S3PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S3PSS_A) -> Self {
        variant as u8 != 0
    }
}
impl S3PSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3PSS_A {
        match self.bits {
            false => S3PSS_A::VALUE1,
            true => S3PSS_A::VALUE2,
        }
    }
    #[doc = "Prescaler shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3PSS_A::VALUE1
    }
    #[doc = "Prescaler shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3PSS_A::VALUE2
    }
}
#[doc = "Field `CC80ST1` reader - Slice 0 compare channel 1 status bit"]
pub type CC80ST1_R = crate::BitReader;
#[doc = "Field `CC81ST1` reader - Slice 1 compare channel 1 status bit"]
pub type CC81ST1_R = crate::BitReader;
#[doc = "Field `CC82ST1` reader - Slice 2 compare channel 1 status bit"]
pub type CC82ST1_R = crate::BitReader;
#[doc = "Field `CC83ST1` reader - Slice 3 compare channel 1 status bit"]
pub type CC83ST1_R = crate::BitReader;
#[doc = "Field `CC80ST2` reader - Slice 0 compare channel 2 status bit"]
pub type CC80ST2_R = crate::BitReader;
#[doc = "Field `CC81ST2` reader - Slice 1 compare channel 2 status bit"]
pub type CC81ST2_R = crate::BitReader;
#[doc = "Field `CC82ST2` reader - Slice 2 compare channel 2 status bit"]
pub type CC82ST2_R = crate::BitReader;
#[doc = "Field `CC83ST2` reader - Slice 3 compare channel 2 status bit"]
pub type CC83ST2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Slice 0 shadow transfer status"]
    #[inline(always)]
    pub fn s0ss(&self) -> S0SS_R {
        S0SS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s0dss(&self) -> S0DSS_R {
        S0DSS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s0pss(&self) -> S0PSS_R {
        S0PSS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer status"]
    #[inline(always)]
    pub fn s1ss(&self) -> S1SS_R {
        S1SS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s1dss(&self) -> S1DSS_R {
        S1DSS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s1pss(&self) -> S1PSS_R {
        S1PSS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer status"]
    #[inline(always)]
    pub fn s2ss(&self) -> S2SS_R {
        S2SS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s2dss(&self) -> S2DSS_R {
        S2DSS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s2pss(&self) -> S2PSS_R {
        S2PSS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer status"]
    #[inline(always)]
    pub fn s3ss(&self) -> S3SS_R {
        S3SS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s3dss(&self) -> S3DSS_R {
        S3DSS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s3pss(&self) -> S3PSS_R {
        S3PSS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Slice 0 compare channel 1 status bit"]
    #[inline(always)]
    pub fn cc80st1(&self) -> CC80ST1_R {
        CC80ST1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slice 1 compare channel 1 status bit"]
    #[inline(always)]
    pub fn cc81st1(&self) -> CC81ST1_R {
        CC81ST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slice 2 compare channel 1 status bit"]
    #[inline(always)]
    pub fn cc82st1(&self) -> CC82ST1_R {
        CC82ST1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slice 3 compare channel 1 status bit"]
    #[inline(always)]
    pub fn cc83st1(&self) -> CC83ST1_R {
        CC83ST1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Slice 0 compare channel 2 status bit"]
    #[inline(always)]
    pub fn cc80st2(&self) -> CC80ST2_R {
        CC80ST2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Slice 1 compare channel 2 status bit"]
    #[inline(always)]
    pub fn cc81st2(&self) -> CC81ST2_R {
        CC81ST2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Slice 2 compare channel 2 status bit"]
    #[inline(always)]
    pub fn cc82st2(&self) -> CC82ST2_R {
        CC82ST2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slice 3 compare channel 2 status bit"]
    #[inline(always)]
    pub fn cc83st2(&self) -> CC83ST2_R {
        CC83ST2_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Global Channel status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCST_SPEC;
impl crate::RegisterSpec for GCST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcst::R`](R) reader structure"]
impl crate::Readable for GCST_SPEC {}
#[doc = "`reset()` method sets GCST to value 0"]
impl crate::Resettable for GCST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
