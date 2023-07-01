#[doc = "Register `DC_SYNC1_STAT` reader"]
pub struct R(crate::R<DC_SYNC1_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_SYNC1_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_SYNC1_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_SYNC1_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `S1_STATE` reader - SYNC1 state for Acknowledge mode"]
pub type S1_STATE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SYNC1 state for Acknowledge mode"]
    #[inline(always)]
    pub fn s1_state(&self) -> S1_STATE_R {
        S1_STATE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYNC1 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sync1_stat](index.html) module"]
pub struct DC_SYNC1_STAT_SPEC;
impl crate::RegisterSpec for DC_SYNC1_STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dc_sync1_stat::R](R) reader structure"]
impl crate::Readable for DC_SYNC1_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_SYNC1_STAT to value 0"]
impl crate::Resettable for DC_SYNC1_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
