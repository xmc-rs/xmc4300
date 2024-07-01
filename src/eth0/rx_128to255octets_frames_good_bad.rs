#[doc = "Register `RX_128TO255OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<RX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Field `RX128_255OCTGB` reader - This field indicates the number of received good and bad frames with length between 128 and 255 (inclusive) bytes, exclusive of preamble."]
pub type RX128_255OCTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 128 and 255 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx128_255octgb(&self) -> RX128_255OCTGB_R {
        RX128_255OCTGB_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad 128 to 255 Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_128to255octets_frames_good_bad::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_128to255octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for RX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets RX_128TO255OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for RX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
