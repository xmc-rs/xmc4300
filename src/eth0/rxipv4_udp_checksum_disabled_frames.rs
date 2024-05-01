#[doc = "Register `RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES` reader"]
pub type R = crate::R<Rxipv4UdpChecksumDisabledFramesSpec>;
#[doc = "Field `RXIPV4UDSBLFRM` reader - This field indicates the number of received good IPv4 datagrams which have the UDP payload with checksum disabled."]
pub type Rxipv4udsblfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good IPv4 datagrams which have the UDP payload with checksum disabled."]
    #[inline(always)]
    pub fn rxipv4udsblfrm(&self) -> Rxipv4udsblfrmR {
        Rxipv4udsblfrmR::new(self.bits)
    }
}
#[doc = "Receive IPV4 UDP Checksum Disabled Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_udp_checksum_disabled_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv4UdpChecksumDisabledFramesSpec;
impl crate::RegisterSpec for Rxipv4UdpChecksumDisabledFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_udp_checksum_disabled_frames::R`](R) reader structure"]
impl crate::Readable for Rxipv4UdpChecksumDisabledFramesSpec {}
#[doc = "`reset()` method sets RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES to value 0"]
impl crate::Resettable for Rxipv4UdpChecksumDisabledFramesSpec {
    const RESET_VALUE: u32 = 0;
}
