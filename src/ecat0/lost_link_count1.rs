#[doc = "Reader of register LOST_LINK_COUNT1"]
pub type R = crate::R<u8, super::LOST_LINK_COUNT1>;
#[doc = "Reader of field `LL_COUNTER`"]
pub type LL_COUNTER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Lost Link counter of Port p"]
    #[inline(always)]
    pub fn ll_counter(&self) -> LL_COUNTER_R {
        LL_COUNTER_R::new((self.bits & 0xff) as u8)
    }
}
