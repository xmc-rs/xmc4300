#[doc = "Register `DC_SYNC1_STAT` reader"]
pub type R = crate::R<DcSync1StatSpec>;
#[doc = "Field `S1_STATE` reader - SYNC1 state for Acknowledge mode"]
pub type S1StateR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SYNC1 state for Acknowledge mode"]
    #[inline(always)]
    pub fn s1_state(&self) -> S1StateR {
        S1StateR::new((self.bits & 1) != 0)
    }
}
#[doc = "SYNC1 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sync1_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcSync1StatSpec;
impl crate::RegisterSpec for DcSync1StatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_sync1_stat::R`](R) reader structure"]
impl crate::Readable for DcSync1StatSpec {}
#[doc = "`reset()` method sets DC_SYNC1_STAT to value 0"]
impl crate::Resettable for DcSync1StatSpec {
    const RESET_VALUE: u8 = 0;
}
