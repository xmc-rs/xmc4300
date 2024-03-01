#[doc = "Register `RXICMP_GOOD_OCTETS` reader"]
pub type R = crate::R<RxicmpGoodOctetsSpec>;
#[doc = "Field `RXICMPGDOCT` reader - This field indicates the number of bytes received in a good ICMP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type RxicmpgdoctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a good ICMP segment. This counter does not count the IP Header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxicmpgdoct(&self) -> RxicmpgdoctR {
        RxicmpgdoctR::new(self.bits)
    }
}
#[doc = "Receive ICMP Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_good_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxicmpGoodOctetsSpec;
impl crate::RegisterSpec for RxicmpGoodOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxicmp_good_octets::R`](R) reader structure"]
impl crate::Readable for RxicmpGoodOctetsSpec {}
#[doc = "`reset()` method sets RXICMP_GOOD_OCTETS to value 0"]
impl crate::Resettable for RxicmpGoodOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
