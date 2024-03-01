#[doc = "Register `RX_512TO1023OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<Rx512to1023octetsFramesGoodBadSpec>;
#[doc = "Field `RX512_1023OCTGB` reader - This field indicates the number of received good and bad frames with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble."]
pub type Rx512_1023octgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx512_1023octgb(&self) -> Rx512_1023octgbR {
        Rx512_1023octgbR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_512to1023octets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx512to1023octetsFramesGoodBadSpec;
impl crate::RegisterSpec for Rx512to1023octetsFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_512to1023octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for Rx512to1023octetsFramesGoodBadSpec {}
#[doc = "`reset()` method sets RX_512TO1023OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for Rx512to1023octetsFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
