#[doc = "Reader of register SM_P_START_ADR"]
pub type R = crate::R<u16, super::SM_P_START_ADR>;
#[doc = "Reader of field `FIRST_BYTE`"]
pub type FIRST_BYTE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specifies first byte that will be handled by SyncManager"]
    #[inline(always)]
    pub fn first_byte(&self) -> FIRST_BYTE_R {
        FIRST_BYTE_R::new((self.bits & 0xffff) as u16)
    }
}
