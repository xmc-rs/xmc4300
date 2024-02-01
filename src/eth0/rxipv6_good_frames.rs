#[doc = "Register `RXIPV6_GOOD_FRAMES` reader"]
pub type R = crate::R<RXIPV6_GOOD_FRAMES_SPEC>;
#[doc = "Field `RXIPV6GDFRM` reader - This field indicates the number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads."]
pub type RXIPV6GDFRM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads."]
    #[inline(always)]
    pub fn rxipv6gdfrm(&self) -> RXIPV6GDFRM_R {
        RXIPV6GDFRM_R::new(self.bits)
    }
}
#[doc = "RxIPv6 Good Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_good_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV6_GOOD_FRAMES_SPEC;
impl crate::RegisterSpec for RXIPV6_GOOD_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_good_frames::R`](R) reader structure"]
impl crate::Readable for RXIPV6_GOOD_FRAMES_SPEC {}
#[doc = "`reset()` method sets RXIPV6_GOOD_FRAMES to value 0"]
impl crate::Resettable for RXIPV6_GOOD_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
