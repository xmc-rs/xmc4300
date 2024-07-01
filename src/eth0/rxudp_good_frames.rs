#[doc = "Register `RXUDP_GOOD_FRAMES` reader"]
pub type R = crate::R<RXUDP_GOOD_FRAMES_SPEC>;
#[doc = "Field `RXUDPGDFRM` reader - This field indicates the number of good IP datagrams with a good UDP payload. This counter is not updated when the counter is incremented."]
pub type RXUDPGDFRM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams with a good UDP payload. This counter is not updated when the counter is incremented."]
    #[inline(always)]
    pub fn rxudpgdfrm(&self) -> RXUDPGDFRM_R {
        RXUDPGDFRM_R::new(self.bits)
    }
}
#[doc = "RxUDP Good Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxudp_good_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUDP_GOOD_FRAMES_SPEC;
impl crate::RegisterSpec for RXUDP_GOOD_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxudp_good_frames::R`](R) reader structure"]
impl crate::Readable for RXUDP_GOOD_FRAMES_SPEC {}
#[doc = "`reset()` method sets RXUDP_GOOD_FRAMES to value 0"]
impl crate::Resettable for RXUDP_GOOD_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
