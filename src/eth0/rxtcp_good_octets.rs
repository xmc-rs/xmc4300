#[doc = "Register `RXTCP_GOOD_OCTETS` reader"]
pub type R = crate::R<RxtcpGoodOctetsSpec>;
#[doc = "Field `RXTCPGDOCT` reader - This field indicates the number of bytes received in a good TCP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type RxtcpgdoctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a good TCP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxtcpgdoct(&self) -> RxtcpgdoctR {
        RxtcpgdoctR::new(self.bits)
    }
}
#[doc = "Receive TCP Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_good_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtcpGoodOctetsSpec;
impl crate::RegisterSpec for RxtcpGoodOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtcp_good_octets::R`](R) reader structure"]
impl crate::Readable for RxtcpGoodOctetsSpec {}
#[doc = "`reset()` method sets RXTCP_GOOD_OCTETS to value 0"]
impl crate::Resettable for RxtcpGoodOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
