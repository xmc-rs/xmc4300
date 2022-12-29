#[doc = "Register `DOEPDMA` reader"]
pub struct R(crate::R<DOEPDMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPDMA` writer"]
pub struct W(crate::W<DOEPDMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DOEPDMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAAddr` reader - DMA Address"]
pub type DMAADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMAAddr` writer - DMA Address"]
pub type DMAADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPDMA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<0> {
        DMAADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepdma](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct DOEPDMA_SPEC;
impl crate::RegisterSpec for DOEPDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepdma::R](R) reader structure"]
impl crate::Readable for DOEPDMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepdma::W](W) writer structure"]
impl crate::Writable for DOEPDMA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMA to value 0"]
impl crate::Resettable for DOEPDMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
