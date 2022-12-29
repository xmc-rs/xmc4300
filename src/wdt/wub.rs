#[doc = "Register `WUB` reader"]
pub struct R(crate::R<WUB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUB` writer"]
pub struct W(crate::W<WUB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUB_SPEC>;
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
impl From<crate::W<WUB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUB` reader - Window Upper Bound"]
pub type WUB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WUB` writer - Window Upper Bound"]
pub type WUB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Window Upper Bound"]
    #[inline(always)]
    pub fn wub(&self) -> WUB_R {
        WUB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Window Upper Bound"]
    #[inline(always)]
    #[must_use]
    pub fn wub(&mut self) -> WUB_W<0> {
        WUB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Window Upper Bound Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wub](index.html) module"]
pub struct WUB_SPEC;
impl crate::RegisterSpec for WUB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wub::R](R) reader structure"]
impl crate::Readable for WUB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wub::W](W) writer structure"]
impl crate::Writable for WUB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUB to value 0xffff_ffff"]
impl crate::Resettable for WUB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
