#[doc = "Reader of register RXTCP_ERROR_OCTETS"]
pub type R = crate::R<u32, super::RXTCP_ERROR_OCTETS>;
#[doc = "Reader of field `RXTCPERROCT`"]
pub type RXTCPERROCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Thsi field indicates the number of bytes received in a TCP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxtcperroct(&self) -> RXTCPERROCT_R {
        RXTCPERROCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
