#[doc = "Reader of register PHYSICAL_RW_OFFSET"]
pub type R = crate::R<u16, super::PHYSICAL_RW_OFFSET>;
#[doc = "Reader of field `OFFSET`"]
pub type OFFSET_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Offset of R/W Commands (FPRW, APRW) between Read address and Write address"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
