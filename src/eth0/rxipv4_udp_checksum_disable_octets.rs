#[doc = "Register `RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS` reader"]
pub type R = crate::R<RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS_SPEC>;
#[doc = "Field `RXIPV4UDSBLOCT` reader - This field indicates the number of bytes received in a UDP segment that had the UDP checksum disabled. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type RXIPV4UDSBLOCT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a UDP segment that had the UDP checksum disabled. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxipv4udsbloct(&self) -> RXIPV4UDSBLOCT_R {
        RXIPV4UDSBLOCT_R::new(self.bits)
    }
}
#[doc = "Receive IPV4 Fragmented Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_udp_checksum_disable_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_udp_checksum_disable_octets::R`](R) reader structure"]
impl crate::Readable for RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS_SPEC {}
#[doc = "`reset()` method sets RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS to value 0"]
impl crate::Resettable for RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
