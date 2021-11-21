#[doc = "Register `MITR` writer"]
pub struct W(crate::W<MITR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MITR_SPEC>;
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
impl From<crate::W<MITR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MITR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IT` writer - Interrupt Trigger"]
pub struct IT_W<'a> {
    w: &'a mut W,
}
impl<'a> IT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Trigger"]
    #[inline(always)]
    pub fn it(&mut self) -> IT_W {
        IT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Interrupt Trigger Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mitr](index.html) module"]
pub struct MITR_SPEC;
impl crate::RegisterSpec for MITR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mitr::W](W) writer structure"]
impl crate::Writable for MITR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MITR to value 0"]
impl crate::Resettable for MITR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
