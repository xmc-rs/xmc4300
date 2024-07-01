#[doc = "Register `RX_64OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<RX_64OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Field `RX64OCTGB` reader - This field indicates the number of received good and bad frames with length 64 bytes, exclusive of preamble."]
pub type RX64OCTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length 64 bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx64octgb(&self) -> RX64OCTGB_R {
        RX64OCTGB_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad 64 Byte Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_64octets_frames_good_bad::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_64OCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_64OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_64octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for RX_64OCTETS_FRAMES_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets RX_64OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for RX_64OCTETS_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
