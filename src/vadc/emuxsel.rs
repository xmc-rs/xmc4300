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
pub type EMUXGRP0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMUXGRP0` writer - External Multiplexer Group for Interface x"]
pub type EMUXGRP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMUXSEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `EMUXGRP1` reader - External Multiplexer Group for Interface x"]
pub type EMUXGRP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMUXGRP1` writer - External Multiplexer Group for Interface x"]
pub type EMUXGRP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMUXSEL_SPEC, u8, u8, 4, O>;
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
    #[must_use]
    pub fn emuxgrp0(&mut self) -> EMUXGRP0_W<0> {
        EMUXGRP0_W::new(self)
    }
    #[doc = "Bits 4:7 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    #[must_use]
    pub fn emuxgrp1(&mut self) -> EMUXGRP1_W<4> {
        EMUXGRP1_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMUXSEL to value 0"]
impl crate::Resettable for EMUXSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
