#[doc = "Register `RX_OVERSIZE_FRAMES_GOOD` reader"]
pub struct R(crate::R<RX_OVERSIZE_FRAMES_GOOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_OVERSIZE_FRAMES_GOOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_OVERSIZE_FRAMES_GOOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_OVERSIZE_FRAMES_GOOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXOVERSZG` reader - This field indicates the number of frames received without errors, with length greater than the maxsize (1,518 or 1,522 for VLAN tagged frames; 2,000 bytes if enabled by setting MAC Configuration.2KPE)."]
pub struct RXOVERSZG_R(crate::FieldReader<u32, u32>);
impl RXOVERSZG_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXOVERSZG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERSZG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received without errors, with length greater than the maxsize (1,518 or 1,522 for VLAN tagged frames; 2,000 bytes if enabled by setting MAC Configuration.2KPE)."]
    #[inline(always)]
    pub fn rxoverszg(&self) -> RXOVERSZG_R {
        RXOVERSZG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Rx Oversize Frames Good Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_oversize_frames_good](index.html) module"]
pub struct RX_OVERSIZE_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for RX_OVERSIZE_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_oversize_frames_good::R](R) reader structure"]
impl crate::Readable for RX_OVERSIZE_FRAMES_GOOD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_OVERSIZE_FRAMES_GOOD to value 0"]
impl crate::Resettable for RX_OVERSIZE_FRAMES_GOOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
