#[doc = "Register `WD_TIME_PDATA` reader"]
pub struct R(crate::R<WD_TIME_PDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WD_TIME_PDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WD_TIME_PDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WD_TIME_PDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WD_TIME_PDATA` writer"]
pub struct W(crate::W<WD_TIME_PDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WD_TIME_PDATA_SPEC>;
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
impl From<crate::W<WD_TIME_PDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WD_TIME_PDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WD_TIME_PD` reader - Watchdog Time Process Data"]
pub type WD_TIME_PD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WD_TIME_PD` writer - Watchdog Time Process Data"]
pub type WD_TIME_PD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WD_TIME_PDATA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Watchdog Time Process Data"]
    #[inline(always)]
    pub fn wd_time_pd(&self) -> WD_TIME_PD_R {
        WD_TIME_PD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog Time Process Data"]
    #[inline(always)]
    #[must_use]
    pub fn wd_time_pd(&mut self) -> WD_TIME_PD_W<0> {
        WD_TIME_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Time Process Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wd_time_pdata](index.html) module"]
pub struct WD_TIME_PDATA_SPEC;
impl crate::RegisterSpec for WD_TIME_PDATA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wd_time_pdata::R](R) reader structure"]
impl crate::Readable for WD_TIME_PDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wd_time_pdata::W](W) writer structure"]
impl crate::Writable for WD_TIME_PDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WD_TIME_PDATA to value 0x03e8"]
impl crate::Resettable for WD_TIME_PDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x03e8;
}
