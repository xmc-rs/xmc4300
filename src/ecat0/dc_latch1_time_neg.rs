#[doc = "Register `DC_LATCH1_TIME_NEG[%s]` reader"]
pub type R = crate::R<DC_LATCH1_TIME_NEG_SPEC>;
#[doc = "Field `DC_LATCH1_TIME_NEG` reader - Captures System time at the negative edge of the Latch1 signal"]
pub type DC_LATCH1_TIME_NEG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Captures System time at the negative edge of the Latch1 signal"]
    #[inline(always)]
    pub fn dc_latch1_time_neg(&self) -> DC_LATCH1_TIME_NEG_R {
        DC_LATCH1_TIME_NEG_R::new(self.bits)
    }
}
#[doc = "Register captures System time at the negative edge of the Latch1 signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch1_time_neg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_LATCH1_TIME_NEG_SPEC;
impl crate::RegisterSpec for DC_LATCH1_TIME_NEG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_latch1_time_neg::R`](R) reader structure"]
impl crate::Readable for DC_LATCH1_TIME_NEG_SPEC {}
#[doc = "`reset()` method sets DC_LATCH1_TIME_NEG[%s]
to value 0"]
impl crate::Resettable for DC_LATCH1_TIME_NEG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
