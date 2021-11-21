#[doc = "Register `RXICMP_GOOD_FRAMES` reader"]
pub struct R(crate::R<RXICMP_GOOD_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXICMP_GOOD_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXICMP_GOOD_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXICMP_GOOD_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXICMPGDFRM` reader - This field indicates the number of good IP datagrams with a good ICMP payload."]
pub struct RXICMPGDFRM_R(crate::FieldReader<u32, u32>);
impl RXICMPGDFRM_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXICMPGDFRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXICMPGDFRM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams with a good ICMP payload."]
    #[inline(always)]
    pub fn rxicmpgdfrm(&self) -> RXICMPGDFRM_R {
        RXICMPGDFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RxICMP Good Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxicmp_good_frames](index.html) module"]
pub struct RXICMP_GOOD_FRAMES_SPEC;
impl crate::RegisterSpec for RXICMP_GOOD_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxicmp_good_frames::R](R) reader structure"]
impl crate::Readable for RXICMP_GOOD_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXICMP_GOOD_FRAMES to value 0"]
impl crate::Resettable for RXICMP_GOOD_FRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
