#[doc = "Reader of register RXIPV4_GOOD_OCTETS"]
pub type R = crate::R<u32, super::RXIPV4_GOOD_OCTETS>;
#[doc = "Reader of field `RXIPV4GDOCT`"]
pub type RXIPV4GDOCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. The Ethernet header, FCS, pad, or IP pad"]
    #[inline(always)]
    pub fn rxipv4gdoct(&self) -> RXIPV4GDOCT_R {
        RXIPV4GDOCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
