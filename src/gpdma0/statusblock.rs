#[doc = "Register `STATUSBLOCK` reader"]
pub struct R(crate::R<STATUSBLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSBLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSBLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSBLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0` reader - Interrupt Status for channel 0"]
pub struct CH0_R(crate::FieldReader<bool, bool>);
impl CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1` reader - Interrupt Status for channel 1"]
pub struct CH1_R(crate::FieldReader<bool, bool>);
impl CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2` reader - Interrupt Status for channel 2"]
pub struct CH2_R(crate::FieldReader<bool, bool>);
impl CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3` reader - Interrupt Status for channel 3"]
pub struct CH3_R(crate::FieldReader<bool, bool>);
impl CH3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4` reader - Interrupt Status for channel 4"]
pub struct CH4_R(crate::FieldReader<bool, bool>);
impl CH4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5` reader - Interrupt Status for channel 5"]
pub struct CH5_R(crate::FieldReader<bool, bool>);
impl CH5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6` reader - Interrupt Status for channel 6"]
pub struct CH6_R(crate::FieldReader<bool, bool>);
impl CH6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7` reader - Interrupt Status for channel 7"]
pub struct CH7_R(crate::FieldReader<bool, bool>);
impl CH7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status for channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status for channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status for channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status for channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "IntBlock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusblock](index.html) module"]
pub struct STATUSBLOCK_SPEC;
impl crate::RegisterSpec for STATUSBLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusblock::R](R) reader structure"]
impl crate::Readable for STATUSBLOCK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSBLOCK to value 0"]
impl crate::Resettable for STATUSBLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
