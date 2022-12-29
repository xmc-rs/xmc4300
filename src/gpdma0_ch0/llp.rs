#[doc = "Register `LLP` reader"]
pub struct R(crate::R<LLP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LLP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LLP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LLP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LLP` writer"]
pub struct W(crate::W<LLP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LLP_SPEC>;
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
impl From<crate::W<LLP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LLP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOC` reader - Starting Address In Memory"]
pub type LOC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOC` writer - Starting Address In Memory"]
pub type LOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LLP_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Starting Address In Memory"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Starting Address In Memory"]
    #[inline(always)]
    #[must_use]
    pub fn loc(&mut self) -> LOC_W<2> {
        LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Linked List Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [llp](index.html) module"]
pub struct LLP_SPEC;
impl crate::RegisterSpec for LLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [llp::R](R) reader structure"]
impl crate::Readable for LLP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [llp::W](W) writer structure"]
impl crate::Writable for LLP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LLP to value 0"]
impl crate::Resettable for LLP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
