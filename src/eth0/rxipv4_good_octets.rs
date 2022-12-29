#[doc = "Register `RXIPV4_GOOD_OCTETS` reader"]
pub struct R(crate::R<RXIPV4_GOOD_OCTETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIPV4_GOOD_OCTETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIPV4_GOOD_OCTETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIPV4_GOOD_OCTETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXIPV4GDOCT` reader - This field indicates the number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. The Ethernet header, FCS, pad, or IP pad"]
pub type RXIPV4GDOCT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. The Ethernet header, FCS, pad, or IP pad"]
    #[inline(always)]
    pub fn rxipv4gdoct(&self) -> RXIPV4GDOCT_R {
        RXIPV4GDOCT_R::new(self.bits)
    }
}
#[doc = "RxIPv4 Good Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxipv4_good_octets](index.html) module"]
pub struct RXIPV4_GOOD_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV4_GOOD_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxipv4_good_octets::R](R) reader structure"]
impl crate::Readable for RXIPV4_GOOD_OCTETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXIPV4_GOOD_OCTETS to value 0"]
impl crate::Resettable for RXIPV4_GOOD_OCTETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
