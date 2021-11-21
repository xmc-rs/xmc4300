#[doc = "Register `RXTCP_ERROR_FRAMES` reader"]
pub struct R(crate::R<RXTCP_ERROR_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXTCP_ERROR_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXTCP_ERROR_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXTCP_ERROR_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXTCPERRFRM` reader - This field indicates the number of good IP datagrams whose TCP payload has a checksum error."]
pub struct RXTCPERRFRM_R(crate::FieldReader<u32, u32>);
impl RXTCPERRFRM_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXTCPERRFRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTCPERRFRM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams whose TCP payload has a checksum error."]
    #[inline(always)]
    pub fn rxtcperrfrm(&self) -> RXTCPERRFRM_R {
        RXTCPERRFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RxTCP Error Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtcp_error_frames](index.html) module"]
pub struct RXTCP_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RXTCP_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxtcp_error_frames::R](R) reader structure"]
impl crate::Readable for RXTCP_ERROR_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXTCP_ERROR_FRAMES to value 0"]
impl crate::Resettable for RXTCP_ERROR_FRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
