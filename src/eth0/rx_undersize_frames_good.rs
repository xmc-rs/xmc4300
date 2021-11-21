#[doc = "Register `RX_UNDERSIZE_FRAMES_GOOD` reader"]
pub struct R(crate::R<RX_UNDERSIZE_FRAMES_GOOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_UNDERSIZE_FRAMES_GOOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_UNDERSIZE_FRAMES_GOOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_UNDERSIZE_FRAMES_GOOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXUNDERSZG` reader - This field indicates the number of frames received with length less than 64 bytes and without errors."]
pub struct RXUNDERSZG_R(crate::FieldReader<u32, u32>);
impl RXUNDERSZG_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXUNDERSZG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUNDERSZG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with length less than 64 bytes and without errors."]
    #[inline(always)]
    pub fn rxunderszg(&self) -> RXUNDERSZG_R {
        RXUNDERSZG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive Frame Count for Undersize Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_undersize_frames_good](index.html) module"]
pub struct RX_UNDERSIZE_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for RX_UNDERSIZE_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_undersize_frames_good::R](R) reader structure"]
impl crate::Readable for RX_UNDERSIZE_FRAMES_GOOD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_UNDERSIZE_FRAMES_GOOD to value 0"]
impl crate::Resettable for RX_UNDERSIZE_FRAMES_GOOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
