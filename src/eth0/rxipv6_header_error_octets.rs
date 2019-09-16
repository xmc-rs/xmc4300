#[doc = "Reader of register RXIPV6_HEADER_ERROR_OCTETS"]
pub type R = crate::R<u32, super::RXIPV6_HEADER_ERROR_OCTETS>;
#[doc = "Reader of field `RXIPV6HDRERROCT`"]
pub type RXIPV6HDRERROCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in IPv6 datagrams with header errors (length or version mismatch). The value in the IPv6 headers Length field is used to update this counter."]
    #[inline(always)]
    pub fn rxipv6hdrerroct(&self) -> RXIPV6HDRERROCT_R {
        RXIPV6HDRERROCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
