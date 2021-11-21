#[doc = "Register `ARGUMENT1` reader"]
pub struct R(crate::R<ARGUMENT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARGUMENT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARGUMENT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARGUMENT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARGUMENT1` writer"]
pub struct W(crate::W<ARGUMENT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARGUMENT1_SPEC>;
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
impl From<crate::W<ARGUMENT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARGUMENT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARGUMENT1` reader - Command Argument"]
pub struct ARGUMENT1_R(crate::FieldReader<u32, u32>);
impl ARGUMENT1_R {
    pub(crate) fn new(bits: u32) -> Self {
        ARGUMENT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARGUMENT1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARGUMENT1` writer - Command Argument"]
pub struct ARGUMENT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ARGUMENT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn argument1(&self) -> ARGUMENT1_R {
        ARGUMENT1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn argument1(&mut self) -> ARGUMENT1_W {
        ARGUMENT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Argument1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [argument1](index.html) module"]
pub struct ARGUMENT1_SPEC;
impl crate::RegisterSpec for ARGUMENT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [argument1::R](R) reader structure"]
impl crate::Readable for ARGUMENT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [argument1::W](W) writer structure"]
impl crate::Writable for ARGUMENT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARGUMENT1 to value 0"]
impl crate::Resettable for ARGUMENT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
