#[doc = "Register `REVISION` reader"]
pub struct R(crate::R<REVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Revision` reader - Revision of EtherCAT controller"]
pub type REVISION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Revision of EtherCAT controller"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(self.bits)
    }
}
#[doc = "Revision of EtherCAT Controller\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](index.html) module"]
pub struct REVISION_SPEC;
impl crate::RegisterSpec for REVISION_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [revision::R](R) reader structure"]
impl crate::Readable for REVISION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REVISION to value 0x01"]
impl crate::Resettable for REVISION_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
