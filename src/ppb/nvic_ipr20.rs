#[doc = "Register `NVIC_IPR20` reader"]
pub struct R(crate::R<NVIC_IPR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR20` writer"]
pub struct W(crate::W<NVIC_IPR20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR20_SPEC>;
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
impl From<crate::W<NVIC_IPR20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_0` reader - Priority value 0"]
pub struct PRI_0_R(crate::FieldReader<u8, u8>);
impl PRI_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_0` writer - Priority value 0"]
pub struct PRI_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PRI_1` reader - Priority value 1"]
pub struct PRI_1_R(crate::FieldReader<u8, u8>);
impl PRI_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_1` writer - Priority value 1"]
pub struct PRI_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `PRI_2` reader - Priority value 2"]
pub struct PRI_2_R(crate::FieldReader<u8, u8>);
impl PRI_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_2` writer - Priority value 2"]
pub struct PRI_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PRI_3` reader - Priority value 3"]
pub struct PRI_3_R(crate::FieldReader<u8, u8>);
impl PRI_3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_3` writer - Priority value 3"]
pub struct PRI_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority value 0"]
    #[inline(always)]
    pub fn pri_0(&self) -> PRI_0_R {
        PRI_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority value 1"]
    #[inline(always)]
    pub fn pri_1(&self) -> PRI_1_R {
        PRI_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Priority value 2"]
    #[inline(always)]
    pub fn pri_2(&self) -> PRI_2_R {
        PRI_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Priority value 3"]
    #[inline(always)]
    pub fn pri_3(&self) -> PRI_3_R {
        PRI_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority value 0"]
    #[inline(always)]
    pub fn pri_0(&mut self) -> PRI_0_W {
        PRI_0_W { w: self }
    }
    #[doc = "Bits 8:15 - Priority value 1"]
    #[inline(always)]
    pub fn pri_1(&mut self) -> PRI_1_W {
        PRI_1_W { w: self }
    }
    #[doc = "Bits 16:23 - Priority value 2"]
    #[inline(always)]
    pub fn pri_2(&mut self) -> PRI_2_W {
        PRI_2_W { w: self }
    }
    #[doc = "Bits 24:31 - Priority value 3"]
    #[inline(always)]
    pub fn pri_3(&mut self) -> PRI_3_W {
        PRI_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr20](index.html) module"]
pub struct NVIC_IPR20_SPEC;
impl crate::RegisterSpec for NVIC_IPR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr20::R](R) reader structure"]
impl crate::Readable for NVIC_IPR20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr20::W](W) writer structure"]
impl crate::Writable for NVIC_IPR20_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IPR20 to value 0"]
impl crate::Resettable for NVIC_IPR20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
