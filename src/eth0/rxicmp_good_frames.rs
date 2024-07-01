#[doc = "Register `RXICMP_GOOD_FRAMES` reader"]
pub type R = crate::R<RXICMP_GOOD_FRAMES_SPEC>;
#[doc = "Field `RXICMPGDFRM` reader - This field indicates the number of good IP datagrams with a good ICMP payload."]
pub type RXICMPGDFRM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams with a good ICMP payload."]
    #[inline(always)]
    pub fn rxicmpgdfrm(&self) -> RXICMPGDFRM_R {
        RXICMPGDFRM_R::new(self.bits)
    }
}
#[doc = "RxICMP Good Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxicmp_good_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXICMP_GOOD_FRAMES_SPEC;
impl crate::RegisterSpec for RXICMP_GOOD_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxicmp_good_frames::R`](R) reader structure"]
impl crate::Readable for RXICMP_GOOD_FRAMES_SPEC {}
#[doc = "`reset()` method sets RXICMP_GOOD_FRAMES to value 0"]
impl crate::Resettable for RXICMP_GOOD_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
