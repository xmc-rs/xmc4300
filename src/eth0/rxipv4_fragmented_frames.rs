#[doc = "Register `RXIPV4_FRAGMENTED_FRAMES` reader"]
pub struct R(crate::R<RXIPV4_FRAGMENTED_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIPV4_FRAGMENTED_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIPV4_FRAGMENTED_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIPV4_FRAGMENTED_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXIPV4FRAGFRM` reader - This field indicates the number of good IPv4 datagrams received with fragmentation."]
pub struct RXIPV4FRAGFRM_R(crate::FieldReader<u32, u32>);
impl RXIPV4FRAGFRM_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXIPV4FRAGFRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4FRAGFRM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IPv4 datagrams received with fragmentation."]
    #[inline(always)]
    pub fn rxipv4fragfrm(&self) -> RXIPV4FRAGFRM_R {
        RXIPV4FRAGFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive IPV4 Fragmented Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxipv4_fragmented_frames](index.html) module"]
pub struct RXIPV4_FRAGMENTED_FRAMES_SPEC;
impl crate::RegisterSpec for RXIPV4_FRAGMENTED_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxipv4_fragmented_frames::R](R) reader structure"]
impl crate::Readable for RXIPV4_FRAGMENTED_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXIPV4_FRAGMENTED_FRAMES to value 0"]
impl crate::Resettable for RXIPV4_FRAGMENTED_FRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
