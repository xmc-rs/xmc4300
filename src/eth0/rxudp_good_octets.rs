#[doc = "Register `RXUDP_GOOD_OCTETS` reader"]
pub struct R(crate::R<RXUDP_GOOD_OCTETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXUDP_GOOD_OCTETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXUDP_GOOD_OCTETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXUDP_GOOD_OCTETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXUDPGDOCT` reader - This field indicates the number of bytes received in a good UDP segment. This counter does not count IP header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type RXUDPGDOCT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a good UDP segment. This counter does not count IP header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxudpgdoct(&self) -> RXUDPGDOCT_R {
        RXUDPGDOCT_R::new(self.bits)
    }
}
#[doc = "Receive UDP Good Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxudp_good_octets](index.html) module"]
pub struct RXUDP_GOOD_OCTETS_SPEC;
impl crate::RegisterSpec for RXUDP_GOOD_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxudp_good_octets::R](R) reader structure"]
impl crate::Readable for RXUDP_GOOD_OCTETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXUDP_GOOD_OCTETS to value 0"]
impl crate::Resettable for RXUDP_GOOD_OCTETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
