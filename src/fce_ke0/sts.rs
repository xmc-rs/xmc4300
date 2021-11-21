#[doc = "Register `STS` reader"]
pub struct R(crate::R<STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STS` writer"]
pub struct W(crate::W<STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STS_SPEC>;
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
impl From<crate::W<STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMF` reader - CRC Mismatch Flag"]
pub struct CMF_R(crate::FieldReader<bool, bool>);
impl CMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMF` writer - CRC Mismatch Flag"]
pub struct CMF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CEF` reader - Configuration Error Flag"]
pub struct CEF_R(crate::FieldReader<bool, bool>);
impl CEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEF` writer - Configuration Error Flag"]
pub struct CEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CEF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `LEF` reader - Length Error Flag"]
pub struct LEF_R(crate::FieldReader<bool, bool>);
impl LEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEF` writer - Length Error Flag"]
pub struct LEF_W<'a> {
    w: &'a mut W,
}
impl<'a> LEF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `BEF` reader - Bus Error Flag"]
pub struct BEF_R(crate::FieldReader<bool, bool>);
impl BEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEF` writer - Bus Error Flag"]
pub struct BEF_W<'a> {
    w: &'a mut W,
}
impl<'a> BEF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CRC Mismatch Flag"]
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configuration Error Flag"]
    #[inline(always)]
    pub fn cef(&self) -> CEF_R {
        CEF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Length Error Flag"]
    #[inline(always)]
    pub fn lef(&self) -> LEF_R {
        LEF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bus Error Flag"]
    #[inline(always)]
    pub fn bef(&self) -> BEF_R {
        BEF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Mismatch Flag"]
    #[inline(always)]
    pub fn cmf(&mut self) -> CMF_W {
        CMF_W { w: self }
    }
    #[doc = "Bit 1 - Configuration Error Flag"]
    #[inline(always)]
    pub fn cef(&mut self) -> CEF_W {
        CEF_W { w: self }
    }
    #[doc = "Bit 2 - Length Error Flag"]
    #[inline(always)]
    pub fn lef(&mut self) -> LEF_W {
        LEF_W { w: self }
    }
    #[doc = "Bit 3 - Bus Error Flag"]
    #[inline(always)]
    pub fn bef(&mut self) -> BEF_W {
        BEF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts::R](R) reader structure"]
impl crate::Readable for STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sts::W](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
