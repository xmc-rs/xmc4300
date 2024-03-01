#[doc = "Register `RX_OCTET_COUNT_GOOD_BAD` reader"]
pub type R = crate::R<RxOctetCountGoodBadSpec>;
#[doc = "Field `RXOCTGB` reader - This field indicates the number of bytes received, exclusive of preamble, in good and bad frames."]
pub type RxoctgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received, exclusive of preamble, in good and bad frames."]
    #[inline(always)]
    pub fn rxoctgb(&self) -> RxoctgbR {
        RxoctgbR::new(self.bits)
    }
}
#[doc = "Receive Octet Count for Good and Bad Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_octet_count_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOctetCountGoodBadSpec;
impl crate::RegisterSpec for RxOctetCountGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_octet_count_good_bad::R`](R) reader structure"]
impl crate::Readable for RxOctetCountGoodBadSpec {}
#[doc = "`reset()` method sets RX_OCTET_COUNT_GOOD_BAD to value 0"]
impl crate::Resettable for RxOctetCountGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
