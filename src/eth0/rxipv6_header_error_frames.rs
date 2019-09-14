#[doc = "Reader of register RXIPV6_HEADER_ERROR_FRAMES"]
pub type R = crate::R<u32, super::RXIPV6_HEADER_ERROR_FRAMES>;
#[doc = "Reader of field `RXIPV6HDRERRFRM`"]
pub type RXIPV6HDRERRFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of IPv6 datagrams received with header errors (length or version mismatch)."]
    #[inline(always)]
    pub fn rxipv6hdrerrfrm(&self) -> RXIPV6HDRERRFRM_R {
        RXIPV6HDRERRFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
