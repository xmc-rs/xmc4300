#[doc = "Register `WUB` reader"]
pub type R = crate::R<WubSpec>;
#[doc = "Register `WUB` writer"]
pub type W = crate::W<WubSpec>;
#[doc = "Field `WUB` reader - Window Upper Bound"]
pub type WubR = crate::FieldReader<u32>;
#[doc = "Field `WUB` writer - Window Upper Bound"]
pub type WubW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Window Upper Bound"]
    #[inline(always)]
    pub fn wub(&self) -> WubR {
        WubR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Window Upper Bound"]
    #[inline(always)]
    #[must_use]
    pub fn wub(&mut self) -> WubW<WubSpec> {
        WubW::new(self, 0)
    }
}
#[doc = "WDT Window Upper Bound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wub::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wub::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WubSpec;
impl crate::RegisterSpec for WubSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wub::R`](R) reader structure"]
impl crate::Readable for WubSpec {}
#[doc = "`write(|w| ..)` method takes [`wub::W`](W) writer structure"]
impl crate::Writable for WubSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUB to value 0xffff_ffff"]
impl crate::Resettable for WubSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
