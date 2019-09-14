#[doc = "Reader of register MAX_CURRENT_CAP"]
pub type R = crate::R<u32, super::MAX_CURRENT_CAP>;
#[doc = "Reader of field `MAX_CURRENT_FOR_3_3V`"]
pub type MAX_CURRENT_FOR_3_3V_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline(always)]
    pub fn max_current_for_3_3v(&self) -> MAX_CURRENT_FOR_3_3V_R {
        MAX_CURRENT_FOR_3_3V_R::new((self.bits & 0xff) as u8)
    }
}
