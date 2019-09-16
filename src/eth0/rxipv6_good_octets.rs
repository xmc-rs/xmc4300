#[doc = "Reader of register RXIPV6_GOOD_OCTETS"]
pub type R = crate::R<u32, super::RXIPV6_GOOD_OCTETS>;
#[doc = "Reader of field `RXIPV6GDOCT`"]
pub type RXIPV6GDOCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Thsi field indicates the number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data."]
    #[inline(always)]
    pub fn rxipv6gdoct(&self) -> RXIPV6GDOCT_R {
        RXIPV6GDOCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
