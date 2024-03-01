#[doc = "Register `SRV` writer"]
pub type W = crate::W<SrvSpec>;
#[doc = "Field `SRV` writer - Service"]
pub type SrvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Service"]
    #[inline(always)]
    #[must_use]
    pub fn srv(&mut self) -> SrvW<SrvSpec> {
        SrvW::new(self, 0)
    }
}
#[doc = "WDT Service Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srv::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrvSpec;
impl crate::RegisterSpec for SrvSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`srv::W`](W) writer structure"]
impl crate::Writable for SrvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRV to value 0"]
impl crate::Resettable for SrvSpec {
    const RESET_VALUE: u32 = 0;
}
