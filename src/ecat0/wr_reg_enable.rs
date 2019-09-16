#[doc = "Reader of register WR_REG_ENABLE"]
pub type R = crate::R<u8, super::WR_REG_ENABLE>;
#[doc = "Reader of field `WR_REG_EN`"]
pub type WR_REG_EN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Write register protection enabled"]
    #[inline(always)]
    pub fn wr_reg_en(&self) -> WR_REG_EN_R {
        WR_REG_EN_R::new((self.bits & 0x01) != 0)
    }
}
