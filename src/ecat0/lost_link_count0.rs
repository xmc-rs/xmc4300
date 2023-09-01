#[doc = "Register `LOST_LINK_COUNT0` reader"]
pub type R = crate::R<LOST_LINK_COUNT0_SPEC>;
#[doc = "Field `LL_COUNTER` reader - Lost Link counter of Port p"]
pub type LL_COUNTER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Lost Link counter of Port p"]
    #[inline(always)]
    pub fn ll_counter(&self) -> LL_COUNTER_R {
        LL_COUNTER_R::new(self.bits)
    }
}
#[doc = "Lost Link Counter Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lost_link_count0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOST_LINK_COUNT0_SPEC;
impl crate::RegisterSpec for LOST_LINK_COUNT0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lost_link_count0::R`](R) reader structure"]
impl crate::Readable for LOST_LINK_COUNT0_SPEC {}
#[doc = "`reset()` method sets LOST_LINK_COUNT0 to value 0"]
impl crate::Resettable for LOST_LINK_COUNT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
