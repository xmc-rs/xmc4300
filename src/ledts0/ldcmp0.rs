#[doc = "Register `LDCMP0` reader"]
pub struct R(crate::R<LDCMP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDCMP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDCMP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDCMP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDCMP0` writer"]
pub struct W(crate::W<LDCMP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDCMP0_SPEC>;
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
impl From<crate::W<LDCMP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDCMP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP_LD0` reader - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD0_R(crate::FieldReader<u8, u8>);
impl CMP_LD0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_LD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_LD0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_LD0` writer - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CMP_LD1` reader - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD1_R(crate::FieldReader<u8, u8>);
impl CMP_LD1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_LD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_LD1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_LD1` writer - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CMP_LD2` reader - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD2_R(crate::FieldReader<u8, u8>);
impl CMP_LD2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_LD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_LD2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_LD2` writer - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CMP_LD3` reader - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD3_R(crate::FieldReader<u8, u8>);
impl CMP_LD3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_LD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_LD3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_LD3` writer - Compare Value for LED COL\\[x\\]"]
pub struct CMP_LD3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LD3_W<'a> {
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
    pub fn cmp_ld0(&self) -> CMP_LD0_R {
        CMP_LD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld1(&self) -> CMP_LD1_R {
        CMP_LD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld2(&self) -> CMP_LD2_R {
        CMP_LD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld3(&self) -> CMP_LD3_R {
        CMP_LD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld0(&mut self) -> CMP_LD0_W {
        CMP_LD0_W { w: self }
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld1(&mut self) -> CMP_LD1_W {
        CMP_LD1_W { w: self }
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld2(&mut self) -> CMP_LD2_W {
        CMP_LD2_W { w: self }
    }
    #[doc = "Bits 24:31 - Compare Value for LED COL\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ld3(&mut self) -> CMP_LD3_W {
        CMP_LD3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LED Compare Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldcmp0](index.html) module"]
pub struct LDCMP0_SPEC;
impl crate::RegisterSpec for LDCMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldcmp0::R](R) reader structure"]
impl crate::Readable for LDCMP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldcmp0::W](W) writer structure"]
impl crate::Writable for LDCMP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDCMP0 to value 0"]
impl crate::Resettable for LDCMP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
