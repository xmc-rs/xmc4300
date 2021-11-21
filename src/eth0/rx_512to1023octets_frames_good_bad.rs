#[doc = "Register `RX_512TO1023OCTETS_FRAMES_GOOD_BAD` reader"]
pub struct R(crate::R<RX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX512_1023OCTGB` reader - This field indicates the number of received good and bad frames with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble."]
pub struct RX512_1023OCTGB_R(crate::FieldReader<u32, u32>);
impl RX512_1023OCTGB_R {
    pub(crate) fn new(bits: u32) -> Self {
        RX512_1023OCTGB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX512_1023OCTGB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx512_1023octgb(&self) -> RX512_1023OCTGB_R {
        RX512_1023OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_512to1023octets_frames_good_bad](index.html) module"]
pub struct RX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_512to1023octets_frames_good_bad::R](R) reader structure"]
impl crate::Readable for RX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_512TO1023OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for RX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
