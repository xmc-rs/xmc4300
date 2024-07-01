#[doc = "Register `RXIPV6_GOOD_OCTETS` reader"]
pub type R = crate::R<RXIPV6_GOOD_OCTETS_SPEC>;
#[doc = "Field `RXIPV6GDOCT` reader - Thsi field indicates the number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data."]
pub type RXIPV6GDOCT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Thsi field indicates the number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data."]
    #[inline(always)]
    pub fn rxipv6gdoct(&self) -> RXIPV6GDOCT_R {
        RXIPV6GDOCT_R::new(self.bits)
    }
}
#[doc = "RxIPv6 Good Octets Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv6_good_octets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV6_GOOD_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV6_GOOD_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_good_octets::R`](R) reader structure"]
impl crate::Readable for RXIPV6_GOOD_OCTETS_SPEC {}
#[doc = "`reset()` method sets RXIPV6_GOOD_OCTETS to value 0"]
impl crate::Resettable for RXIPV6_GOOD_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
