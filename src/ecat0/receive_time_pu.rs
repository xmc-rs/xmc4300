#[doc = "Register `RECEIVE_TIME_PU[%s]` reader"]
pub struct R(crate::R<RECEIVE_TIME_PU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_TIME_PU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_TIME_PU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_TIME_PU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECEIVE_TIME_PU` reader - Local time of the beginning of a frame"]
pub type RECEIVE_TIME_PU_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Local time of the beginning of a frame"]
    #[inline(always)]
    pub fn receive_time_pu(&self) -> RECEIVE_TIME_PU_R {
        RECEIVE_TIME_PU_R::new(self.bits)
    }
}
#[doc = "Local time of the beginning of a frame\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_time_pu](index.html) module"]
pub struct RECEIVE_TIME_PU_SPEC;
impl crate::RegisterSpec for RECEIVE_TIME_PU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_time_pu::R](R) reader structure"]
impl crate::Readable for RECEIVE_TIME_PU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RECEIVE_TIME_PU[%s]
to value 0"]
impl crate::Resettable for RECEIVE_TIME_PU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
