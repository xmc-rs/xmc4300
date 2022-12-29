#[doc = "Register `PMTPR` reader"]
pub struct R(crate::R<PMTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMTPR` writer"]
pub struct W(crate::W<PMTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMTPR_SPEC>;
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
impl From<crate::W<PMTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR` reader - Parity Write Values for Memory Test"]
pub type PWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR` writer - Parity Write Values for Memory Test"]
pub type PWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMTPR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRD` reader - Parity Read Values for Memory Test"]
pub type PRD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Parity Write Values for Memory Test"]
    #[inline(always)]
    pub fn pwr(&self) -> PWR_R {
        PWR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Parity Read Values for Memory Test"]
    #[inline(always)]
    pub fn prd(&self) -> PRD_R {
        PRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Parity Write Values for Memory Test"]
    #[inline(always)]
    #[must_use]
    pub fn pwr(&mut self) -> PWR_W<0> {
        PWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Memory Test Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmtpr](index.html) module"]
pub struct PMTPR_SPEC;
impl crate::RegisterSpec for PMTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmtpr::R](R) reader structure"]
impl crate::Readable for PMTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmtpr::W](W) writer structure"]
impl crate::Writable for PMTPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMTPR to value 0"]
impl crate::Resettable for PMTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
