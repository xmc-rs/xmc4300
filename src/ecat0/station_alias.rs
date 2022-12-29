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
pub type ALIAS_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ALIAS_ADDR` writer - Alias Address used for node addressing(FPxx commands)"]
pub type ALIAS_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, STATION_ALIAS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Alias Address used for node addressing(FPxx commands)"]
    #[inline(always)]
    pub fn alias_addr(&self) -> ALIAS_ADDR_R {
        ALIAS_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Alias Address used for node addressing(FPxx commands)"]
    #[inline(always)]
    #[must_use]
    pub fn alias_addr(&mut self) -> ALIAS_ADDR_W<0> {
        ALIAS_ADDR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATION_ALIAS to value 0"]
impl crate::Resettable for STATION_ALIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
