#[doc = "Register `CLKCALCONST` reader"]
pub type R = crate::R<CLKCALCONST_SPEC>;
#[doc = "Field `CALIBCONST` reader - Clock Calibration Constant Value"]
pub type CALIBCONST_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    pub fn calibconst(&self) -> CALIBCONST_R {
        CALIBCONST_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Clock Calibration Constant Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcalconst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCALCONST_SPEC;
impl crate::RegisterSpec for CLKCALCONST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcalconst::R`](R) reader structure"]
impl crate::Readable for CLKCALCONST_SPEC {}
#[doc = "`reset()` method sets CLKCALCONST to value 0"]
impl crate::Resettable for CLKCALCONST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
