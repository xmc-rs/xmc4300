#[doc = "Register `RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS` reader"]
pub type R = crate::R<Rxipv4UdpChecksumDisableOctetsSpec>;
#[doc = "Field `RXIPV4UDSBLOCT` reader - This field indicates the number of bytes received in a UDP segment that had the UDP checksum disabled. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type Rxipv4udsbloctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a UDP segment that had the UDP checksum disabled. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxipv4udsbloct(&self) -> Rxipv4udsbloctR {
        Rxipv4udsbloctR::new(self.bits)
    }
}
#[doc = "Receive IPV4 Fragmented Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_udp_checksum_disable_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv4UdpChecksumDisableOctetsSpec;
impl crate::RegisterSpec for Rxipv4UdpChecksumDisableOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_udp_checksum_disable_octets::R`](R) reader structure"]
impl crate::Readable for Rxipv4UdpChecksumDisableOctetsSpec {}
#[doc = "`reset()` method sets RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS to value 0"]
impl crate::Resettable for Rxipv4UdpChecksumDisableOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
