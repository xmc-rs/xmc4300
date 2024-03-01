#[doc = "Register `DOEPDMA0` reader"]
pub type R = crate::R<Doepdma0Spec>;
#[doc = "Register `DOEPDMA0` writer"]
pub type W = crate::W<Doepdma0Spec>;
#[doc = "Field `DMAAddr` reader - DMA Address"]
pub type DmaaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMAAddr` writer - DMA Address"]
pub type DmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DmaaddrR {
        DmaaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DmaaddrW<Doepdma0Spec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "Device Endpoint DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma0::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepdma0Spec;
impl crate::RegisterSpec for Doepdma0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma0::R`](R) reader structure"]
impl crate::Readable for Doepdma0Spec {}
#[doc = "`write(|w| ..)` method takes [`doepdma0::W`](W) writer structure"]
impl crate::Writable for Doepdma0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPDMA0 to value 0"]
impl crate::Resettable for Doepdma0Spec {
    const RESET_VALUE: u32 = 0;
}
