#[doc = "Register `TX_OSIZE_FRAMES_GOOD` reader"]
pub struct R(crate::R<TX_OSIZE_FRAMES_GOOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_OSIZE_FRAMES_GOOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_OSIZE_FRAMES_GOOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_OSIZE_FRAMES_GOOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXOSIZG` reader - This field indicates the number of frames transmitted without errors and with length greater than the maxsize (1,518 or 1,522 bytes for VLAN tagged frames; 2000 bytes if enabled by setting MAC Configuration.2KPE)."]
pub struct TXOSIZG_R(crate::FieldReader<u32, u32>);
impl TXOSIZG_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXOSIZG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOSIZG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames transmitted without errors and with length greater than the maxsize (1,518 or 1,522 bytes for VLAN tagged frames; 2000 bytes if enabled by setting MAC Configuration.2KPE)."]
    #[inline(always)]
    pub fn txosizg(&self) -> TXOSIZG_R {
        TXOSIZG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Transmit Frame Count for Good Oversize Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_osize_frames_good](index.html) module"]
pub struct TX_OSIZE_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for TX_OSIZE_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_osize_frames_good::R](R) reader structure"]
impl crate::Readable for TX_OSIZE_FRAMES_GOOD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_OSIZE_FRAMES_GOOD to value 0"]
impl crate::Resettable for TX_OSIZE_FRAMES_GOOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
