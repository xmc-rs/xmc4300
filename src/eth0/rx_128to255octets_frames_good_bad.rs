#[doc = "Register `RX_128TO255OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<Rx128to255octetsFramesGoodBadSpec>;
#[doc = "Field `RX128_255OCTGB` reader - This field indicates the number of received good and bad frames with length between 128 and 255 (inclusive) bytes, exclusive of preamble."]
pub type Rx128_255octgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 128 and 255 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx128_255octgb(&self) -> Rx128_255octgbR {
        Rx128_255octgbR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad 128 to 255 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_128to255octets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx128to255octetsFramesGoodBadSpec;
impl crate::RegisterSpec for Rx128to255octetsFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_128to255octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for Rx128to255octetsFramesGoodBadSpec {}
#[doc = "`reset()` method sets RX_128TO255OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for Rx128to255octetsFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
