#[doc = "Reader of register HCDMA_BUFFERMODE"]
pub type R = crate::R<u32, super::HCDMA_BUFFERMODE>;
#[doc = "Writer for register HCDMA_BUFFERMODE"]
pub type W = crate::W<u32, super::HCDMA_BUFFERMODE>;
#[doc = "Register HCDMA_BUFFERMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::HCDMA_BUFFERMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMAAddr`"]
pub type DMAADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMAAddr`"]
pub struct DMAADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W {
        DMAADDR_W { w: self }
    }
}
