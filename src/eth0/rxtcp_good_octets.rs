#[doc = "Reader of register RXTCP_GOOD_OCTETS"]
pub type R = crate::R<u32, super::RXTCP_GOOD_OCTETS>;
#[doc = "Reader of field `RXTCPGDOCT`"]
pub type RXTCPGDOCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a good TCP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxtcpgdoct(&self) -> RXTCPGDOCT_R {
        RXTCPGDOCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
