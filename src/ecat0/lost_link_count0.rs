#[doc = "Register `LOST_LINK_COUNT0` reader"]
pub type R = crate::R<LostLinkCount0Spec>;
#[doc = "Field `LL_COUNTER` reader - Lost Link counter of Port p"]
pub type LlCounterR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Lost Link counter of Port p"]
    #[inline(always)]
    pub fn ll_counter(&self) -> LlCounterR {
        LlCounterR::new(self.bits)
    }
}
#[doc = "Lost Link Counter Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lost_link_count0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LostLinkCount0Spec;
impl crate::RegisterSpec for LostLinkCount0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lost_link_count0::R`](R) reader structure"]
impl crate::Readable for LostLinkCount0Spec {}
#[doc = "`reset()` method sets LOST_LINK_COUNT0 to value 0"]
impl crate::Resettable for LostLinkCount0Spec {
    const RESET_VALUE: u8 = 0;
}
