#[doc = "Register `DSTATAR` reader"]
pub struct R(crate::R<DSTATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSTATAR` writer"]
pub struct W(crate::W<DSTATAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSTATAR_SPEC>;
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
impl From<crate::W<DSTATAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSTATAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTATAR` reader - Destination Status Address"]
pub type DSTATAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSTATAR` writer - Destination Status Address"]
pub type DSTATAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSTATAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Destination Status Address"]
    #[inline(always)]
    pub fn dstatar(&self) -> DSTATAR_R {
        DSTATAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Status Address"]
    #[inline(always)]
    #[must_use]
    pub fn dstatar(&mut self) -> DSTATAR_W<0> {
        DSTATAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination Status Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dstatar](index.html) module"]
pub struct DSTATAR_SPEC;
impl crate::RegisterSpec for DSTATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dstatar::R](R) reader structure"]
impl crate::Readable for DSTATAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dstatar::W](W) writer structure"]
impl crate::Writable for DSTATAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSTATAR to value 0"]
impl crate::Resettable for DSTATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
