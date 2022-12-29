#[doc = "Register `FMMU_L_START_BIT` reader"]
pub struct R(crate::R<FMMU_L_START_BIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMMU_L_START_BIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMMU_L_START_BIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMMU_L_START_BIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L_START_BIT` reader - Logical starting bit that shall be mapped"]
pub type L_START_BIT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Logical starting bit that shall be mapped"]
    #[inline(always)]
    pub fn l_start_bit(&self) -> L_START_BIT_R {
        L_START_BIT_R::new(self.bits & 7)
    }
}
#[doc = "Start bit FMMU 0 in logical address space\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmmu_l_start_bit](index.html) module"]
pub struct FMMU_L_START_BIT_SPEC;
impl crate::RegisterSpec for FMMU_L_START_BIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fmmu_l_start_bit::R](R) reader structure"]
impl crate::Readable for FMMU_L_START_BIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMMU_L_START_BIT to value 0"]
impl crate::Resettable for FMMU_L_START_BIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
