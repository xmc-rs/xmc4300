#[doc = "Reader of register RXICMP_GOOD_OCTETS"]
pub type R = crate::R<u32, super::RXICMP_GOOD_OCTETS>;
#[doc = "Reader of field `RXICMPGDOCT`"]
pub type RXICMPGDOCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a good ICMP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxicmpgdoct(&self) -> RXICMPGDOCT_R {
        RXICMPGDOCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
