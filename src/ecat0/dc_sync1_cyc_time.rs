#[doc = "Register `DC_SYNC1_CYC_TIME` reader"]
pub type R = crate::R<DcSync1CycTimeSpec>;
#[doc = "Register `DC_SYNC1_CYC_TIME` writer"]
pub type W = crate::W<DcSync1CycTimeSpec>;
#[doc = "Field `TIME_SYNC1_SYNC0` reader - Time between SYNC1 pulses and SYNC0 pulse"]
pub type TimeSync1Sync0R = crate::FieldReader<u32>;
#[doc = "Field `TIME_SYNC1_SYNC0` writer - Time between SYNC1 pulses and SYNC0 pulse"]
pub type TimeSync1Sync0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Time between SYNC1 pulses and SYNC0 pulse"]
    #[inline(always)]
    pub fn time_sync1_sync0(&self) -> TimeSync1Sync0R {
        TimeSync1Sync0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time between SYNC1 pulses and SYNC0 pulse"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync1_sync0(&mut self) -> TimeSync1Sync0W<DcSync1CycTimeSpec> {
        TimeSync1Sync0W::new(self, 0)
    }
}
#[doc = "SYNC1 Cycle Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sync1_cyc_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_sync1_cyc_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcSync1CycTimeSpec;
impl crate::RegisterSpec for DcSync1CycTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_sync1_cyc_time::R`](R) reader structure"]
impl crate::Readable for DcSync1CycTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_sync1_cyc_time::W`](W) writer structure"]
impl crate::Writable for DcSync1CycTimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_SYNC1_CYC_TIME to value 0"]
impl crate::Resettable for DcSync1CycTimeSpec {
    const RESET_VALUE: u32 = 0;
}
