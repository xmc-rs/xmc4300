#[doc = "Register `HDCLR` writer"]
pub type W = crate::W<HDCLR_SPEC>;
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
pub type EPEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EPEV_AW>;
impl<'a, REG, const O: u8> EPEV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(EPEV_AW::CONST_0)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
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
pub type ENEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENEV_AW>;
impl<'a, REG, const O: u8> ENEV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ENEV_AW::CONST_0)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
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
pub type RTCEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTCEV_AW>;
impl<'a, REG, const O: u8> RTCEV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEV_AW::CONST_0)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
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
pub type ULPWDG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ULPWDG_AW>;
impl<'a, REG, const O: u8> ULPWDG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDG_AW::CONST_0)
    }
    #[doc = "Clear watchdog alarm"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDG_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epev(&mut self) -> EPEV_W<HDCLR_SPEC, 0> {
        EPEV_W::new(self)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Clear"]
    #[inline(always)]
    #[must_use]
    pub fn enev(&mut self) -> ENEV_W<HDCLR_SPEC, 1> {
        ENEV_W::new(self)
    }
    #[doc = "Bit 2 - RTC Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtcev(&mut self) -> RTCEV_W<HDCLR_SPEC, 2> {
        RTCEV_W::new(self)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdg(&mut self) -> ULPWDG_W<HDCLR_SPEC, 3> {
        ULPWDG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hibernate Domain Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDCLR_SPEC;
impl crate::RegisterSpec for HDCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hdclr::W`](W) writer structure"]
impl crate::Writable for HDCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HDCLR to value 0"]
impl crate::Resettable for HDCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
