#[doc = "Reader of register RXIPV4_FRAGMENTED_OCTETS"]
pub type R = crate::R<u32, super::RXIPV4_FRAGMENTED_OCTETS>;
#[doc = "Reader of field `RXIPV4FRAGOCT`"]
pub type RXIPV4FRAGOCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in fragmented IPv4 datagrams. The value in the IPv4 headers Length field is used to update this counter."]
    #[inline(always)]
    pub fn rxipv4fragoct(&self) -> RXIPV4FRAGOCT_R {
        RXIPV4FRAGOCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
