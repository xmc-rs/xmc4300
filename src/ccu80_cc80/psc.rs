#[doc = "Register `PSC` reader"]
pub struct R(crate::R<PSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSC` writer"]
pub struct W(crate::W<PSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSC_SPEC>;
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
impl From<crate::W<PSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSIV` reader - Prescaler Initial Value"]
pub type PSIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSIV` writer - Prescaler Initial Value"]
pub type PSIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Prescaler Initial Value"]
    #[inline(always)]
    pub fn psiv(&self) -> PSIV_R {
        PSIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn psiv(&mut self) -> PSIV_W<0> {
        PSIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prescaler Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psc](index.html) module"]
pub struct PSC_SPEC;
impl crate::RegisterSpec for PSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psc::R](R) reader structure"]
impl crate::Readable for PSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psc::W](W) writer structure"]
impl crate::Writable for PSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSC to value 0"]
impl crate::Resettable for PSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
