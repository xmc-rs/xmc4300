#[doc = "Register `DC_LATCH1_TIME_POS[%s]` reader"]
pub type R = crate::R<DC_LATCH1_TIME_POS_SPEC>;
#[doc = "Field `DC_LATCH1_TIME_POS` reader - Captures System time at the positive edge of the Latch1 signal"]
pub type DC_LATCH1_TIME_POS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Captures System time at the positive edge of the Latch1 signal"]
    #[inline(always)]
    pub fn dc_latch1_time_pos(&self) -> DC_LATCH1_TIME_POS_R {
        DC_LATCH1_TIME_POS_R::new(self.bits)
    }
}
#[doc = "Register captures System time at the positive edge of the Latch1 signal\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_latch1_time_pos::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_LATCH1_TIME_POS_SPEC;
impl crate::RegisterSpec for DC_LATCH1_TIME_POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_latch1_time_pos::R`](R) reader structure"]
impl crate::Readable for DC_LATCH1_TIME_POS_SPEC {}
#[doc = "`reset()` method sets DC_LATCH1_TIME_POS[%s]
to value 0"]
impl crate::Resettable for DC_LATCH1_TIME_POS_SPEC {
    const RESET_VALUE: u32 = 0;
}
