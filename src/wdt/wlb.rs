#[doc = "Register `WLB` reader"]
pub type R = crate::R<WlbSpec>;
#[doc = "Register `WLB` writer"]
pub type W = crate::W<WlbSpec>;
#[doc = "Field `WLB` reader - Window Lower Bound"]
pub type WlbR = crate::FieldReader<u32>;
#[doc = "Field `WLB` writer - Window Lower Bound"]
pub type WlbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Window Lower Bound"]
    #[inline(always)]
    pub fn wlb(&self) -> WlbR {
        WlbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Window Lower Bound"]
    #[inline(always)]
    #[must_use]
    pub fn wlb(&mut self) -> WlbW<WlbSpec> {
        WlbW::new(self, 0)
    }
}
#[doc = "WDT Window Lower Bound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WlbSpec;
impl crate::RegisterSpec for WlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wlb::R`](R) reader structure"]
impl crate::Readable for WlbSpec {}
#[doc = "`write(|w| ..)` method takes [`wlb::W`](W) writer structure"]
impl crate::Writable for WlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WLB to value 0"]
impl crate::Resettable for WlbSpec {
    const RESET_VALUE: u32 = 0;
}
