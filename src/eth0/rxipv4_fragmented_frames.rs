#[doc = "Register `RXIPV4_FRAGMENTED_FRAMES` reader"]
pub type R = crate::R<RXIPV4_FRAGMENTED_FRAMES_SPEC>;
#[doc = "Field `RXIPV4FRAGFRM` reader - This field indicates the number of good IPv4 datagrams received with fragmentation."]
pub type RXIPV4FRAGFRM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IPv4 datagrams received with fragmentation."]
    #[inline(always)]
    pub fn rxipv4fragfrm(&self) -> RXIPV4FRAGFRM_R {
        RXIPV4FRAGFRM_R::new(self.bits)
    }
}
#[doc = "Receive IPV4 Fragmented Frame Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_fragmented_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV4_FRAGMENTED_FRAMES_SPEC;
impl crate::RegisterSpec for RXIPV4_FRAGMENTED_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_fragmented_frames::R`](R) reader structure"]
impl crate::Readable for RXIPV4_FRAGMENTED_FRAMES_SPEC {}
#[doc = "`reset()` method sets RXIPV4_FRAGMENTED_FRAMES to value 0"]
impl crate::Resettable for RXIPV4_FRAGMENTED_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
