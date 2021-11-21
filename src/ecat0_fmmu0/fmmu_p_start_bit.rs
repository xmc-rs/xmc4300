#[doc = "Register `FMMU_P_START_BIT` reader"]
pub struct R(crate::R<FMMU_P_START_BIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMMU_P_START_BIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMMU_P_START_BIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMMU_P_START_BIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P_START_BIT` reader - Physical starting bit as target of logical start bit mapping"]
pub struct P_START_BIT_R(crate::FieldReader<u8, u8>);
impl P_START_BIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        P_START_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_START_BIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Physical starting bit as target of logical start bit mapping"]
    #[inline(always)]
    pub fn p_start_bit(&self) -> P_START_BIT_R {
        P_START_BIT_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Ph0sical Start bit FMMU y\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmmu_p_start_bit](index.html) module"]
pub struct FMMU_P_START_BIT_SPEC;
impl crate::RegisterSpec for FMMU_P_START_BIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fmmu_p_start_bit::R](R) reader structure"]
impl crate::Readable for FMMU_P_START_BIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMMU_P_START_BIT to value 0"]
impl crate::Resettable for FMMU_P_START_BIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
