#[doc = "Register `RX_64OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<Rx64octetsFramesGoodBadSpec>;
#[doc = "Field `RX64OCTGB` reader - This field indicates the number of received good and bad frames with length 64 bytes, exclusive of preamble."]
pub type Rx64octgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length 64 bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx64octgb(&self) -> Rx64octgbR {
        Rx64octgbR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad 64 Byte Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_64octets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx64octetsFramesGoodBadSpec;
impl crate::RegisterSpec for Rx64octetsFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_64octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for Rx64octetsFramesGoodBadSpec {}
#[doc = "`reset()` method sets RX_64OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for Rx64octetsFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
