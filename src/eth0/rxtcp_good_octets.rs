#[doc = "Register `RXTCP_GOOD_OCTETS` reader"]
pub struct R(crate::R<RXTCP_GOOD_OCTETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXTCP_GOOD_OCTETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXTCP_GOOD_OCTETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXTCP_GOOD_OCTETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXTCPGDOCT` reader - This field indicates the number of bytes received in a good TCP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub struct RXTCPGDOCT_R(crate::FieldReader<u32, u32>);
impl RXTCPGDOCT_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXTCPGDOCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTCPGDOCT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a good TCP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxtcpgdoct(&self) -> RXTCPGDOCT_R {
        RXTCPGDOCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive TCP Good Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtcp_good_octets](index.html) module"]
pub struct RXTCP_GOOD_OCTETS_SPEC;
impl crate::RegisterSpec for RXTCP_GOOD_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxtcp_good_octets::R](R) reader structure"]
impl crate::Readable for RXTCP_GOOD_OCTETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXTCP_GOOD_OCTETS to value 0"]
impl crate::Resettable for RXTCP_GOOD_OCTETS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
