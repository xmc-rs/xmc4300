#[doc = "Register `RXIPV4_NO_PAYLOAD_OCTETS` reader"]
pub type R = crate::R<RXIPV4_NO_PAYLOAD_OCTETS_SPEC>;
#[doc = "Field `RXIPV4NOPAYOCT` reader - This field indicates the number of bytes received in IPv4 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv4 headers Length field is used to update this counter."]
pub type RXIPV4NOPAYOCT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in IPv4 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv4 headers Length field is used to update this counter."]
    #[inline(always)]
    pub fn rxipv4nopayoct(&self) -> RXIPV4NOPAYOCT_R {
        RXIPV4NOPAYOCT_R::new(self.bits)
    }
}
#[doc = "Receive IPV4 No Payload Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_no_payload_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV4_NO_PAYLOAD_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV4_NO_PAYLOAD_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_no_payload_octets::R`](R) reader structure"]
impl crate::Readable for RXIPV4_NO_PAYLOAD_OCTETS_SPEC {}
#[doc = "`reset()` method sets RXIPV4_NO_PAYLOAD_OCTETS to value 0"]
impl crate::Resettable for RXIPV4_NO_PAYLOAD_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
