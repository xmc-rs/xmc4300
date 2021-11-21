#[doc = "Register `EMUXSEL` reader"]
pub struct R(crate::R<EMUXSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMUXSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMUXSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMUXSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMUXSEL` writer"]
pub struct W(crate::W<EMUXSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMUXSEL_SPEC>;
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
impl From<crate::W<EMUXSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMUXSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMUXGRP0` reader - External Multiplexer Group for Interface x"]
pub struct EMUXGRP0_R(crate::FieldReader<u8, u8>);
impl EMUXGRP0_R {
    pub(crate) fn new(bits: u8) -> Self {
        EMUXGRP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMUXGRP0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMUXGRP0` writer - External Multiplexer Group for Interface x"]
pub struct EMUXGRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUXGRP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `EMUXGRP1` reader - External Multiplexer Group for Interface x"]
pub struct EMUXGRP1_R(crate::FieldReader<u8, u8>);
impl EMUXGRP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        EMUXGRP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMUXGRP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMUXGRP1` writer - External Multiplexer Group for Interface x"]
pub struct EMUXGRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUXGRP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp0(&self) -> EMUXGRP0_R {
        EMUXGRP0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp1(&self) -> EMUXGRP1_R {
        EMUXGRP1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp0(&mut self) -> EMUXGRP0_W {
        EMUXGRP0_W { w: self }
    }
    #[doc = "Bits 4:7 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp1(&mut self) -> EMUXGRP1_W {
        EMUXGRP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Multiplexer Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emuxsel](index.html) module"]
pub struct EMUXSEL_SPEC;
impl crate::RegisterSpec for EMUXSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emuxsel::R](R) reader structure"]
impl crate::Readable for EMUXSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emuxsel::W](W) writer structure"]
impl crate::Writable for EMUXSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMUXSEL to value 0"]
impl crate::Resettable for EMUXSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
