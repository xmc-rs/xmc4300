#[doc = "Register `TX_64OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<Tx64octetsFramesGoodBadSpec>;
#[doc = "Field `TX64OCTGB` reader - This field indicates the number of transmitted good and bad frames with length of 64 bytes, exclusive of preamble and retried frames."]
pub type Tx64octgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length of 64 bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx64octgb(&self) -> Tx64octgbR {
        Tx64octgbR::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 64 Byte Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_64octets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx64octetsFramesGoodBadSpec;
impl crate::RegisterSpec for Tx64octetsFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_64octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for Tx64octetsFramesGoodBadSpec {}
#[doc = "`reset()` method sets TX_64OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for Tx64octetsFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
