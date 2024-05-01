#[doc = "Register `CLKCALCONST` reader"]
pub type R = crate::R<ClkcalconstSpec>;
#[doc = "Field `CALIBCONST` reader - Clock Calibration Constant Value"]
pub type CalibconstR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    pub fn calibconst(&self) -> CalibconstR {
        CalibconstR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Clock Calibration Constant Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcalconst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkcalconstSpec;
impl crate::RegisterSpec for ClkcalconstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcalconst::R`](R) reader structure"]
impl crate::Readable for ClkcalconstSpec {}
#[doc = "`reset()` method sets CLKCALCONST to value 0"]
impl crate::Resettable for ClkcalconstSpec {
    const RESET_VALUE: u32 = 0;
}
