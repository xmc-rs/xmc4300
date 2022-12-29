#[doc = "Register `HCDMA_SCATGATHER` reader"]
pub struct R(crate::R<HCDMA_SCATGATHER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMA_SCATGATHER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMA_SCATGATHER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMA_SCATGATHER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDMA_SCATGATHER` writer"]
pub struct W(crate::W<HCDMA_SCATGATHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDMA_SCATGATHER_SPEC>;
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
impl From<crate::W<HCDMA_SCATGATHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDMA_SCATGATHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTD` reader - Current Transfer Desc:"]
pub type CTD_R = crate::FieldReader<u8, CTD_A>;
#[doc = "Current Transfer Desc:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTD_A {
    #[doc = "0: 1 descriptor"]
    VALUE1 = 0,
    #[doc = "63: 64 descriptors"]
    VALUE2 = 63,
}
impl From<CTD_A> for u8 {
    #[inline(always)]
    fn from(variant: CTD_A) -> Self {
        variant as _
    }
}
impl CTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTD_A> {
        match self.bits {
            0 => Some(CTD_A::VALUE1),
            63 => Some(CTD_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CTD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CTD_A::VALUE2
    }
}
#[doc = "Field `CTD` writer - Current Transfer Desc:"]
pub type CTD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCDMA_SCATGATHER_SPEC, u8, CTD_A, 6, O>;
impl<'a, const O: u8> CTD_W<'a, O> {
    #[doc = "1 descriptor"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTD_A::VALUE1)
    }
    #[doc = "64 descriptors"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTD_A::VALUE2)
    }
}
#[doc = "Field `DMAAddr` reader - DMA Address"]
pub type DMAADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMAAddr` writer - DMA Address"]
pub type DMAADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCDMA_SCATGATHER_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bits 3:8 - Current Transfer Desc:"]
    #[inline(always)]
    pub fn ctd(&self) -> CTD_R {
        CTD_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 3:8 - Current Transfer Desc:"]
    #[inline(always)]
    #[must_use]
    pub fn ctd(&mut self) -> CTD_W<3> {
        CTD_W::new(self)
    }
    #[doc = "Bits 9:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<9> {
        DMAADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel DMA Address Register \\[SCATGATHER\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma_scatgather](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct HCDMA_SCATGATHER_SPEC;
impl crate::RegisterSpec for HCDMA_SCATGATHER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdma_scatgather::R](R) reader structure"]
impl crate::Readable for HCDMA_SCATGATHER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdma_scatgather::W](W) writer structure"]
impl crate::Writable for HCDMA_SCATGATHER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCDMA_SCATGATHER to value 0"]
impl crate::Resettable for HCDMA_SCATGATHER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
