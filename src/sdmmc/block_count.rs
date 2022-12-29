#[doc = "Register `BLOCK_COUNT` reader"]
pub struct R(crate::R<BLOCK_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLOCK_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLOCK_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLOCK_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLOCK_COUNT` writer"]
pub struct W(crate::W<BLOCK_COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLOCK_COUNT_SPEC>;
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
impl From<crate::W<BLOCK_COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLOCK_COUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK_COUNT` reader - Blocks Count for Current Transfer"]
pub type BLOCK_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLOCK_COUNT` writer - Blocks Count for Current Transfer"]
pub type BLOCK_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, BLOCK_COUNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn block_count(&self) -> BLOCK_COUNT_R {
        BLOCK_COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn block_count(&mut self) -> BLOCK_COUNT_W<0> {
        BLOCK_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [block_count](index.html) module"]
pub struct BLOCK_COUNT_SPEC;
impl crate::RegisterSpec for BLOCK_COUNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [block_count::R](R) reader structure"]
impl crate::Readable for BLOCK_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [block_count::W](W) writer structure"]
impl crate::Writable for BLOCK_COUNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLOCK_COUNT to value 0"]
impl crate::Resettable for BLOCK_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
