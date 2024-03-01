#[doc = "Register `GSTAT` reader"]
pub type R = crate::R<GstatSpec>;
#[doc = "CC80 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0i {
    #[doc = "0: Running"]
    Value1 = 0,
    #[doc = "1: Idle"]
    Value2 = 1,
}
impl From<S0i> for bool {
    #[inline(always)]
    fn from(variant: S0i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0I` reader - CC80 IDLE status"]
pub type S0iR = crate::BitReader<S0i>;
impl S0iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0i {
        match self.bits {
            false => S0i::Value1,
            true => S0i::Value2,
        }
    }
    #[doc = "Running"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0i::Value1
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0i::Value2
    }
}
#[doc = "CC81 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1i {
    #[doc = "0: Running"]
    Value1 = 0,
    #[doc = "1: Idle"]
    Value2 = 1,
}
impl From<S1i> for bool {
    #[inline(always)]
    fn from(variant: S1i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1I` reader - CC81 IDLE status"]
pub type S1iR = crate::BitReader<S1i>;
impl S1iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1i {
        match self.bits {
            false => S1i::Value1,
            true => S1i::Value2,
        }
    }
    #[doc = "Running"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1i::Value1
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1i::Value2
    }
}
#[doc = "CC82 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2i {
    #[doc = "0: Running"]
    Value1 = 0,
    #[doc = "1: Idle"]
    Value2 = 1,
}
impl From<S2i> for bool {
    #[inline(always)]
    fn from(variant: S2i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2I` reader - CC82 IDLE status"]
pub type S2iR = crate::BitReader<S2i>;
impl S2iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2i {
        match self.bits {
            false => S2i::Value1,
            true => S2i::Value2,
        }
    }
    #[doc = "Running"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2i::Value1
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2i::Value2
    }
}
#[doc = "CC83 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3i {
    #[doc = "0: Running"]
    Value1 = 0,
    #[doc = "1: Idle"]
    Value2 = 1,
}
impl From<S3i> for bool {
    #[inline(always)]
    fn from(variant: S3i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3I` reader - CC83 IDLE status"]
pub type S3iR = crate::BitReader<S3i>;
impl S3iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3i {
        match self.bits {
            false => S3i::Value1,
            true => S3i::Value2,
        }
    }
    #[doc = "Running"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3i::Value1
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3i::Value2
    }
}
#[doc = "Prescaler Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prb {
    #[doc = "0: Prescaler is stopped"]
    Value1 = 0,
    #[doc = "1: Prescaler is running"]
    Value2 = 1,
}
impl From<Prb> for bool {
    #[inline(always)]
    fn from(variant: Prb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRB` reader - Prescaler Run Bit"]
pub type PrbR = crate::BitReader<Prb>;
impl PrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prb {
        match self.bits {
            false => Prb::Value1,
            true => Prb::Value2,
        }
    }
    #[doc = "Prescaler is stopped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Prb::Value1
    }
    #[doc = "Prescaler is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Prb::Value2
    }
}
#[doc = "Parity Checker Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcrb {
    #[doc = "0: Parity Checker is stopped"]
    Value1 = 0,
    #[doc = "1: Parity Checker is running"]
    Value2 = 1,
}
impl From<Pcrb> for bool {
    #[inline(always)]
    fn from(variant: Pcrb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCRB` reader - Parity Checker Run Bit"]
pub type PcrbR = crate::BitReader<Pcrb>;
impl PcrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcrb {
        match self.bits {
            false => Pcrb::Value1,
            true => Pcrb::Value2,
        }
    }
    #[doc = "Parity Checker is stopped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pcrb::Value1
    }
    #[doc = "Parity Checker is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pcrb::Value2
    }
}
impl R {
    #[doc = "Bit 0 - CC80 IDLE status"]
    #[inline(always)]
    pub fn s0i(&self) -> S0iR {
        S0iR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC81 IDLE status"]
    #[inline(always)]
    pub fn s1i(&self) -> S1iR {
        S1iR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC82 IDLE status"]
    #[inline(always)]
    pub fn s2i(&self) -> S2iR {
        S2iR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC83 IDLE status"]
    #[inline(always)]
    pub fn s3i(&self) -> S3iR {
        S3iR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Prescaler Run Bit"]
    #[inline(always)]
    pub fn prb(&self) -> PrbR {
        PrbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity Checker Run Bit"]
    #[inline(always)]
    pub fn pcrb(&self) -> PcrbR {
        PcrbR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Global Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GstatSpec;
impl crate::RegisterSpec for GstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gstat::R`](R) reader structure"]
impl crate::Readable for GstatSpec {}
#[doc = "`reset()` method sets GSTAT to value 0x0f"]
impl crate::Resettable for GstatSpec {
    const RESET_VALUE: u32 = 0x0f;
}
