#[doc = "Register `DC_LATCH0_TIME_POS[%s]` reader"]
pub struct R(crate::R<DC_LATCH0_TIME_POS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_LATCH0_TIME_POS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_LATCH0_TIME_POS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_LATCH0_TIME_POS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC_LATCH0_TIME_POS` reader - Captures System time at the positive edge of the Latch0 signal"]
pub type DC_LATCH0_TIME_POS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Captures System time at the positive edge of the Latch0 signal"]
    #[inline(always)]
    pub fn dc_latch0_time_pos(&self) -> DC_LATCH0_TIME_POS_R {
        DC_LATCH0_TIME_POS_R::new(self.bits)
    }
}
#[doc = "Register captures System time at the positive edge of the Latch0 signal\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch0_time_pos](index.html) module"]
pub struct DC_LATCH0_TIME_POS_SPEC;
impl crate::RegisterSpec for DC_LATCH0_TIME_POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_latch0_time_pos::R](R) reader structure"]
impl crate::Readable for DC_LATCH0_TIME_POS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_LATCH0_TIME_POS[%s]
to value 0"]
impl crate::Resettable for DC_LATCH0_TIME_POS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
