#[doc = "Register `SM_P_START_ADR` reader"]
pub struct R(crate::R<SM_P_START_ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM_P_START_ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM_P_START_ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM_P_START_ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIRST_BYTE` reader - Specifies first byte that will be handled by SyncManager"]
pub struct FIRST_BYTE_R(crate::FieldReader<u16, u16>);
impl FIRST_BYTE_R {
    pub(crate) fn new(bits: u16) -> Self {
        FIRST_BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIRST_BYTE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Specifies first byte that will be handled by SyncManager"]
    #[inline(always)]
    pub fn first_byte(&self) -> FIRST_BYTE_R {
        FIRST_BYTE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Physical Start Address SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_p_start_adr](index.html) module"]
pub struct SM_P_START_ADR_SPEC;
impl crate::RegisterSpec for SM_P_START_ADR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sm_p_start_adr::R](R) reader structure"]
impl crate::Readable for SM_P_START_ADR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SM_P_START_ADR to value 0"]
impl crate::Resettable for SM_P_START_ADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
