#[doc = "Register `ALIAS` reader"]
pub struct R(crate::R<ALIAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALIAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALIAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALIAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALIAS` writer"]
pub struct W(crate::W<ALIAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALIAS_SPEC>;
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
impl From<crate::W<ALIAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALIAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALIAS0` reader - Alias Value for CH0 Conversion Requests"]
pub struct ALIAS0_R(crate::FieldReader<u8, u8>);
impl ALIAS0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALIAS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALIAS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIAS0` writer - Alias Value for CH0 Conversion Requests"]
pub struct ALIAS0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIAS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `ALIAS1` reader - Alias Value for CH1 Conversion Requests"]
pub struct ALIAS1_R(crate::FieldReader<u8, u8>);
impl ALIAS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALIAS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALIAS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIAS1` writer - Alias Value for CH1 Conversion Requests"]
pub struct ALIAS1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIAS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Alias Value for CH0 Conversion Requests"]
    #[inline(always)]
    pub fn alias0(&self) -> ALIAS0_R {
        ALIAS0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Alias Value for CH1 Conversion Requests"]
    #[inline(always)]
    pub fn alias1(&self) -> ALIAS1_R {
        ALIAS1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alias Value for CH0 Conversion Requests"]
    #[inline(always)]
    pub fn alias0(&mut self) -> ALIAS0_W {
        ALIAS0_W { w: self }
    }
    #[doc = "Bits 8:12 - Alias Value for CH1 Conversion Requests"]
    #[inline(always)]
    pub fn alias1(&mut self) -> ALIAS1_W {
        ALIAS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alias Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alias](index.html) module"]
pub struct ALIAS_SPEC;
impl crate::RegisterSpec for ALIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alias::R](R) reader structure"]
impl crate::Readable for ALIAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alias::W](W) writer structure"]
impl crate::Writable for ALIAS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALIAS to value 0x0100"]
impl crate::Resettable for ALIAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
