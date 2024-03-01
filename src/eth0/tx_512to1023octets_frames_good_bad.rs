#[doc = "Register `TX_512TO1023OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<Tx512to1023octetsFramesGoodBadSpec>;
#[doc = "Field `TX512_1023OCTGB` reader - This field indicates the number of transmitted good and bad frames with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
pub type Tx512_1023octgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx512_1023octgb(&self) -> Tx512_1023octgbR {
        Tx512_1023octgbR::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_512to1023octets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx512to1023octetsFramesGoodBadSpec;
impl crate::RegisterSpec for Tx512to1023octetsFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_512to1023octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for Tx512to1023octetsFramesGoodBadSpec {}
#[doc = "`reset()` method sets TX_512TO1023OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for Tx512to1023octetsFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
