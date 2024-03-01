#[doc = "Register `DMACFGREG` reader"]
pub type R = crate::R<DmacfgregSpec>;
#[doc = "Register `DMACFGREG` writer"]
pub type W = crate::W<DmacfgregSpec>;
#[doc = "GPDMA Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaEn {
    #[doc = "0: GPDMA Disabled"]
    Value1 = 0,
    #[doc = "1: GPDMA Enabled."]
    Value2 = 1,
}
impl From<DmaEn> for bool {
    #[inline(always)]
    fn from(variant: DmaEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_EN` reader - GPDMA Enable bit."]
pub type DmaEnR = crate::BitReader<DmaEn>;
impl DmaEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaEn {
        match self.bits {
            false => DmaEn::Value1,
            true => DmaEn::Value2,
        }
    }
    #[doc = "GPDMA Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DmaEn::Value1
    }
    #[doc = "GPDMA Enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DmaEn::Value2
    }
}
#[doc = "Field `DMA_EN` writer - GPDMA Enable bit."]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG, DmaEn>;
impl<'a, REG> DmaEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPDMA Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DmaEn::Value1)
    }
    #[doc = "GPDMA Enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DmaEn::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - GPDMA Enable bit."]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA Enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn dma_en(&mut self) -> DmaEnW<DmacfgregSpec> {
        DmaEnW::new(self, 0)
    }
}
#[doc = "GPDMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfgreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfgreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacfgregSpec;
impl crate::RegisterSpec for DmacfgregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfgreg::R`](R) reader structure"]
impl crate::Readable for DmacfgregSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacfgreg::W`](W) writer structure"]
impl crate::Writable for DmacfgregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACFGREG to value 0"]
impl crate::Resettable for DmacfgregSpec {
    const RESET_VALUE: u32 = 0;
}
