#[doc = "Register `CHECK` reader"]
pub struct R(crate::R<CHECK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHECK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHECK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHECK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHECK` writer"]
pub struct W(crate::W<CHECK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHECK_SPEC>;
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
impl From<crate::W<CHECK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHECK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHECK` reader - CHECK Register"]
pub type CHECK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CHECK` writer - CHECK Register"]
pub type CHECK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHECK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CHECK Register"]
    #[inline(always)]
    pub fn check(&self) -> CHECK_R {
        CHECK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CHECK Register"]
    #[inline(always)]
    #[must_use]
    pub fn check(&mut self) -> CHECK_W<0> {
        CHECK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Check Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [check](index.html) module"]
pub struct CHECK_SPEC;
impl crate::RegisterSpec for CHECK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [check::R](R) reader structure"]
impl crate::Readable for CHECK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [check::W](W) writer structure"]
impl crate::Writable for CHECK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHECK to value 0"]
impl crate::Resettable for CHECK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
