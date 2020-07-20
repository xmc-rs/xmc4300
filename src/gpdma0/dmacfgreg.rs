#[doc = "Reader of register DMACFGREG"]
pub type R = crate::R<u32, super::DMACFGREG>;
#[doc = "Writer for register DMACFGREG"]
pub type W = crate::W<u32, super::DMACFGREG>;
#[doc = "Register DMACFGREG `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACFGREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPDMA Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `DMA_EN`"]
pub type DMA_EN_R = crate::R<bool, DMA_EN_A>;
impl DMA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_EN_A {
        match self.bits {
            false => DMA_EN_A::VALUE1,
            true => DMA_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DMA_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DMA_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `DMA_EN`"]
pub struct DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "GPDMA Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMA_EN_A::VALUE1)
    }
    #[doc = "GPDMA Enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMA_EN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPDMA Enable bit."]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA Enable bit."]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DMA_EN_W {
        DMA_EN_W { w: self }
    }
}
