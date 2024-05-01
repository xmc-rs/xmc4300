#[doc = "Register `RXIPV4_GOOD_OCTETS` reader"]
pub type R = crate::R<Rxipv4GoodOctetsSpec>;
#[doc = "Field `RXIPV4GDOCT` reader - This field indicates the number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. The Ethernet header, FCS, pad, or IP pad"]
pub type Rxipv4gdoctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. The Ethernet header, FCS, pad, or IP pad"]
    #[inline(always)]
    pub fn rxipv4gdoct(&self) -> Rxipv4gdoctR {
        Rxipv4gdoctR::new(self.bits)
    }
}
#[doc = "RxIPv4 Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_good_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv4GoodOctetsSpec;
impl crate::RegisterSpec for Rxipv4GoodOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_good_octets::R`](R) reader structure"]
impl crate::Readable for Rxipv4GoodOctetsSpec {}
#[doc = "`reset()` method sets RXIPV4_GOOD_OCTETS to value 0"]
impl crate::Resettable for Rxipv4GoodOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
