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
pub type CR2S_R = crate::FieldReader<u16>;
#[doc = "Field `CR2S` writer - Shadow Compare Register for Channel 2"]
pub type CR2S_W<'a, const O: u8> = crate::FieldWriter<'a, CR2S_SPEC, 16, O, u16>;
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
    #[must_use]
    pub fn cr2s(&mut self) -> CR2S_W<0> {
        CR2S_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2S to value 0"]
impl crate::Resettable for CR2S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
