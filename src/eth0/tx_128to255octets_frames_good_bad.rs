#[doc = "Register `TX_128TO255OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<Tx128to255octetsFramesGoodBadSpec>;
#[doc = "Field `TX128_255OCTGB` reader - This field indicates the number of transmitted good and bad frames with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames."]
pub type Tx128_255octgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx128_255octgb(&self) -> Tx128_255octgbR {
        Tx128_255octgbR::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_128to255octets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx128to255octetsFramesGoodBadSpec;
impl crate::RegisterSpec for Tx128to255octetsFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_128to255octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for Tx128to255octetsFramesGoodBadSpec {}
#[doc = "`reset()` method sets TX_128TO255OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for Tx128to255octetsFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
