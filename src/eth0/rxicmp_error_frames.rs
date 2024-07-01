#[doc = "Register `RXICMP_ERROR_FRAMES` reader"]
pub type R = crate::R<RXICMP_ERROR_FRAMES_SPEC>;
#[doc = "Field `RXICMPERRFRM` reader - This field indicates the number of good IP datagrams whose ICMP payload has a checksum error."]
pub type RXICMPERRFRM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams whose ICMP payload has a checksum error."]
    #[inline(always)]
    pub fn rxicmperrfrm(&self) -> RXICMPERRFRM_R {
        RXICMPERRFRM_R::new(self.bits)
    }
}
#[doc = "RxICMP Error Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxicmp_error_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXICMP_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RXICMP_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxicmp_error_frames::R`](R) reader structure"]
impl crate::Readable for RXICMP_ERROR_FRAMES_SPEC {}
#[doc = "`reset()` method sets RXICMP_ERROR_FRAMES to value 0"]
impl crate::Resettable for RXICMP_ERROR_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
