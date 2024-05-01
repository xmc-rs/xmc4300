#[doc = "Register `WD_TIME_PDATA` reader"]
pub type R = crate::R<WdTimePdataSpec>;
#[doc = "Register `WD_TIME_PDATA` writer"]
pub type W = crate::W<WdTimePdataSpec>;
#[doc = "Field `WD_TIME_PD` reader - Watchdog Time Process Data"]
pub type WdTimePdR = crate::FieldReader<u16>;
#[doc = "Field `WD_TIME_PD` writer - Watchdog Time Process Data"]
pub type WdTimePdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Watchdog Time Process Data"]
    #[inline(always)]
    pub fn wd_time_pd(&self) -> WdTimePdR {
        WdTimePdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog Time Process Data"]
    #[inline(always)]
    #[must_use]
    pub fn wd_time_pd(&mut self) -> WdTimePdW<WdTimePdataSpec> {
        WdTimePdW::new(self, 0)
    }
}
#[doc = "Watchdog Time Process Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_time_pdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wd_time_pdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdTimePdataSpec;
impl crate::RegisterSpec for WdTimePdataSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wd_time_pdata::R`](R) reader structure"]
impl crate::Readable for WdTimePdataSpec {}
#[doc = "`write(|w| ..)` method takes [`wd_time_pdata::W`](W) writer structure"]
impl crate::Writable for WdTimePdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WD_TIME_PDATA to value 0x03e8"]
impl crate::Resettable for WdTimePdataSpec {
    const RESET_VALUE: u16 = 0x03e8;
}
