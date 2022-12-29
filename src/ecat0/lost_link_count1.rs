#[doc = "Register `LOST_LINK_COUNT1` reader"]
pub struct R(crate::R<LOST_LINK_COUNT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOST_LINK_COUNT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOST_LINK_COUNT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOST_LINK_COUNT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LL_COUNTER` reader - Lost Link counter of Port p"]
pub type LL_COUNTER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Lost Link counter of Port p"]
    #[inline(always)]
    pub fn ll_counter(&self) -> LL_COUNTER_R {
        LL_COUNTER_R::new(self.bits)
    }
}
#[doc = "Lost Link Counter Port 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lost_link_count1](index.html) module"]
pub struct LOST_LINK_COUNT1_SPEC;
impl crate::RegisterSpec for LOST_LINK_COUNT1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lost_link_count1::R](R) reader structure"]
impl crate::Readable for LOST_LINK_COUNT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOST_LINK_COUNT1 to value 0"]
impl crate::Resettable for LOST_LINK_COUNT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
