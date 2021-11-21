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
#[doc = "Current Transfer Desc:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CTD` reader - Current Transfer Desc:"]
pub struct CTD_R(crate::FieldReader<u8, CTD_A>);
impl CTD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CTD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CTD_A::VALUE2
    }
}
impl core::ops::Deref for CTD_R {
    type Target = crate::FieldReader<u8, CTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTD` writer - Current Transfer Desc:"]
pub struct CTD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 3)) | ((value as u32 & 0x3f) << 3);
        self.w
    }
}
#[doc = "Field `DMAAddr` reader - DMA Address"]
pub struct DMAADDR_R(crate::FieldReader<u32, u32>);
impl DMAADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMAADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAAddr` writer - DMA Address"]
pub struct DMAADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | ((value as u32 & 0x007f_ffff) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:8 - Current Transfer Desc:"]
    #[inline(always)]
    pub fn ctd(&self) -> CTD_R {
        CTD_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:8 - Current Transfer Desc:"]
    #[inline(always)]
    pub fn ctd(&mut self) -> CTD_W {
        CTD_W { w: self }
    }
    #[doc = "Bits 9:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W {
        DMAADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel DMA Address Register \\[SCATGATHER\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma_scatgather](index.html) module"]
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
}
#[doc = "`reset()` method sets HCDMA_SCATGATHER to value 0"]
impl crate::Resettable for HCDMA_SCATGATHER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
