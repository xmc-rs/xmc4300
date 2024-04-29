#[doc = "Register `WD_TIME_PDI` reader"]
pub type R = crate::R<WD_TIME_PDI_SPEC>;
#[doc = "Register `WD_TIME_PDI` writer"]
pub type W = crate::W<WD_TIME_PDI_SPEC>;
#[doc = "Field `WD_TIME_PDI` reader - Watchdog Time PDI"]
pub type WD_TIME_PDI_R = crate::FieldReader<u16>;
#[doc = "Field `WD_TIME_PDI` writer - Watchdog Time PDI"]
pub type WD_TIME_PDI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Watchdog Time PDI"]
    #[inline(always)]
    pub fn wd_time_pdi(&self) -> WD_TIME_PDI_R {
        WD_TIME_PDI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog Time PDI"]
    #[inline(always)]
    #[must_use]
    pub fn wd_time_pdi(&mut self) -> WD_TIME_PDI_W<WD_TIME_PDI_SPEC> {
        WD_TIME_PDI_W::new(self, 0)
    }
}
#[doc = "Watchdog Time PDI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_time_pdi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wd_time_pdi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WD_TIME_PDI_SPEC;
impl crate::RegisterSpec for WD_TIME_PDI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wd_time_pdi::R`](R) reader structure"]
impl crate::Readable for WD_TIME_PDI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wd_time_pdi::W`](W) writer structure"]
impl crate::Writable for WD_TIME_PDI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WD_TIME_PDI to value 0x03e8"]
impl crate::Resettable for WD_TIME_PDI_SPEC {
    const RESET_VALUE: u16 = 0x03e8;
}
