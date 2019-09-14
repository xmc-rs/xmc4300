#[doc = "Reader of register CLKCALCONST"]
pub type R = crate::R<u32, super::CLKCALCONST>;
#[doc = "Reader of field `CALIBCONST`"]
pub type CALIBCONST_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    pub fn calibconst(&self) -> CALIBCONST_R {
        CALIBCONST_R::new((self.bits & 0x0f) as u8)
    }
}
