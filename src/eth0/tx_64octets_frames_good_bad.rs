#[doc = "Register `TX_64OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<TX_64OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Field `TX64OCTGB` reader - This field indicates the number of transmitted good and bad frames with length of 64 bytes, exclusive of preamble and retried frames."]
pub type TX64OCTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length of 64 bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx64octgb(&self) -> TX64OCTGB_R {
        TX64OCTGB_R::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 64 Byte Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_64octets_frames_good_bad::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_64OCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_64OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_64octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for TX_64OCTETS_FRAMES_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets TX_64OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for TX_64OCTETS_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
