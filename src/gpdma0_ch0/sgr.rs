#[doc = "Register `SGR` reader"]
pub struct R(crate::R<SGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SGR` writer"]
pub struct W(crate::W<SGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SGR_SPEC>;
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
impl From<crate::W<SGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGC` reader - Source gather count"]
pub struct SGC_R(crate::FieldReader<u16, u16>);
impl SGC_R {
    pub(crate) fn new(bits: u16) -> Self {
        SGC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SGC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SGC` writer - Source gather count"]
pub struct SGC_W<'a> {
    w: &'a mut W,
}
impl<'a> SGC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | ((value as u32 & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Field `SGI` reader - Source gather interval"]
pub struct SGI_R(crate::FieldReader<u32, u32>);
impl SGI_R {
    pub(crate) fn new(bits: u32) -> Self {
        SGI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SGI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SGI` writer - Source gather interval"]
pub struct SGI_W<'a> {
    w: &'a mut W,
}
impl<'a> SGI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - Source gather count"]
    #[inline(always)]
    pub fn sgc(&self) -> SGC_R {
        SGC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:19 - Source gather interval"]
    #[inline(always)]
    pub fn sgi(&self) -> SGI_R {
        SGI_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 20:31 - Source gather count"]
    #[inline(always)]
    pub fn sgc(&mut self) -> SGC_W {
        SGC_W { w: self }
    }
    #[doc = "Bits 0:19 - Source gather interval"]
    #[inline(always)]
    pub fn sgi(&mut self) -> SGI_W {
        SGI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Gather Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sgr](index.html) module"]
pub struct SGR_SPEC;
impl crate::RegisterSpec for SGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sgr::R](R) reader structure"]
impl crate::Readable for SGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sgr::W](W) writer structure"]
impl crate::Writable for SGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SGR to value 0"]
impl crate::Resettable for SGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
