#[doc = "Register `PHYSICAL_RW_OFFSET` reader"]
pub type R = crate::R<PhysicalRwOffsetSpec>;
#[doc = "Field `OFFSET` reader - Offset of R/W Commands (FPRW, APRW) between Read address and Write address"]
pub type OffsetR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Offset of R/W Commands (FPRW, APRW) between Read address and Write address"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(self.bits)
    }
}
#[doc = "Physical Read/Write Offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_rw_offset::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhysicalRwOffsetSpec;
impl crate::RegisterSpec for PhysicalRwOffsetSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`physical_rw_offset::R`](R) reader structure"]
impl crate::Readable for PhysicalRwOffsetSpec {}
#[doc = "`reset()` method sets PHYSICAL_RW_OFFSET to value 0"]
impl crate::Resettable for PhysicalRwOffsetSpec {
    const RESET_VALUE: u16 = 0;
}
