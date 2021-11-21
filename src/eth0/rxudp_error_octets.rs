#[doc = "Register `RXUDP_ERROR_OCTETS` reader"]
pub struct R(crate::R<RXUDP_ERROR_OCTETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXUDP_ERROR_OCTETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXUDP_ERROR_OCTETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXUDP_ERROR_OCTETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXUDPERROCT` reader - This field indicates the number of bytes received in a UDP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub struct RXUDPERROCT_R(crate::FieldReader<u32, u32>);
impl RXUDPERROCT_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXUDPERROCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUDPERROCT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a UDP segment with checksum errors. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxudperroct(&self) -> RXUDPERROCT_R {
        RXUDPERROCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive UDP Error Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxudp_error_octets](index.html) module"]
pub struct RXUDP_ERROR_OCTETS_SPEC;
impl crate::RegisterSpec for RXUDP_ERROR_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxudp_error_octets::R](R) reader structure"]
impl crate::Readable for RXUDP_ERROR_OCTETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXUDP_ERROR_OCTETS to value 0"]
impl crate::Resettable for RXUDP_ERROR_OCTETS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
