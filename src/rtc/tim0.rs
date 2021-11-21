#[doc = "Register `TIM0` reader"]
pub struct R(crate::R<TIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM0` writer"]
pub struct W(crate::W<TIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM0_SPEC>;
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
impl From<crate::W<TIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SE` reader - Seconds Time Value"]
pub struct SE_R(crate::FieldReader<u8, u8>);
impl SE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE` writer - Seconds Time Value"]
pub struct SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `MI` reader - Minutes Time Value"]
pub struct MI_R(crate::FieldReader<u8, u8>);
impl MI_R {
    pub(crate) fn new(bits: u8) -> Self {
        MI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MI` writer - Minutes Time Value"]
pub struct MI_W<'a> {
    w: &'a mut W,
}
impl<'a> MI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `HO` reader - Hours Time Value"]
pub struct HO_R(crate::FieldReader<u8, u8>);
impl HO_R {
    pub(crate) fn new(bits: u8) -> Self {
        HO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HO` writer - Hours Time Value"]
pub struct HO_W<'a> {
    w: &'a mut W,
}
impl<'a> HO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `DA` reader - Days Time Value"]
pub struct DA_R(crate::FieldReader<u8, u8>);
impl DA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DA` writer - Days Time Value"]
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Seconds Time Value"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes Time Value"]
    #[inline(always)]
    pub fn mi(&self) -> MI_R {
        MI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours Time Value"]
    #[inline(always)]
    pub fn ho(&self) -> HO_R {
        HO_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Days Time Value"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds Time Value"]
    #[inline(always)]
    pub fn se(&mut self) -> SE_W {
        SE_W { w: self }
    }
    #[doc = "Bits 8:13 - Minutes Time Value"]
    #[inline(always)]
    pub fn mi(&mut self) -> MI_W {
        MI_W { w: self }
    }
    #[doc = "Bits 16:20 - Hours Time Value"]
    #[inline(always)]
    pub fn ho(&mut self) -> HO_W {
        HO_W { w: self }
    }
    #[doc = "Bits 24:28 - Days Time Value"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Time Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim0](index.html) module"]
pub struct TIM0_SPEC;
impl crate::RegisterSpec for TIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim0::R](R) reader structure"]
impl crate::Readable for TIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim0::W](W) writer structure"]
impl crate::Writable for TIM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM0 to value 0"]
impl crate::Resettable for TIM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
