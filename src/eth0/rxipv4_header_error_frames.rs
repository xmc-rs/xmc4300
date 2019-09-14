#[doc = "Reader of register RXIPV4_HEADER_ERROR_FRAMES"]
pub type R = crate::R<u32, super::RXIPV4_HEADER_ERROR_FRAMES>;
#[doc = "Reader of field `RXIPV4HDRERRFRM`"]
pub type RXIPV4HDRERRFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of IPv4 datagrams received with header errors (checksum, length, or version mismatch)."]
    #[inline(always)]
    pub fn rxipv4hdrerrfrm(&self) -> RXIPV4HDRERRFRM_R {
        RXIPV4HDRERRFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
