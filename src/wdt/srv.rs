#[doc = "Register `SRV` writer"]
pub struct W(crate::W<SRV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRV_SPEC>;
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
impl From<crate::W<SRV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRV` writer - Service"]
pub struct SRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Service"]
    #[inline(always)]
    pub fn srv(&mut self) -> SRV_W {
        SRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Service Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srv](index.html) module"]
pub struct SRV_SPEC;
impl crate::RegisterSpec for SRV_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [srv::W](W) writer structure"]
impl crate::Writable for SRV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRV to value 0"]
impl crate::Resettable for SRV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
