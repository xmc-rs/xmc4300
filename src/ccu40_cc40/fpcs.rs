#[doc = "Register `FPCS` reader"]
pub struct R(crate::R<FPCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPCS` writer"]
pub struct W(crate::W<FPCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPCS_SPEC>;
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
impl From<crate::W<FPCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCMP` reader - Floating Prescaler Shadow Compare Value"]
pub type PCMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCMP` writer - Floating Prescaler Shadow Compare Value"]
pub type PCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPCS_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Floating Prescaler Shadow Compare Value"]
    #[inline(always)]
    pub fn pcmp(&self) -> PCMP_R {
        PCMP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Floating Prescaler Shadow Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn pcmp(&mut self) -> PCMP_W<0> {
        PCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Floating Prescaler Shadow\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpcs](index.html) module"]
pub struct FPCS_SPEC;
impl crate::RegisterSpec for FPCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpcs::R](R) reader structure"]
impl crate::Readable for FPCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpcs::W](W) writer structure"]
impl crate::Writable for FPCS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPCS to value 0"]
impl crate::Resettable for FPCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
