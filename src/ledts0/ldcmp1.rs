#[doc = "Register `LDCMP1` reader"]
pub struct R(crate::R<LDCMP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDCMP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDCMP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDCMP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDCMP1` writer"]
pub struct W(crate::W<LDCMP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDCMP1_SPEC>;
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
impl From<crate::W<LDCMP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDCMP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP_LD4` reader - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD4_R(crate::FieldReader<u8, u8>);
impl CMP_LD4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_LD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_LD4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_LD4` writer - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CMP_LD5` reader - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD5_R(crate::FieldReader<u8, u8>);
impl CMP_LD5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_LD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_LD5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_LD5` writer - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CMP_LD6` reader - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD6_R(crate::FieldReader<u8, u8>);
impl CMP_LD6_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_LD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_LD6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_LD6` writer - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CMP_LDA_TSCOM` reader - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
pub struct CMP_LDA_TSCOM_R(crate::FieldReader<u8, u8>);
impl CMP_LDA_TSCOM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_LDA_TSCOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_LDA_TSCOM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_LDA_TSCOM` writer - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
pub struct CMP_LDA_TSCOM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LDA_TSCOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld4(&self) -> CMP_LD4_R {
        CMP_LD4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld5(&self) -> CMP_LD5_R {
        CMP_LD5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld6(&self) -> CMP_LD6_R {
        CMP_LD6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline(always)]
    pub fn cmp_lda_tscom(&self) -> CMP_LDA_TSCOM_R {
        CMP_LDA_TSCOM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld4(&mut self) -> CMP_LD4_W {
        CMP_LD4_W { w: self }
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld5(&mut self) -> CMP_LD5_W {
        CMP_LD5_W { w: self }
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld6(&mut self) -> CMP_LD6_W {
        CMP_LD6_W { w: self }
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline(always)]
    pub fn cmp_lda_tscom(&mut self) -> CMP_LDA_TSCOM_W {
        CMP_LDA_TSCOM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LED Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldcmp1](index.html) module"]
pub struct LDCMP1_SPEC;
impl crate::RegisterSpec for LDCMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldcmp1::R](R) reader structure"]
impl crate::Readable for LDCMP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldcmp1::W](W) writer structure"]
impl crate::Writable for LDCMP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDCMP1 to value 0"]
impl crate::Resettable for LDCMP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
