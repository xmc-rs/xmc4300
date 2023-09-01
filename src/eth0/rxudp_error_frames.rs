#[doc = "Register `RXUDP_ERROR_FRAMES` reader"]
pub type R = crate::R<RXUDP_ERROR_FRAMES_SPEC>;
#[doc = "Field `RXUDPERRFRM` reader - This field indicates the number of good IP datagrams whose UDP payload has a checksum error."]
pub type RXUDPERRFRM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams whose UDP payload has a checksum error."]
    #[inline(always)]
    pub fn rxudperrfrm(&self) -> RXUDPERRFRM_R {
        RXUDPERRFRM_R::new(self.bits)
    }
}
#[doc = "RxUDP Error Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUDP_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RXUDP_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxudp_error_frames::R`](R) reader structure"]
impl crate::Readable for RXUDP_ERROR_FRAMES_SPEC {}
#[doc = "`reset()` method sets RXUDP_ERROR_FRAMES to value 0"]
impl crate::Resettable for RXUDP_ERROR_FRAMES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
