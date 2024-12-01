#[doc = "Register `SRV` writer"]
pub type W = crate::W<SRV_SPEC>;
#[doc = "Field `SRV` writer - Service"]
pub type SRV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Service"]
    #[inline(always)]
    pub fn srv(&mut self) -> SRV_W<SRV_SPEC> {
        SRV_W::new(self, 0)
    }
}
#[doc = "WDT Service Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srv::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRV_SPEC;
impl crate::RegisterSpec for SRV_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`srv::W`](W) writer structure"]
impl crate::Writable for SRV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRV to value 0"]
impl crate::Resettable for SRV_SPEC {
    const RESET_VALUE: u32 = 0;
}
