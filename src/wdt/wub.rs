#[doc = "Register `WUB` reader"]
pub struct R(crate::R<WUB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUB` writer"]
pub struct W(crate::W<WUB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUB_SPEC>;
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
impl From<crate::W<WUB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUB` reader - Window Upper Bound"]
pub struct WUB_R(crate::FieldReader<u32, u32>);
impl WUB_R {
    pub(crate) fn new(bits: u32) -> Self {
        WUB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUB` writer - Window Upper Bound"]
pub struct WUB_W<'a> {
    w: &'a mut W,
}
impl<'a> WUB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Window Upper Bound"]
    #[inline(always)]
    pub fn wub(&self) -> WUB_R {
        WUB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Window Upper Bound"]
    #[inline(always)]
    pub fn wub(&mut self) -> WUB_W {
        WUB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Window Upper Bound Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wub](index.html) module"]
pub struct WUB_SPEC;
impl crate::RegisterSpec for WUB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wub::R](R) reader structure"]
impl crate::Readable for WUB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wub::W](W) writer structure"]
impl crate::Writable for WUB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUB to value 0xffff_ffff"]
impl crate::Resettable for WUB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
