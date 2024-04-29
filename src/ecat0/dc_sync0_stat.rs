#[doc = "Register `DC_SYNC0_STAT` reader"]
pub type R = crate::R<DC_SYNC0_STAT_SPEC>;
#[doc = "Field `S0_STATE` reader - SYNC0 state for Acknowledge mode"]
pub type S0_STATE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SYNC0 state for Acknowledge mode"]
    #[inline(always)]
    pub fn s0_state(&self) -> S0_STATE_R {
        S0_STATE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYNC0 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sync0_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_SYNC0_STAT_SPEC;
impl crate::RegisterSpec for DC_SYNC0_STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_sync0_stat::R`](R) reader structure"]
impl crate::Readable for DC_SYNC0_STAT_SPEC {}
#[doc = "`reset()` method sets DC_SYNC0_STAT to value 0"]
impl crate::Resettable for DC_SYNC0_STAT_SPEC {
    const RESET_VALUE: u8 = 0;
}
