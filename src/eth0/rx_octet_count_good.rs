#[doc = "Register `RX_OCTET_COUNT_GOOD` reader"]
pub type R = crate::R<RxOctetCountGoodSpec>;
#[doc = "Field `RXOCTG` reader - This field indicates the number of bytes received, exclusive of preamble, only in good frames."]
pub type RxoctgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received, exclusive of preamble, only in good frames."]
    #[inline(always)]
    pub fn rxoctg(&self) -> RxoctgR {
        RxoctgR::new(self.bits)
    }
}
#[doc = "Rx Octet Count Good Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_octet_count_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOctetCountGoodSpec;
impl crate::RegisterSpec for RxOctetCountGoodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_octet_count_good::R`](R) reader structure"]
impl crate::Readable for RxOctetCountGoodSpec {}
#[doc = "`reset()` method sets RX_OCTET_COUNT_GOOD to value 0"]
impl crate::Resettable for RxOctetCountGoodSpec {
    const RESET_VALUE: u32 = 0;
}
