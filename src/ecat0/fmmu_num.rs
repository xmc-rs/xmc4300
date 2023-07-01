#[doc = "Register `FMMU_NUM` reader"]
pub struct R(crate::R<FMMU_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMMU_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMMU_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMMU_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_FMMU` reader - Number of supported FMMU channels (or entities) of the EtherCAT Slave Controller"]
pub type NUM_FMMU_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of supported FMMU channels (or entities) of the EtherCAT Slave Controller"]
    #[inline(always)]
    pub fn num_fmmu(&self) -> NUM_FMMU_R {
        NUM_FMMU_R::new(self.bits)
    }
}
#[doc = "FMMUs Supported\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmmu_num](index.html) module"]
pub struct FMMU_NUM_SPEC;
impl crate::RegisterSpec for FMMU_NUM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fmmu_num::R](R) reader structure"]
impl crate::Readable for FMMU_NUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMMU_NUM to value 0x08"]
impl crate::Resettable for FMMU_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
