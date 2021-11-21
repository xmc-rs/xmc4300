#[doc = "Register `RXIPV6_GOOD_OCTETS` reader"]
pub struct R(crate::R<RXIPV6_GOOD_OCTETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIPV6_GOOD_OCTETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIPV6_GOOD_OCTETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIPV6_GOOD_OCTETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXIPV6GDOCT` reader - Thsi field indicates the number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data."]
pub struct RXIPV6GDOCT_R(crate::FieldReader<u32, u32>);
impl RXIPV6GDOCT_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXIPV6GDOCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6GDOCT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Thsi field indicates the number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data."]
    #[inline(always)]
    pub fn rxipv6gdoct(&self) -> RXIPV6GDOCT_R {
        RXIPV6GDOCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RxIPv6 Good Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxipv6_good_octets](index.html) module"]
pub struct RXIPV6_GOOD_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV6_GOOD_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxipv6_good_octets::R](R) reader structure"]
impl crate::Readable for RXIPV6_GOOD_OCTETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXIPV6_GOOD_OCTETS to value 0"]
impl crate::Resettable for RXIPV6_GOOD_OCTETS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
