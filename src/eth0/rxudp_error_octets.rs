#[doc = "Reader of register RXUDP_ERROR_OCTETS"]
pub type R = crate::R<u32, super::RXUDP_ERROR_OCTETS>;
#[doc = "Reader of field `RXUDPERROCT`"]
pub type RXUDPERROCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a UDP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxudperroct(&self) -> RXUDPERROCT_R {
        RXUDPERROCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
