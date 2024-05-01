#[doc = "Register `WDTCLR` writer"]
pub type W = crate::W<WdtclrSpec>;
#[doc = "Field `ALMC` writer - Pre-warning Alarm"]
pub type AlmcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Pre-warning Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn almc(&mut self) -> AlmcW<WdtclrSpec> {
        AlmcW::new(self, 0)
    }
}
#[doc = "WDT Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtclrSpec;
impl crate::RegisterSpec for WdtclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdtclr::W`](W) writer structure"]
impl crate::Writable for WdtclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTCLR to value 0"]
impl crate::Resettable for WdtclrSpec {
    const RESET_VALUE: u32 = 0;
}
