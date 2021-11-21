#[doc = "Register `FMMU_LEN` reader"]
pub struct R(crate::R<FMMU_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMMU_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMMU_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMMU_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFFSET` reader - Offset from the first logical FMMU Byte to the last FMMU Byte + 1"]
pub struct OFFSET_R(crate::FieldReader<u16, u16>);
impl OFFSET_R {
    pub(crate) fn new(bits: u16) -> Self {
        OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Offset from the first logical FMMU Byte to the last FMMU Byte + 1"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Length FMMU 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmmu_len](index.html) module"]
pub struct FMMU_LEN_SPEC;
impl crate::RegisterSpec for FMMU_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fmmu_len::R](R) reader structure"]
impl crate::Readable for FMMU_LEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMMU_LEN to value 0"]
impl crate::Resettable for FMMU_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
