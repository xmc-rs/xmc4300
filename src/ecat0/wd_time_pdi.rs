#[doc = "Register `WD_TIME_PDI` reader"]
pub type R = crate::R<WdTimePdiSpec>;
#[doc = "Register `WD_TIME_PDI` writer"]
pub type W = crate::W<WdTimePdiSpec>;
#[doc = "Field `WD_TIME_PDI` reader - Watchdog Time PDI"]
pub type WdTimePdiR = crate::FieldReader<u16>;
#[doc = "Field `WD_TIME_PDI` writer - Watchdog Time PDI"]
pub type WdTimePdiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Watchdog Time PDI"]
    #[inline(always)]
    pub fn wd_time_pdi(&self) -> WdTimePdiR {
        WdTimePdiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog Time PDI"]
    #[inline(always)]
    #[must_use]
    pub fn wd_time_pdi(&mut self) -> WdTimePdiW<WdTimePdiSpec> {
        WdTimePdiW::new(self, 0)
    }
}
#[doc = "Watchdog Time PDI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_time_pdi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wd_time_pdi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdTimePdiSpec;
impl crate::RegisterSpec for WdTimePdiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wd_time_pdi::R`](R) reader structure"]
impl crate::Readable for WdTimePdiSpec {}
#[doc = "`write(|w| ..)` method takes [`wd_time_pdi::W`](W) writer structure"]
impl crate::Writable for WdTimePdiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WD_TIME_PDI to value 0x03e8"]
impl crate::Resettable for WdTimePdiSpec {
    const RESET_VALUE: u16 = 0x03e8;
}
