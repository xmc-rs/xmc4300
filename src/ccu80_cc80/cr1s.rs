#[doc = "Register `CR1S` reader"]
pub struct R(crate::R<CR1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1S` writer"]
pub struct W(crate::W<CR1S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1S_SPEC>;
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
impl From<crate::W<CR1S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR1S` reader - Shadow Compare Register for Channel 1"]
pub type CR1S_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CR1S` writer - Shadow Compare Register for Channel 1"]
pub type CR1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1S_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 1"]
    #[inline(always)]
    pub fn cr1s(&self) -> CR1S_R {
        CR1S_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr1s(&mut self) -> CR1S_W<0> {
        CR1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 1 Compare Shadow Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1s](index.html) module"]
pub struct CR1S_SPEC;
impl crate::RegisterSpec for CR1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1s::R](R) reader structure"]
impl crate::Readable for CR1S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1s::W](W) writer structure"]
impl crate::Writable for CR1S_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1S to value 0"]
impl crate::Resettable for CR1S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
