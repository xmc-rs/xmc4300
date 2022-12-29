#[doc = "Register `MSIMASK` reader"]
pub struct R(crate::R<MSIMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSIMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSIMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSIMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSIMASK` writer"]
pub struct W(crate::W<MSIMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSIMASK_SPEC>;
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
impl From<crate::W<MSIMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSIMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM` reader - Message Index Mask"]
pub type IM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IM` writer - Message Index Mask"]
pub type IM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSIMASK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Message Index Mask"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Index Mask"]
    #[inline(always)]
    #[must_use]
    pub fn im(&mut self) -> IM_W<0> {
        IM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Index Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msimask](index.html) module"]
pub struct MSIMASK_SPEC;
impl crate::RegisterSpec for MSIMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msimask::R](R) reader structure"]
impl crate::Readable for MSIMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msimask::W](W) writer structure"]
impl crate::Writable for MSIMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSIMASK to value 0"]
impl crate::Resettable for MSIMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
