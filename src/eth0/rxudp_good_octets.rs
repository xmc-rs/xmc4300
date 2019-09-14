#[doc = "Reader of register RXUDP_GOOD_OCTETS"]
pub type R = crate::R<u32, super::RXUDP_GOOD_OCTETS>;
#[doc = "Reader of field `RXUDPGDOCT`"]
pub type RXUDPGDOCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a good UDP segment. This counter does not count IP header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxudpgdoct(&self) -> RXUDPGDOCT_R {
        RXUDPGDOCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
