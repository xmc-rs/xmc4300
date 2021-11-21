#[doc = "Register `WD_DIVIDE` reader"]
pub struct R(crate::R<WD_DIVIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WD_DIVIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WD_DIVIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WD_DIVIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WD_DIVIDE` writer"]
pub struct W(crate::W<WD_DIVIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WD_DIVIDE_SPEC>;
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
impl From<crate::W<WD_DIVIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WD_DIVIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WD_DIV` reader - Watchdog divider"]
pub struct WD_DIV_R(crate::FieldReader<u16, u16>);
impl WD_DIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        WD_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WD_DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WD_DIV` writer - Watchdog divider"]
pub struct WD_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Watchdog divider"]
    #[inline(always)]
    pub fn wd_div(&self) -> WD_DIV_R {
        WD_DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog divider"]
    #[inline(always)]
    pub fn wd_div(&mut self) -> WD_DIV_W {
        WD_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wd_divide](index.html) module"]
pub struct WD_DIVIDE_SPEC;
impl crate::RegisterSpec for WD_DIVIDE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wd_divide::R](R) reader structure"]
impl crate::Readable for WD_DIVIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wd_divide::W](W) writer structure"]
impl crate::Writable for WD_DIVIDE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WD_DIVIDE to value 0x09c2"]
impl crate::Resettable for WD_DIVIDE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x09c2
    }
}
