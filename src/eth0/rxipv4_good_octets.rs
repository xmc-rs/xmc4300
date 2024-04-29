#[doc = "Register `RXIPV4_GOOD_OCTETS` reader"]
pub type R = crate::R<RXIPV4_GOOD_OCTETS_SPEC>;
#[doc = "Field `RXIPV4GDOCT` reader - This field indicates the number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. The Ethernet header, FCS, pad, or IP pad"]
pub type RXIPV4GDOCT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. The Ethernet header, FCS, pad, or IP pad"]
    #[inline(always)]
    pub fn rxipv4gdoct(&self) -> RXIPV4GDOCT_R {
        RXIPV4GDOCT_R::new(self.bits)
    }
}
#[doc = "RxIPv4 Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_good_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV4_GOOD_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV4_GOOD_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_good_octets::R`](R) reader structure"]
impl crate::Readable for RXIPV4_GOOD_OCTETS_SPEC {}
#[doc = "`reset()` method sets RXIPV4_GOOD_OCTETS to value 0"]
impl crate::Resettable for RXIPV4_GOOD_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
