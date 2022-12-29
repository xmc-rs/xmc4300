#[doc = "Register `DC_LATCH0_TIME_NEG[%s]` reader"]
pub struct R(crate::R<DC_LATCH0_TIME_NEG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_LATCH0_TIME_NEG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_LATCH0_TIME_NEG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_LATCH0_TIME_NEG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC_LATCH0_TIME_NEG` reader - Captures System time at the negative edge of the Latch0 signal"]
pub type DC_LATCH0_TIME_NEG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Captures System time at the negative edge of the Latch0 signal"]
    #[inline(always)]
    pub fn dc_latch0_time_neg(&self) -> DC_LATCH0_TIME_NEG_R {
        DC_LATCH0_TIME_NEG_R::new(self.bits)
    }
}
#[doc = "Register captures System time at the negative edge of the Latch0 signal\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch0_time_neg](index.html) module"]
pub struct DC_LATCH0_TIME_NEG_SPEC;
impl crate::RegisterSpec for DC_LATCH0_TIME_NEG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_latch0_time_neg::R](R) reader structure"]
impl crate::Readable for DC_LATCH0_TIME_NEG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_LATCH0_TIME_NEG[%s]
to value 0"]
impl crate::Resettable for DC_LATCH0_TIME_NEG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
