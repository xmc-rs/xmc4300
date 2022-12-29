#[doc = "Register `RXTCP_ERROR_OCTETS` reader"]
pub struct R(crate::R<RXTCP_ERROR_OCTETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXTCP_ERROR_OCTETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXTCP_ERROR_OCTETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXTCP_ERROR_OCTETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXTCPERROCT` reader - Thsi field indicates the number of bytes received in a TCP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type RXTCPERROCT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Thsi field indicates the number of bytes received in a TCP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxtcperroct(&self) -> RXTCPERROCT_R {
        RXTCPERROCT_R::new(self.bits)
    }
}
#[doc = "Receive TCP Error Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtcp_error_octets](index.html) module"]
pub struct RXTCP_ERROR_OCTETS_SPEC;
impl crate::RegisterSpec for RXTCP_ERROR_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxtcp_error_octets::R](R) reader structure"]
impl crate::Readable for RXTCP_ERROR_OCTETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXTCP_ERROR_OCTETS to value 0"]
impl crate::Resettable for RXTCP_ERROR_OCTETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
