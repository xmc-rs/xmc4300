#[doc = "Register `GCST` reader"]
pub type R = crate::R<GcstSpec>;
#[doc = "Slice 0 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0ss {
    #[doc = "0: Shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S0ss> for bool {
    #[inline(always)]
    fn from(variant: S0ss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0SS` reader - Slice 0 shadow transfer status"]
pub type S0ssR = crate::BitReader<S0ss>;
impl S0ssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0ss {
        match self.bits {
            false => S0ss::Value1,
            true => S0ss::Value2,
        }
    }
    #[doc = "Shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0ss::Value1
    }
    #[doc = "Shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0ss::Value2
    }
}
#[doc = "Slice 0 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0dss {
    #[doc = "0: Dither shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S0dss> for bool {
    #[inline(always)]
    fn from(variant: S0dss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0DSS` reader - Slice 0 Dither shadow transfer status"]
pub type S0dssR = crate::BitReader<S0dss>;
impl S0dssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0dss {
        match self.bits {
            false => S0dss::Value1,
            true => S0dss::Value2,
        }
    }
    #[doc = "Dither shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0dss::Value1
    }
    #[doc = "Dither shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0dss::Value2
    }
}
#[doc = "Slice 0 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0pss {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S0pss> for bool {
    #[inline(always)]
    fn from(variant: S0pss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0PSS` reader - Slice 0 Prescaler shadow transfer status"]
pub type S0pssR = crate::BitReader<S0pss>;
impl S0pssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0pss {
        match self.bits {
            false => S0pss::Value1,
            true => S0pss::Value2,
        }
    }
    #[doc = "Prescaler shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0pss::Value1
    }
    #[doc = "Prescaler shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0pss::Value2
    }
}
#[doc = "Slice 1 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1ss {
    #[doc = "0: Shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S1ss> for bool {
    #[inline(always)]
    fn from(variant: S1ss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1SS` reader - Slice 1 shadow transfer status"]
pub type S1ssR = crate::BitReader<S1ss>;
impl S1ssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1ss {
        match self.bits {
            false => S1ss::Value1,
            true => S1ss::Value2,
        }
    }
    #[doc = "Shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1ss::Value1
    }
    #[doc = "Shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1ss::Value2
    }
}
#[doc = "Slice 1 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1dss {
    #[doc = "0: Dither shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S1dss> for bool {
    #[inline(always)]
    fn from(variant: S1dss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1DSS` reader - Slice 1 Dither shadow transfer status"]
pub type S1dssR = crate::BitReader<S1dss>;
impl S1dssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1dss {
        match self.bits {
            false => S1dss::Value1,
            true => S1dss::Value2,
        }
    }
    #[doc = "Dither shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1dss::Value1
    }
    #[doc = "Dither shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1dss::Value2
    }
}
#[doc = "Slice 1 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1pss {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S1pss> for bool {
    #[inline(always)]
    fn from(variant: S1pss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1PSS` reader - Slice 1 Prescaler shadow transfer status"]
pub type S1pssR = crate::BitReader<S1pss>;
impl S1pssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1pss {
        match self.bits {
            false => S1pss::Value1,
            true => S1pss::Value2,
        }
    }
    #[doc = "Prescaler shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1pss::Value1
    }
    #[doc = "Prescaler shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1pss::Value2
    }
}
#[doc = "Slice 2 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2ss {
    #[doc = "0: Shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S2ss> for bool {
    #[inline(always)]
    fn from(variant: S2ss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2SS` reader - Slice 2 shadow transfer status"]
pub type S2ssR = crate::BitReader<S2ss>;
impl S2ssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2ss {
        match self.bits {
            false => S2ss::Value1,
            true => S2ss::Value2,
        }
    }
    #[doc = "Shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2ss::Value1
    }
    #[doc = "Shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2ss::Value2
    }
}
#[doc = "Slice 2 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2dss {
    #[doc = "0: Dither shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S2dss> for bool {
    #[inline(always)]
    fn from(variant: S2dss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2DSS` reader - Slice 2 Dither shadow transfer status"]
pub type S2dssR = crate::BitReader<S2dss>;
impl S2dssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2dss {
        match self.bits {
            false => S2dss::Value1,
            true => S2dss::Value2,
        }
    }
    #[doc = "Dither shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2dss::Value1
    }
    #[doc = "Dither shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2dss::Value2
    }
}
#[doc = "Slice 2 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2pss {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S2pss> for bool {
    #[inline(always)]
    fn from(variant: S2pss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2PSS` reader - Slice 2 Prescaler shadow transfer status"]
pub type S2pssR = crate::BitReader<S2pss>;
impl S2pssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2pss {
        match self.bits {
            false => S2pss::Value1,
            true => S2pss::Value2,
        }
    }
    #[doc = "Prescaler shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2pss::Value1
    }
    #[doc = "Prescaler shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2pss::Value2
    }
}
#[doc = "Slice 3 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3ss {
    #[doc = "0: Shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S3ss> for bool {
    #[inline(always)]
    fn from(variant: S3ss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3SS` reader - Slice 3 shadow transfer status"]
pub type S3ssR = crate::BitReader<S3ss>;
impl S3ssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3ss {
        match self.bits {
            false => S3ss::Value1,
            true => S3ss::Value2,
        }
    }
    #[doc = "Shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3ss::Value1
    }
    #[doc = "Shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3ss::Value2
    }
}
#[doc = "Slice 3 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3dss {
    #[doc = "0: Dither shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S3dss> for bool {
    #[inline(always)]
    fn from(variant: S3dss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3DSS` reader - Slice 3 Dither shadow transfer status"]
pub type S3dssR = crate::BitReader<S3dss>;
impl S3dssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3dss {
        match self.bits {
            false => S3dss::Value1,
            true => S3dss::Value2,
        }
    }
    #[doc = "Dither shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3dss::Value1
    }
    #[doc = "Dither shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3dss::Value2
    }
}
#[doc = "Slice 3 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3pss {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    Value1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    Value2 = 1,
}
impl From<S3pss> for bool {
    #[inline(always)]
    fn from(variant: S3pss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3PSS` reader - Slice 3 Prescaler shadow transfer status"]
pub type S3pssR = crate::BitReader<S3pss>;
impl S3pssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3pss {
        match self.bits {
            false => S3pss::Value1,
            true => S3pss::Value2,
        }
    }
    #[doc = "Prescaler shadow transfer has not been requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3pss::Value1
    }
    #[doc = "Prescaler shadow transfer has been requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3pss::Value2
    }
}
#[doc = "Field `CC80ST1` reader - Slice 0 compare channel 1 status bit"]
pub type Cc80st1R = crate::BitReader;
#[doc = "Field `CC81ST1` reader - Slice 1 compare channel 1 status bit"]
pub type Cc81st1R = crate::BitReader;
#[doc = "Field `CC82ST1` reader - Slice 2 compare channel 1 status bit"]
pub type Cc82st1R = crate::BitReader;
#[doc = "Field `CC83ST1` reader - Slice 3 compare channel 1 status bit"]
pub type Cc83st1R = crate::BitReader;
#[doc = "Field `CC80ST2` reader - Slice 0 compare channel 2 status bit"]
pub type Cc80st2R = crate::BitReader;
#[doc = "Field `CC81ST2` reader - Slice 1 compare channel 2 status bit"]
pub type Cc81st2R = crate::BitReader;
#[doc = "Field `CC82ST2` reader - Slice 2 compare channel 2 status bit"]
pub type Cc82st2R = crate::BitReader;
#[doc = "Field `CC83ST2` reader - Slice 3 compare channel 2 status bit"]
pub type Cc83st2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Slice 0 shadow transfer status"]
    #[inline(always)]
    pub fn s0ss(&self) -> S0ssR {
        S0ssR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s0dss(&self) -> S0dssR {
        S0dssR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s0pss(&self) -> S0pssR {
        S0pssR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer status"]
    #[inline(always)]
    pub fn s1ss(&self) -> S1ssR {
        S1ssR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s1dss(&self) -> S1dssR {
        S1dssR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s1pss(&self) -> S1pssR {
        S1pssR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer status"]
    #[inline(always)]
    pub fn s2ss(&self) -> S2ssR {
        S2ssR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s2dss(&self) -> S2dssR {
        S2dssR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s2pss(&self) -> S2pssR {
        S2pssR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer status"]
    #[inline(always)]
    pub fn s3ss(&self) -> S3ssR {
        S3ssR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s3dss(&self) -> S3dssR {
        S3dssR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s3pss(&self) -> S3pssR {
        S3pssR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Slice 0 compare channel 1 status bit"]
    #[inline(always)]
    pub fn cc80st1(&self) -> Cc80st1R {
        Cc80st1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slice 1 compare channel 1 status bit"]
    #[inline(always)]
    pub fn cc81st1(&self) -> Cc81st1R {
        Cc81st1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slice 2 compare channel 1 status bit"]
    #[inline(always)]
    pub fn cc82st1(&self) -> Cc82st1R {
        Cc82st1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slice 3 compare channel 1 status bit"]
    #[inline(always)]
    pub fn cc83st1(&self) -> Cc83st1R {
        Cc83st1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Slice 0 compare channel 2 status bit"]
    #[inline(always)]
    pub fn cc80st2(&self) -> Cc80st2R {
        Cc80st2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Slice 1 compare channel 2 status bit"]
    #[inline(always)]
    pub fn cc81st2(&self) -> Cc81st2R {
        Cc81st2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Slice 2 compare channel 2 status bit"]
    #[inline(always)]
    pub fn cc82st2(&self) -> Cc82st2R {
        Cc82st2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slice 3 compare channel 2 status bit"]
    #[inline(always)]
    pub fn cc83st2(&self) -> Cc83st2R {
        Cc83st2R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Global Channel status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcstSpec;
impl crate::RegisterSpec for GcstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcst::R`](R) reader structure"]
impl crate::Readable for GcstSpec {}
#[doc = "`reset()` method sets GCST to value 0"]
impl crate::Resettable for GcstSpec {
    const RESET_VALUE: u32 = 0;
}
