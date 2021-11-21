#[doc = "Register `PLLCON1` reader"]
pub struct R(crate::R<PLLCON1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCON1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCON1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCON1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCON1` writer"]
pub struct W(crate::W<PLLCON1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCON1_SPEC>;
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
impl From<crate::W<PLLCON1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCON1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `K1DIV` reader - K1-Divider Value"]
pub struct K1DIV_R(crate::FieldReader<u8, u8>);
impl K1DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        K1DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for K1DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `K1DIV` writer - K1-Divider Value"]
pub struct K1DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> K1DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `NDIV` reader - N-Divider Value"]
pub struct NDIV_R(crate::FieldReader<u8, u8>);
impl NDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        NDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDIV` writer - N-Divider Value"]
pub struct NDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> NDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `K2DIV` reader - K2-Divider Value"]
pub struct K2DIV_R(crate::FieldReader<u8, u8>);
impl K2DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        K2DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for K2DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `K2DIV` writer - K2-Divider Value"]
pub struct K2DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> K2DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `PDIV` reader - P-Divider Value"]
pub struct PDIV_R(crate::FieldReader<u8, u8>);
impl PDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIV` writer - P-Divider Value"]
pub struct PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - K1-Divider Value"]
    #[inline(always)]
    pub fn k1div(&self) -> K1DIV_R {
        K1DIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - K2-Divider Value"]
    #[inline(always)]
    pub fn k2div(&self) -> K2DIV_R {
        K2DIV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - K1-Divider Value"]
    #[inline(always)]
    pub fn k1div(&mut self) -> K1DIV_W {
        K1DIV_W { w: self }
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    pub fn ndiv(&mut self) -> NDIV_W {
        NDIV_W { w: self }
    }
    #[doc = "Bits 16:22 - K2-Divider Value"]
    #[inline(always)]
    pub fn k2div(&mut self) -> K2DIV_W {
        K2DIV_W { w: self }
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W {
        PDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcon1](index.html) module"]
pub struct PLLCON1_SPEC;
impl crate::RegisterSpec for PLLCON1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcon1::R](R) reader structure"]
impl crate::Readable for PLLCON1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcon1::W](W) writer structure"]
impl crate::Writable for PLLCON1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCON1 to value 0"]
impl crate::Resettable for PLLCON1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
