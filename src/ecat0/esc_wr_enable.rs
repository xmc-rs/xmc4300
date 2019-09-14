#[doc = "Reader of register ESC_WR_ENABLE"]
pub type R = crate::R<u8, super::ESC_WR_ENABLE>;
#[doc = "Reader of field `ESC_WR_PROT`"]
pub type ESC_WR_PROT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Write protection enabled"]
    #[inline(always)]
    pub fn esc_wr_prot(&self) -> ESC_WR_PROT_R {
        ESC_WR_PROT_R::new((self.bits & 0x01) != 0)
    }
}
