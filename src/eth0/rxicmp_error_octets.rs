#[doc = "Register `RXICMP_ERROR_OCTETS` reader"]
pub struct R(crate::R<RXICMP_ERROR_OCTETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXICMP_ERROR_OCTETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXICMP_ERROR_OCTETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXICMP_ERROR_OCTETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXICMPERROCT` reader - Number of bytes received in an ICMP segment with checksum errors"]
pub struct RXICMPERROCT_R(crate::FieldReader<u32, u32>);
impl RXICMPERROCT_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXICMPERROCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXICMPERROCT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of bytes received in an ICMP segment with checksum errors"]
    #[inline(always)]
    pub fn rxicmperroct(&self) -> RXICMPERROCT_R {
        RXICMPERROCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive ICMP Error Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxicmp_error_octets](index.html) module"]
pub struct RXICMP_ERROR_OCTETS_SPEC;
impl crate::RegisterSpec for RXICMP_ERROR_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxicmp_error_octets::R](R) reader structure"]
impl crate::Readable for RXICMP_ERROR_OCTETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXICMP_ERROR_OCTETS to value 0"]
impl crate::Resettable for RXICMP_ERROR_OCTETS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
