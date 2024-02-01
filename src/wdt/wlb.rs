#[doc = "Register `WLB` reader"]
pub type R = crate::R<WLB_SPEC>;
#[doc = "Register `WLB` writer"]
pub type W = crate::W<WLB_SPEC>;
#[doc = "Field `WLB` reader - Window Lower Bound"]
pub type WLB_R = crate::FieldReader<u32>;
#[doc = "Field `WLB` writer - Window Lower Bound"]
pub type WLB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Window Lower Bound"]
    #[inline(always)]
    pub fn wlb(&self) -> WLB_R {
        WLB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Window Lower Bound"]
    #[inline(always)]
    #[must_use]
    pub fn wlb(&mut self) -> WLB_W<WLB_SPEC> {
        WLB_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "WDT Window Lower Bound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WLB_SPEC;
impl crate::RegisterSpec for WLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wlb::R`](R) reader structure"]
impl crate::Readable for WLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wlb::W`](W) writer structure"]
impl crate::Writable for WLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WLB to value 0"]
impl crate::Resettable for WLB_SPEC {
    const RESET_VALUE: u32 = 0;
}
