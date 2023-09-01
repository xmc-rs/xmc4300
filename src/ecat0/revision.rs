#[doc = "Register `REVISION` reader"]
pub type R = crate::R<REVISION_SPEC>;
#[doc = "Field `Revision` reader - Revision of EtherCAT controller"]
pub type REVISION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Revision of EtherCAT controller"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(self.bits)
    }
}
#[doc = "Revision of EtherCAT Controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REVISION_SPEC;
impl crate::RegisterSpec for REVISION_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`revision::R`](R) reader structure"]
impl crate::Readable for REVISION_SPEC {}
#[doc = "`reset()` method sets REVISION to value 0x01"]
impl crate::Resettable for REVISION_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
