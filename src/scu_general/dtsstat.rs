#[doc = "Register `DTSSTAT` reader"]
pub struct R(crate::R<DTSSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTSSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTSSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTSSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - Result of the DTS Measurement"]
pub type RESULT_R = crate::FieldReader<u16>;
#[doc = "Field `RDY` reader - Sensor Ready Status"]
pub type RDY_R = crate::BitReader<RDY_A>;
#[doc = "Sensor Ready Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY_A {
    #[doc = "0: The DTS is not ready"]
    CONST_0 = 0,
    #[doc = "1: The DTS is ready"]
    CONST_1 = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::CONST_0,
            true => RDY_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RDY_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RDY_A::CONST_1
    }
}
#[doc = "Field `BUSY` reader - Sensor Busy Status"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Sensor Busy Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: not busy"]
    CONST_0 = 0,
    #[doc = "1: busy"]
    CONST_1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::CONST_0,
            true => BUSY_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BUSY_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BUSY_A::CONST_1
    }
}
impl R {
    #[doc = "Bits 0:9 - Result of the DTS Measurement"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - Sensor Ready Status"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Sensor Busy Status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Die Temperature Sensor Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtsstat](index.html) module"]
pub struct DTSSTAT_SPEC;
impl crate::RegisterSpec for DTSSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtsstat::R](R) reader structure"]
impl crate::Readable for DTSSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTSSTAT to value 0"]
impl crate::Resettable for DTSSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
