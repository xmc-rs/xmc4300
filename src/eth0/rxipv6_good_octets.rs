#[doc = "Register `RXIPV6_GOOD_OCTETS` reader"]
pub type R = crate::R<Rxipv6GoodOctetsSpec>;
#[doc = "Field `RXIPV6GDOCT` reader - Thsi field indicates the number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data."]
pub type Rxipv6gdoctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Thsi field indicates the number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data."]
    #[inline(always)]
    pub fn rxipv6gdoct(&self) -> Rxipv6gdoctR {
        Rxipv6gdoctR::new(self.bits)
    }
}
#[doc = "RxIPv6 Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_good_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv6GoodOctetsSpec;
impl crate::RegisterSpec for Rxipv6GoodOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_good_octets::R`](R) reader structure"]
impl crate::Readable for Rxipv6GoodOctetsSpec {}
#[doc = "`reset()` method sets RXIPV6_GOOD_OCTETS to value 0"]
impl crate::Resettable for Rxipv6GoodOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
