#[doc = "Register `REVISION` reader"]
pub type R = crate::R<RevisionSpec>;
#[doc = "Field `Revision` reader - Revision of EtherCAT controller"]
pub type RevisionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Revision of EtherCAT controller"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(self.bits)
    }
}
#[doc = "Revision of EtherCAT Controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevisionSpec;
impl crate::RegisterSpec for RevisionSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`revision::R`](R) reader structure"]
impl crate::Readable for RevisionSpec {}
#[doc = "`reset()` method sets REVISION to value 0x01"]
impl crate::Resettable for RevisionSpec {
    const RESET_VALUE: u8 = 0x01;
}
