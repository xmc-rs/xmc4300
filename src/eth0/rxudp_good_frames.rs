#[doc = "Register `RXUDP_GOOD_FRAMES` reader"]
pub struct R(crate::R<RXUDP_GOOD_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXUDP_GOOD_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXUDP_GOOD_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXUDP_GOOD_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXUDPGDFRM` reader - This field indicates the number of good IP datagrams with a good UDP payload. This counter is not updated when the counter is incremented."]
pub struct RXUDPGDFRM_R(crate::FieldReader<u32, u32>);
impl RXUDPGDFRM_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXUDPGDFRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUDPGDFRM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams with a good UDP payload. This counter is not updated when the counter is incremented."]
    #[inline(always)]
    pub fn rxudpgdfrm(&self) -> RXUDPGDFRM_R {
        RXUDPGDFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RxUDP Good Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxudp_good_frames](index.html) module"]
pub struct RXUDP_GOOD_FRAMES_SPEC;
impl crate::RegisterSpec for RXUDP_GOOD_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxudp_good_frames::R](R) reader structure"]
impl crate::Readable for RXUDP_GOOD_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXUDP_GOOD_FRAMES to value 0"]
impl crate::Resettable for RXUDP_GOOD_FRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
