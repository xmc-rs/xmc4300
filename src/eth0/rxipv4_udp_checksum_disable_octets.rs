#[doc = "Reader of register RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS"]
pub type R = crate::R<u32, super::RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS>;
#[doc = "Reader of field `RXIPV4UDSBLOCT`"]
pub type RXIPV4UDSBLOCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a UDP segment that had the UDP checksum disabled. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxipv4udsbloct(&self) -> RXIPV4UDSBLOCT_R {
        RXIPV4UDSBLOCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
