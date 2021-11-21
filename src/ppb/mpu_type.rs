#[doc = "Register `MPU_TYPE` reader"]
pub struct R(crate::R<MPU_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEPARATE` reader - Support for unified or separate instruction and date memory maps"]
pub struct SEPARATE_R(crate::FieldReader<bool, bool>);
impl SEPARATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEPARATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEPARATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREGION` reader - Number of supported MPU data regions"]
pub struct DREGION_R(crate::FieldReader<u8, u8>);
impl DREGION_R {
    pub(crate) fn new(bits: u8) -> Self {
        DREGION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREGION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREGION` reader - Number of supported MPU instruction regions"]
pub struct IREGION_R(crate::FieldReader<u8, u8>);
impl IREGION_R {
    pub(crate) fn new(bits: u8) -> Self {
        IREGION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IREGION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Support for unified or separate instruction and date memory maps"]
    #[inline(always)]
    pub fn separate(&self) -> SEPARATE_R {
        SEPARATE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Number of supported MPU data regions"]
    #[inline(always)]
    pub fn dregion(&self) -> DREGION_R {
        DREGION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of supported MPU instruction regions"]
    #[inline(always)]
    pub fn iregion(&self) -> IREGION_R {
        IREGION_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "MPU Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_type](index.html) module"]
pub struct MPU_TYPE_SPEC;
impl crate::RegisterSpec for MPU_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_type::R](R) reader structure"]
impl crate::Readable for MPU_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MPU_TYPE to value 0x0800"]
impl crate::Resettable for MPU_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
