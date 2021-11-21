#[doc = "Register `OVRSTAT` reader"]
pub struct R(crate::R<OVRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LN0` reader - Line 0 Overrun Status"]
pub struct LN0_R(crate::FieldReader<bool, bool>);
impl LN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LN1` reader - Line 1 Overrun Status"]
pub struct LN1_R(crate::FieldReader<bool, bool>);
impl LN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LN2` reader - Line 2 Overrun Status"]
pub struct LN2_R(crate::FieldReader<bool, bool>);
impl LN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LN3` reader - Line 3 Overrun Status"]
pub struct LN3_R(crate::FieldReader<bool, bool>);
impl LN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LN4` reader - Line 4 Overrun Status"]
pub struct LN4_R(crate::FieldReader<bool, bool>);
impl LN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        LN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LN5` reader - Line 5 Overrun Status"]
pub struct LN5_R(crate::FieldReader<bool, bool>);
impl LN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        LN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LN6` reader - Line 6 Overrun Status"]
pub struct LN6_R(crate::FieldReader<bool, bool>);
impl LN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        LN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LN7` reader - Line 7 Overrun Status"]
pub struct LN7_R(crate::FieldReader<bool, bool>);
impl LN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        LN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Line 0 Overrun Status"]
    #[inline(always)]
    pub fn ln0(&self) -> LN0_R {
        LN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Line 1 Overrun Status"]
    #[inline(always)]
    pub fn ln1(&self) -> LN1_R {
        LN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Line 2 Overrun Status"]
    #[inline(always)]
    pub fn ln2(&self) -> LN2_R {
        LN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Line 3 Overrun Status"]
    #[inline(always)]
    pub fn ln3(&self) -> LN3_R {
        LN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Line 4 Overrun Status"]
    #[inline(always)]
    pub fn ln4(&self) -> LN4_R {
        LN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Line 5 Overrun Status"]
    #[inline(always)]
    pub fn ln5(&self) -> LN5_R {
        LN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Line 6 Overrun Status"]
    #[inline(always)]
    pub fn ln6(&self) -> LN6_R {
        LN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Line 7 Overrun Status"]
    #[inline(always)]
    pub fn ln7(&self) -> LN7_R {
        LN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Overrun Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovrstat](index.html) module"]
pub struct OVRSTAT_SPEC;
impl crate::RegisterSpec for OVRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ovrstat::R](R) reader structure"]
impl crate::Readable for OVRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OVRSTAT to value 0"]
impl crate::Resettable for OVRSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
