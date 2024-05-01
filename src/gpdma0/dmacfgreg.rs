#[doc = "Register `DMACFGREG` reader"]
pub type R = crate::R<DMACFGREG_SPEC>;
#[doc = "Register `DMACFGREG` writer"]
pub type W = crate::W<DMACFGREG_SPEC>;
#[doc = "GPDMA Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_EN_A {
    #[doc = "0: GPDMA Disabled"]
    VALUE1 = 0,
    #[doc = "1: GPDMA Enabled."]
    VALUE2 = 1,
}
impl From<DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_EN` reader - GPDMA Enable bit."]
pub type DMA_EN_R = crate::BitReader<DMA_EN_A>;
impl DMA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_EN_A {
        match self.bits {
            false => DMA_EN_A::VALUE1,
            true => DMA_EN_A::VALUE2,
        }
    }
    #[doc = "GPDMA Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DMA_EN_A::VALUE1
    }
    #[doc = "GPDMA Enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DMA_EN_A::VALUE2
    }
}
#[doc = "Field `DMA_EN` writer - GPDMA Enable bit."]
pub type DMA_EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA_EN_A>;
impl<'a, REG> DMA_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPDMA Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_EN_A::VALUE1)
    }
    #[doc = "GPDMA Enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_EN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - GPDMA Enable bit."]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA Enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn dma_en(&mut self) -> DMA_EN_W<DMACFGREG_SPEC> {
        DMA_EN_W::new(self, 0)
    }
}
#[doc = "GPDMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfgreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfgreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACFGREG_SPEC;
impl crate::RegisterSpec for DMACFGREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfgreg::R`](R) reader structure"]
impl crate::Readable for DMACFGREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacfgreg::W`](W) writer structure"]
impl crate::Writable for DMACFGREG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACFGREG to value 0"]
impl crate::Resettable for DMACFGREG_SPEC {
    const RESET_VALUE: u32 = 0;
}
