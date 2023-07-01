#[doc = "Register `HDCLR` writer"]
pub struct W(crate::W<HDCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDCLR_SPEC>;
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
impl From<crate::W<HDCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wake-up Pin Event Positive Edge Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPEV_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear wake-up event"]
    CONST_1 = 1,
}
impl From<EPEV_AW> for bool {
    #[inline(always)]
    fn from(variant: EPEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEV` writer - Wake-up Pin Event Positive Edge Clear"]
pub type EPEV_W<'a, const O: u8> = crate::BitWriter<'a, HDCLR_SPEC, O, EPEV_AW>;
impl<'a, const O: u8> EPEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(EPEV_AW::CONST_0)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(EPEV_AW::CONST_1)
    }
}
#[doc = "Wake-up Pin Event Negative Edge Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENEV_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear wake-up event"]
    CONST_1 = 1,
}
impl From<ENEV_AW> for bool {
    #[inline(always)]
    fn from(variant: ENEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENEV` writer - Wake-up Pin Event Negative Edge Clear"]
pub type ENEV_W<'a, const O: u8> = crate::BitWriter<'a, HDCLR_SPEC, O, ENEV_AW>;
impl<'a, const O: u8> ENEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ENEV_AW::CONST_0)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ENEV_AW::CONST_1)
    }
}
#[doc = "RTC Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEV_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear wake-up event"]
    CONST_1 = 1,
}
impl From<RTCEV_AW> for bool {
    #[inline(always)]
    fn from(variant: RTCEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEV` writer - RTC Event Clear"]
pub type RTCEV_W<'a, const O: u8> = crate::BitWriter<'a, HDCLR_SPEC, O, RTCEV_AW>;
impl<'a, const O: u8> RTCEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTCEV_AW::CONST_0)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTCEV_AW::CONST_1)
    }
}
#[doc = "ULP WDG Alarm Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDG_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear watchdog alarm"]
    CONST_1 = 1,
}
impl From<ULPWDG_AW> for bool {
    #[inline(always)]
    fn from(variant: ULPWDG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDG` writer - ULP WDG Alarm Clear"]
pub type ULPWDG_W<'a, const O: u8> = crate::BitWriter<'a, HDCLR_SPEC, O, ULPWDG_AW>;
impl<'a, const O: u8> ULPWDG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ULPWDG_AW::CONST_0)
    }
    #[doc = "Clear watchdog alarm"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ULPWDG_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epev(&mut self) -> EPEV_W<0> {
        EPEV_W::new(self)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Clear"]
    #[inline(always)]
    #[must_use]
    pub fn enev(&mut self) -> ENEV_W<1> {
        ENEV_W::new(self)
    }
    #[doc = "Bit 2 - RTC Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtcev(&mut self) -> RTCEV_W<2> {
        RTCEV_W::new(self)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdg(&mut self) -> ULPWDG_W<3> {
        ULPWDG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernate Domain Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdclr](index.html) module"]
pub struct HDCLR_SPEC;
impl crate::RegisterSpec for HDCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hdclr::W](W) writer structure"]
impl crate::Writable for HDCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HDCLR to value 0"]
impl crate::Resettable for HDCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
