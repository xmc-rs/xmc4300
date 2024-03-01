#[doc = "Register `RXUDP_GOOD_OCTETS` reader"]
pub type R = crate::R<RxudpGoodOctetsSpec>;
#[doc = "Field `RXUDPGDOCT` reader - This field indicates the number of bytes received in a good UDP segment. This counter does not count IP header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
pub type RxudpgdoctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in a good UDP segment. This counter does not count IP header bytes. The Ethernet header, FCS, pad, or IP pad bytes are not included in this counter."]
    #[inline(always)]
    pub fn rxudpgdoct(&self) -> RxudpgdoctR {
        RxudpgdoctR::new(self.bits)
    }
}
#[doc = "Receive UDP Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_good_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxudpGoodOctetsSpec;
impl crate::RegisterSpec for RxudpGoodOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxudp_good_octets::R`](R) reader structure"]
impl crate::Readable for RxudpGoodOctetsSpec {}
#[doc = "`reset()` method sets RXUDP_GOOD_OCTETS to value 0"]
impl crate::Resettable for RxudpGoodOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
