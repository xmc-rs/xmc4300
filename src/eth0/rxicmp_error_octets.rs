#[doc = "Reader of register RXICMP_ERROR_OCTETS"]
pub type R = crate::R<u32, super::RXICMP_ERROR_OCTETS>;
#[doc = "Reader of field `RXICMPERROCT`"]
pub type RXICMPERROCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received in an ICMP segment with checksum errors"]
    #[inline(always)]
    pub fn rxicmperroct(&self) -> RXICMPERROCT_R {
        RXICMPERROCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
