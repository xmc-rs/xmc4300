#[doc = "Register `RXTCP_GOOD_OCTETS` reader"]
pub type R = crate::R<RXTCP_GOOD_OCTETS_SPEC>;
#[doc = "Field `RXTCPGDOCT` reader - This field indicates the number of bytes received in a good TCP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type RXTCPGDOCT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a good TCP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxtcpgdoct(&self) -> RXTCPGDOCT_R {
        RXTCPGDOCT_R::new(self.bits)
    }
}
#[doc = "Receive TCP Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_good_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXTCP_GOOD_OCTETS_SPEC;
impl crate::RegisterSpec for RXTCP_GOOD_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtcp_good_octets::R`](R) reader structure"]
impl crate::Readable for RXTCP_GOOD_OCTETS_SPEC {}
#[doc = "`reset()` method sets RXTCP_GOOD_OCTETS to value 0"]
impl crate::Resettable for RXTCP_GOOD_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
