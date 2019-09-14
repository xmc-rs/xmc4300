#[doc = "Reader of register FMMU_L_STOP_BIT"]
pub type R = crate::R<u8, super::FMMU_L_STOP_BIT>;
#[doc = "Reader of field `L_STOP_BIT`"]
pub type L_STOP_BIT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Last logical bit that shall be mapped"]
    #[inline(always)]
    pub fn l_stop_bit(&self) -> L_STOP_BIT_R {
        L_STOP_BIT_R::new((self.bits & 0x07) as u8)
    }
}
