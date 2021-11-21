#[doc = "Register `STATION_ALIAS` reader"]
pub struct R(crate::R<STATION_ALIAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATION_ALIAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATION_ALIAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATION_ALIAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATION_ALIAS` writer"]
pub struct W(crate::W<STATION_ALIAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATION_ALIAS_SPEC>;
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
impl From<crate::W<STATION_ALIAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATION_ALIAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALIAS_ADDR` reader - Alias Address used for node addressing(FPxx commands)"]
pub struct ALIAS_ADDR_R(crate::FieldReader<u16, u16>);
impl ALIAS_ADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        ALIAS_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALIAS_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIAS_ADDR` writer - Alias Address used for node addressing(FPxx commands)"]
pub struct ALIAS_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIAS_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Alias Address used for node addressing(FPxx commands)"]
    #[inline(always)]
    pub fn alias_addr(&self) -> ALIAS_ADDR_R {
        ALIAS_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Alias Address used for node addressing(FPxx commands)"]
    #[inline(always)]
    pub fn alias_addr(&mut self) -> ALIAS_ADDR_W {
        ALIAS_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configured Station Alias\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [station_alias](index.html) module"]
pub struct STATION_ALIAS_SPEC;
impl crate::RegisterSpec for STATION_ALIAS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [station_alias::R](R) reader structure"]
impl crate::Readable for STATION_ALIAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [station_alias::W](W) writer structure"]
impl crate::Writable for STATION_ALIAS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATION_ALIAS to value 0"]
impl crate::Resettable for STATION_ALIAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
