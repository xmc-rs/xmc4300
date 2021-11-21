#[doc = "Register `MODATAL` reader"]
pub struct R(crate::R<MODATAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODATAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODATAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODATAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODATAL` writer"]
pub struct W(crate::W<MODATAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODATAL_SPEC>;
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
impl From<crate::W<MODATAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODATAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DB0` reader - Data Byte 0 of Message Object n"]
pub struct DB0_R(crate::FieldReader<u8, u8>);
impl DB0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB0` writer - Data Byte 0 of Message Object n"]
pub struct DB0_W<'a> {
    w: &'a mut W,
}
impl<'a> DB0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DB1` reader - Data Byte 1 of Message Object n"]
pub struct DB1_R(crate::FieldReader<u8, u8>);
impl DB1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1` writer - Data Byte 1 of Message Object n"]
pub struct DB1_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `DB2` reader - Data Byte 2 of Message Object n"]
pub struct DB2_R(crate::FieldReader<u8, u8>);
impl DB2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB2` writer - Data Byte 2 of Message Object n"]
pub struct DB2_W<'a> {
    w: &'a mut W,
}
impl<'a> DB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DB3` reader - Data Byte 3 of Message Object n"]
pub struct DB3_R(crate::FieldReader<u8, u8>);
impl DB3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB3` writer - Data Byte 3 of Message Object n"]
pub struct DB3_W<'a> {
    w: &'a mut W,
}
impl<'a> DB3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Byte 0 of Message Object n"]
    #[inline(always)]
    pub fn db0(&self) -> DB0_R {
        DB0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 1 of Message Object n"]
    #[inline(always)]
    pub fn db1(&self) -> DB1_R {
        DB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Byte 2 of Message Object n"]
    #[inline(always)]
    pub fn db2(&self) -> DB2_R {
        DB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data Byte 3 of Message Object n"]
    #[inline(always)]
    pub fn db3(&self) -> DB3_R {
        DB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 0 of Message Object n"]
    #[inline(always)]
    pub fn db0(&mut self) -> DB0_W {
        DB0_W { w: self }
    }
    #[doc = "Bits 8:15 - Data Byte 1 of Message Object n"]
    #[inline(always)]
    pub fn db1(&mut self) -> DB1_W {
        DB1_W { w: self }
    }
    #[doc = "Bits 16:23 - Data Byte 2 of Message Object n"]
    #[inline(always)]
    pub fn db2(&mut self) -> DB2_W {
        DB2_W { w: self }
    }
    #[doc = "Bits 24:31 - Data Byte 3 of Message Object n"]
    #[inline(always)]
    pub fn db3(&mut self) -> DB3_W {
        DB3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Data Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modatal](index.html) module"]
pub struct MODATAL_SPEC;
impl crate::RegisterSpec for MODATAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modatal::R](R) reader structure"]
impl crate::Readable for MODATAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modatal::W](W) writer structure"]
impl crate::Writable for MODATAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODATAL to value 0"]
impl crate::Resettable for MODATAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
