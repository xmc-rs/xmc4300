#[doc = "Reader of register DC_SPEED_COUNT_DIFF"]
pub type R = crate::R<u16, super::DC_SPEED_COUNT_DIFF>;
#[doc = "Reader of field `DEVIATION`"]
pub type DEVIATION_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Representation of the deviation between local clock period and Reference Clock's clock period"]
    #[inline(always)]
    pub fn deviation(&self) -> DEVIATION_R {
        DEVIATION_R::new((self.bits & 0xffff) as u16)
    }
}
