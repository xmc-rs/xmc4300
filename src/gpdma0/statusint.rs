#[doc = "Register `STATUSINT` reader"]
pub struct R(crate::R<STATUSINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERR` reader - OR of the contents of STATUSERR register"]
pub struct ERR_R(crate::FieldReader<bool, bool>);
impl ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSTT` reader - OR of the contents of STATUSDSTTRAN register"]
pub struct DSTT_R(crate::FieldReader<bool, bool>);
impl DSTT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSTT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSTT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRCT` reader - OR of the contents of STATUSSRCTRAN register"]
pub struct SRCT_R(crate::FieldReader<bool, bool>);
impl SRCT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRCT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK` reader - OR of the contents of STATUSBLOCK register"]
pub struct BLOCK_R(crate::FieldReader<bool, bool>);
impl BLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFR` reader - OR of the contents of STATUSTFR register"]
pub struct TFR_R(crate::FieldReader<bool, bool>);
impl TFR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - OR of the contents of STATUSERR register"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OR of the contents of STATUSDSTTRAN register"]
    #[inline(always)]
    pub fn dstt(&self) -> DSTT_R {
        DSTT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OR of the contents of STATUSSRCTRAN register"]
    #[inline(always)]
    pub fn srct(&self) -> SRCT_R {
        SRCT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - OR of the contents of STATUSBLOCK register"]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - OR of the contents of STATUSTFR register"]
    #[inline(always)]
    pub fn tfr(&self) -> TFR_R {
        TFR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Combined Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusint](index.html) module"]
pub struct STATUSINT_SPEC;
impl crate::RegisterSpec for STATUSINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusint::R](R) reader structure"]
impl crate::Readable for STATUSINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSINT to value 0"]
impl crate::Resettable for STATUSINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
