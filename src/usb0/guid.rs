#[doc = "Register `GUID` reader"]
pub struct R(crate::R<GUID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GUID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GUID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GUID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GUID` writer"]
pub struct W(crate::W<GUID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GUID_SPEC>;
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
impl From<crate::W<GUID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GUID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOD_REV` reader - Module Revision"]
pub struct MOD_REV_R(crate::FieldReader<u8, u8>);
impl MOD_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MOD_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD_REV` writer - Module Revision"]
pub struct MOD_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `MOD_TYPE` reader - Module Type"]
pub struct MOD_TYPE_R(crate::FieldReader<u8, u8>);
impl MOD_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MOD_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD_TYPE` writer - Module Type"]
pub struct MOD_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `MOD_NUMBER` reader - Module Number"]
pub struct MOD_NUMBER_R(crate::FieldReader<u16, u16>);
impl MOD_NUMBER_R {
    pub(crate) fn new(bits: u16) -> Self {
        MOD_NUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD_NUMBER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD_NUMBER` writer - Module Number"]
pub struct MOD_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    pub fn mod_rev(&self) -> MOD_REV_R {
        MOD_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn mod_type(&self) -> MOD_TYPE_R {
        MOD_TYPE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    pub fn mod_number(&self) -> MOD_NUMBER_R {
        MOD_NUMBER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    pub fn mod_rev(&mut self) -> MOD_REV_W {
        MOD_REV_W { w: self }
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn mod_type(&mut self) -> MOD_TYPE_W {
        MOD_TYPE_W { w: self }
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    pub fn mod_number(&mut self) -> MOD_NUMBER_W {
        MOD_NUMBER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Module Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [guid](index.html) module"]
pub struct GUID_SPEC;
impl crate::RegisterSpec for GUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [guid::R](R) reader structure"]
impl crate::Readable for GUID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [guid::W](W) writer structure"]
impl crate::Writable for GUID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GUID to value 0x00ae_c000"]
impl crate::Resettable for GUID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ae_c000
    }
}
