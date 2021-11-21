#[doc = "Register `FPC` reader"]
pub struct R(crate::R<FPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPC` writer"]
pub struct W(crate::W<FPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPC_SPEC>;
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
impl From<crate::W<FPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCMP` reader - Floating Prescaler Compare Value"]
pub struct PCMP_R(crate::FieldReader<u8, u8>);
impl PCMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVAL` reader - Actual Prescaler Value"]
pub struct PVAL_R(crate::FieldReader<u8, u8>);
impl PVAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVAL` writer - Actual Prescaler Value"]
pub struct PVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Floating Prescaler Compare Value"]
    #[inline(always)]
    pub fn pcmp(&self) -> PCMP_R {
        PCMP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Actual Prescaler Value"]
    #[inline(always)]
    pub fn pval(&self) -> PVAL_R {
        PVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Actual Prescaler Value"]
    #[inline(always)]
    pub fn pval(&mut self) -> PVAL_W {
        PVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Floating Prescaler Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpc](index.html) module"]
pub struct FPC_SPEC;
impl crate::RegisterSpec for FPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpc::R](R) reader structure"]
impl crate::Readable for FPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpc::W](W) writer structure"]
impl crate::Writable for FPC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPC to value 0"]
impl crate::Resettable for FPC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
