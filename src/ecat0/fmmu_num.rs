#[doc = "Register `FMMU_NUM` reader"]
pub type R = crate::R<FMMU_NUM_SPEC>;
#[doc = "Field `NUM_FMMU` reader - Number of supported FMMU channels (or entities) of the EtherCAT Slave Controller"]
pub type NUM_FMMU_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of supported FMMU channels (or entities) of the EtherCAT Slave Controller"]
    #[inline(always)]
    pub fn num_fmmu(&self) -> NUM_FMMU_R {
        NUM_FMMU_R::new(self.bits)
    }
}
#[doc = "FMMUs Supported\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMMU_NUM_SPEC;
impl crate::RegisterSpec for FMMU_NUM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fmmu_num::R`](R) reader structure"]
impl crate::Readable for FMMU_NUM_SPEC {}
#[doc = "`reset()` method sets FMMU_NUM to value 0x08"]
impl crate::Resettable for FMMU_NUM_SPEC {
    const RESET_VALUE: u8 = 0x08;
}
