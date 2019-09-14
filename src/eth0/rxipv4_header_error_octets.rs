#[doc = "Reader of register RXIPV4_HEADER_ERROR_OCTETS"]
pub type R = crate::R<u32, super::RXIPV4_HEADER_ERROR_OCTETS>;
#[doc = "Reader of field `RXIPV4HDRERROCT`"]
pub type RXIPV4HDRERROCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in the IPv4 datagrams with header errors (checksum, length, or version mismatch). The value in the Length field of IPv4 header is used to update this counter."]
    #[inline(always)]
    pub fn rxipv4hdrerroct(&self) -> RXIPV4HDRERROCT_R {
        RXIPV4HDRERROCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
