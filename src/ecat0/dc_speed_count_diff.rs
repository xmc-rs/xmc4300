#[doc = "Register `DC_SPEED_COUNT_DIFF` reader"]
pub struct R(crate::R<DC_SPEED_COUNT_DIFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_SPEED_COUNT_DIFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_SPEED_COUNT_DIFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_SPEED_COUNT_DIFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVIATION` reader - Representation of the deviation between local clock period and Reference Clock's clock period"]
pub type DEVIATION_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Representation of the deviation between local clock period and Reference Clock's clock period"]
    #[inline(always)]
    pub fn deviation(&self) -> DEVIATION_R {
        DEVIATION_R::new(self.bits)
    }
}
#[doc = "Speed Counter Diff\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_speed_count_diff](index.html) module"]
pub struct DC_SPEED_COUNT_DIFF_SPEC;
impl crate::RegisterSpec for DC_SPEED_COUNT_DIFF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dc_speed_count_diff::R](R) reader structure"]
impl crate::Readable for DC_SPEED_COUNT_DIFF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_SPEED_COUNT_DIFF to value 0"]
impl crate::Resettable for DC_SPEED_COUNT_DIFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
