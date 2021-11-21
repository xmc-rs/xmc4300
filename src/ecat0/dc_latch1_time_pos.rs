#[doc = "Register `DC_LATCH1_TIME_POS[%s]` reader"]
pub struct R(crate::R<DC_LATCH1_TIME_POS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_LATCH1_TIME_POS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_LATCH1_TIME_POS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_LATCH1_TIME_POS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC_LATCH1_TIME_POS` reader - Captures System time at the positive edge of the Latch1 signal"]
pub struct DC_LATCH1_TIME_POS_R(crate::FieldReader<u32, u32>);
impl DC_LATCH1_TIME_POS_R {
    pub(crate) fn new(bits: u32) -> Self {
        DC_LATCH1_TIME_POS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_LATCH1_TIME_POS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Captures System time at the positive edge of the Latch1 signal"]
    #[inline(always)]
    pub fn dc_latch1_time_pos(&self) -> DC_LATCH1_TIME_POS_R {
        DC_LATCH1_TIME_POS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Register captures System time at the positive edge of the Latch1 signal\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch1_time_pos](index.html) module"]
pub struct DC_LATCH1_TIME_POS_SPEC;
impl crate::RegisterSpec for DC_LATCH1_TIME_POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_latch1_time_pos::R](R) reader structure"]
impl crate::Readable for DC_LATCH1_TIME_POS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_LATCH1_TIME_POS[%s]
to value 0"]
impl crate::Resettable for DC_LATCH1_TIME_POS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
