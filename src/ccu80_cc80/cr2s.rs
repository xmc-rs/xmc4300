#[doc = "Register `CR2S` reader"]
pub struct R(crate::R<CR2S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2S` writer"]
pub struct W(crate::W<CR2S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2S_SPEC>;
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
impl From<crate::W<CR2S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR2S` reader - Shadow Compare Register for Channel 2"]
pub struct CR2S_R(crate::FieldReader<u16, u16>);
impl CR2S_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR2S_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR2S` writer - Shadow Compare Register for Channel 2"]
pub struct CR2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CR2S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 2"]
    #[inline(always)]
    pub fn cr2s(&self) -> CR2S_R {
        CR2S_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 2"]
    #[inline(always)]
    pub fn cr2s(&mut self) -> CR2S_W {
        CR2S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 2 Compare Shadow Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2s](index.html) module"]
pub struct CR2S_SPEC;
impl crate::RegisterSpec for CR2S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2s::R](R) reader structure"]
impl crate::Readable for CR2S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2s::W](W) writer structure"]
impl crate::Writable for CR2S_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2S to value 0"]
impl crate::Resettable for CR2S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
