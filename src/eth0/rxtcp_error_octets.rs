#[doc = "Register `RXTCP_ERROR_OCTETS` reader"]
pub type R = crate::R<RxtcpErrorOctetsSpec>;
#[doc = "Field `RXTCPERROCT` reader - Thsi field indicates the number of bytes received in a TCP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type RxtcperroctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Thsi field indicates the number of bytes received in a TCP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxtcperroct(&self) -> RxtcperroctR {
        RxtcperroctR::new(self.bits)
    }
}
#[doc = "Receive TCP Error Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_error_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtcpErrorOctetsSpec;
impl crate::RegisterSpec for RxtcpErrorOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtcp_error_octets::R`](R) reader structure"]
impl crate::Readable for RxtcpErrorOctetsSpec {}
#[doc = "`reset()` method sets RXTCP_ERROR_OCTETS to value 0"]
impl crate::Resettable for RxtcpErrorOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
