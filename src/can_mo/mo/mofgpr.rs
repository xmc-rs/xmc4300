#[doc = "Register `MOFGPR` reader"]
pub struct R(crate::R<MOFGPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOFGPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOFGPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOFGPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOFGPR` writer"]
pub struct W(crate::W<MOFGPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOFGPR_SPEC>;
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
impl From<crate::W<MOFGPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOFGPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOT` reader - Bottom Pointer"]
pub struct BOT_R(crate::FieldReader<u8, u8>);
impl BOT_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOT` writer - Bottom Pointer"]
pub struct BOT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TOP` reader - Top Pointer"]
pub struct TOP_R(crate::FieldReader<u8, u8>);
impl TOP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOP` writer - Top Pointer"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CUR` reader - Current Object Pointer"]
pub struct CUR_R(crate::FieldReader<u8, u8>);
impl CUR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CUR` writer - Current Object Pointer"]
pub struct CUR_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SEL` reader - Object Select Pointer"]
pub struct SEL_R(crate::FieldReader<u8, u8>);
impl SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Object Select Pointer"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bottom Pointer"]
    #[inline(always)]
    pub fn bot(&self) -> BOT_R {
        BOT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Top Pointer"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Current Object Pointer"]
    #[inline(always)]
    pub fn cur(&self) -> CUR_R {
        CUR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Object Select Pointer"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bottom Pointer"]
    #[inline(always)]
    pub fn bot(&mut self) -> BOT_W {
        BOT_W { w: self }
    }
    #[doc = "Bits 8:15 - Top Pointer"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
    #[doc = "Bits 16:23 - Current Object Pointer"]
    #[inline(always)]
    pub fn cur(&mut self) -> CUR_W {
        CUR_W { w: self }
    }
    #[doc = "Bits 24:31 - Object Select Pointer"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object FIFO/Gateway Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mofgpr](index.html) module"]
pub struct MOFGPR_SPEC;
impl crate::RegisterSpec for MOFGPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mofgpr::R](R) reader structure"]
impl crate::Readable for MOFGPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mofgpr::W](W) writer structure"]
impl crate::Writable for MOFGPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOFGPR to value 0"]
impl crate::Resettable for MOFGPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
