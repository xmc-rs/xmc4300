#[doc = "Register `PHYSICAL_RW_OFFSET` reader"]
pub struct R(crate::R<PHYSICAL_RW_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHYSICAL_RW_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHYSICAL_RW_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHYSICAL_RW_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFFSET` reader - Offset of R/W Commands (FPRW, APRW) between Read address and Write address"]
pub type OFFSET_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Offset of R/W Commands (FPRW, APRW) between Read address and Write address"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits)
    }
}
#[doc = "Physical Read/Write Offset\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [physical_rw_offset](index.html) module"]
pub struct PHYSICAL_RW_OFFSET_SPEC;
impl crate::RegisterSpec for PHYSICAL_RW_OFFSET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [physical_rw_offset::R](R) reader structure"]
impl crate::Readable for PHYSICAL_RW_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PHYSICAL_RW_OFFSET to value 0"]
impl crate::Resettable for PHYSICAL_RW_OFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
