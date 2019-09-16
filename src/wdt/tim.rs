#[doc = "Reader of register TIM"]
pub type R = crate::R<u32, super::TIM>;
#[doc = "Reader of field `TIM`"]
pub type TIM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Value"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
