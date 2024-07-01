#[doc = "Register `RXIPV4_NO_PAYLOAD_FRAMES` reader"]
pub type R = crate::R<RXIPV4_NO_PAYLOAD_FRAMES_SPEC>;
#[doc = "Field `RXIPV4NOPAYFRM` reader - This field indicates the number of IPv4 datagram frames received that did not have a TCP, UDP, or ICMP payload processed by the Checksum engine."]
pub type RXIPV4NOPAYFRM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of IPv4 datagram frames received that did not have a TCP, UDP, or ICMP payload processed by the Checksum engine."]
    #[inline(always)]
    pub fn rxipv4nopayfrm(&self) -> RXIPV4NOPAYFRM_R {
        RXIPV4NOPAYFRM_R::new(self.bits)
    }
}
#[doc = "Receive IPV4 No Payload Frame Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_no_payload_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV4_NO_PAYLOAD_FRAMES_SPEC;
impl crate::RegisterSpec for RXIPV4_NO_PAYLOAD_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_no_payload_frames::R`](R) reader structure"]
impl crate::Readable for RXIPV4_NO_PAYLOAD_FRAMES_SPEC {}
#[doc = "`reset()` method sets RXIPV4_NO_PAYLOAD_FRAMES to value 0"]
impl crate::Resettable for RXIPV4_NO_PAYLOAD_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
