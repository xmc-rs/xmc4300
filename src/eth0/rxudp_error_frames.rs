#[doc = "Register `RXUDP_ERROR_FRAMES` reader"]
pub struct R(crate::R<RXUDP_ERROR_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXUDP_ERROR_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXUDP_ERROR_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXUDP_ERROR_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXUDPERRFRM` reader - This field indicates the number of good IP datagrams whose UDP payload has a checksum error."]
pub type RXUDPERRFRM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams whose UDP payload has a checksum error."]
    #[inline(always)]
    pub fn rxudperrfrm(&self) -> RXUDPERRFRM_R {
        RXUDPERRFRM_R::new(self.bits)
    }
}
#[doc = "RxUDP Error Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxudp_error_frames](index.html) module"]
pub struct RXUDP_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RXUDP_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxudp_error_frames::R](R) reader structure"]
impl crate::Readable for RXUDP_ERROR_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXUDP_ERROR_FRAMES to value 0"]
impl crate::Resettable for RXUDP_ERROR_FRAMES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
