#[doc = "Register `TSCMP1` reader"]
pub struct R(crate::R<TSCMP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCMP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCMP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCMP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSCMP1` writer"]
pub struct W(crate::W<TSCMP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSCMP1_SPEC>;
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
impl From<crate::W<TSCMP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSCMP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP_TS4` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub struct CMP_TS4_R(crate::FieldReader<u8, u8>);
impl CMP_TS4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_TS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_TS4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_TS4` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub struct CMP_TS4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CMP_TS5` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub struct CMP_TS5_R(crate::FieldReader<u8, u8>);
impl CMP_TS5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_TS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_TS5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_TS5` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub struct CMP_TS5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CMP_TS6` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub struct CMP_TS6_R(crate::FieldReader<u8, u8>);
impl CMP_TS6_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_TS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_TS6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_TS6` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub struct CMP_TS6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CMP_TS7` reader - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub struct CMP_TS7_R(crate::FieldReader<u8, u8>);
impl CMP_TS7_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_TS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_TS7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_TS7` writer - Compare Value for Touch-Sense TSIN\\[x\\]"]
pub struct CMP_TS7_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TS7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts4(&self) -> CMP_TS4_R {
        CMP_TS4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts5(&self) -> CMP_TS5_R {
        CMP_TS5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts6(&self) -> CMP_TS6_R {
        CMP_TS6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts7(&self) -> CMP_TS7_R {
        CMP_TS7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts4(&mut self) -> CMP_TS4_W {
        CMP_TS4_W { w: self }
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts5(&mut self) -> CMP_TS5_W {
        CMP_TS5_W { w: self }
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts6(&mut self) -> CMP_TS6_W {
        CMP_TS6_W { w: self }
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline(always)]
    pub fn cmp_ts7(&mut self) -> CMP_TS7_W {
        CMP_TS7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Touch-sense Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscmp1](index.html) module"]
pub struct TSCMP1_SPEC;
impl crate::RegisterSpec for TSCMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tscmp1::R](R) reader structure"]
impl crate::Readable for TSCMP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tscmp1::W](W) writer structure"]
impl crate::Writable for TSCMP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSCMP1 to value 0"]
impl crate::Resettable for TSCMP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
