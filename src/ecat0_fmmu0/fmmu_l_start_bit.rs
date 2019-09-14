#[doc = "Reader of register FMMU_L_START_BIT"]
pub type R = crate::R<u8, super::FMMU_L_START_BIT>;
#[doc = "Reader of field `L_START_BIT`"]
pub type L_START_BIT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Logical starting bit that shall be mapped"]
    #[inline(always)]
    pub fn l_start_bit(&self) -> L_START_BIT_R {
        L_START_BIT_R::new((self.bits & 0x07) as u8)
    }
}
