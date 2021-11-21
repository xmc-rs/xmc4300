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
#[doc = "Field `PRD` reader - Parity Read Values for Memory Test"]
pub struct PRD_R(crate::FieldReader<u8, u8>);
impl PRD_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR` reader - Parity Write Values for Memory Test"]
pub struct PWR_R(crate::FieldReader<u8, u8>);
impl PWR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR` writer - Parity Write Values for Memory Test"]
pub struct PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Parity Read Values for Memory Test"]
    #[inline(always)]
    pub fn prd(&self) -> PRD_R {
        PRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Parity Write Values for Memory Test"]
    #[inline(always)]
    pub fn pwr(&self) -> PWR_R {
        PWR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Parity Write Values for Memory Test"]
    #[inline(always)]
    pub fn pwr(&mut self) -> PWR_W {
        PWR_W { w: self }
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
}
#[doc = "`reset()` method sets PMTPR to value 0"]
impl crate::Resettable for PMTPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
