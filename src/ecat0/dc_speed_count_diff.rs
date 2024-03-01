#[doc = "Register `DC_SPEED_COUNT_DIFF` reader"]
pub type R = crate::R<DcSpeedCountDiffSpec>;
#[doc = "Field `DEVIATION` reader - Representation of the deviation between local clock period and Reference Clock's clock period"]
pub type DeviationR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Representation of the deviation between local clock period and Reference Clock's clock period"]
    #[inline(always)]
    pub fn deviation(&self) -> DeviationR {
        DeviationR::new(self.bits)
    }
}
#[doc = "Speed Counter Diff\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_speed_count_diff::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcSpeedCountDiffSpec;
impl crate::RegisterSpec for DcSpeedCountDiffSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dc_speed_count_diff::R`](R) reader structure"]
impl crate::Readable for DcSpeedCountDiffSpec {}
#[doc = "`reset()` method sets DC_SPEED_COUNT_DIFF to value 0"]
impl crate::Resettable for DcSpeedCountDiffSpec {
    const RESET_VALUE: u16 = 0;
}
