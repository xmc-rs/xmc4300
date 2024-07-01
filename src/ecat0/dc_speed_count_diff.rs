#[doc = "Register `DC_SPEED_COUNT_DIFF` reader"]
pub type R = crate::R<DC_SPEED_COUNT_DIFF_SPEC>;
#[doc = "Field `DEVIATION` reader - Representation of the deviation between local clock period and Reference Clock's clock period"]
pub type DEVIATION_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Representation of the deviation between local clock period and Reference Clock's clock period"]
    #[inline(always)]
    pub fn deviation(&self) -> DEVIATION_R {
        DEVIATION_R::new(self.bits)
    }
}
#[doc = "Speed Counter Diff\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_speed_count_diff::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_SPEED_COUNT_DIFF_SPEC;
impl crate::RegisterSpec for DC_SPEED_COUNT_DIFF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dc_speed_count_diff::R`](R) reader structure"]
impl crate::Readable for DC_SPEED_COUNT_DIFF_SPEC {}
#[doc = "`reset()` method sets DC_SPEED_COUNT_DIFF to value 0"]
impl crate::Resettable for DC_SPEED_COUNT_DIFF_SPEC {
    const RESET_VALUE: u16 = 0;
}
