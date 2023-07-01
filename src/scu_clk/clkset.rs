#[doc = "Register `CLKSET` writer"]
pub struct W(crate::W<CLKSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSET_SPEC>;
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
impl From<crate::W<CLKSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<USBCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: USBCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCEN` writer - USB Clock Enable"]
pub type USBCEN_W<'a, const O: u8> = crate::BitWriter<'a, CLKSET_SPEC, O, USBCEN_AW>;
impl<'a, const O: u8> USBCEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBCEN_AW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBCEN_AW::CONST_1)
    }
}
#[doc = "MMC Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCCEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<MMCCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCEN` writer - MMC Clock Enable"]
pub type MMCCEN_W<'a, const O: u8> = crate::BitWriter<'a, CLKSET_SPEC, O, MMCCEN_AW>;
impl<'a, const O: u8> MMCCEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MMCCEN_AW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MMCCEN_AW::CONST_1)
    }
}
#[doc = "Ethernet Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0CEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<ETH0CEN_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0CEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CEN` writer - Ethernet Clock Enable"]
pub type ETH0CEN_W<'a, const O: u8> = crate::BitWriter<'a, CLKSET_SPEC, O, ETH0CEN_AW>;
impl<'a, const O: u8> ETH0CEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ETH0CEN_AW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ETH0CEN_AW::CONST_1)
    }
}
#[doc = "CCU Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<CCUCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: CCUCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCEN` writer - CCU Clock Enable"]
pub type CCUCEN_W<'a, const O: u8> = crate::BitWriter<'a, CLKSET_SPEC, O, CCUCEN_AW>;
impl<'a, const O: u8> CCUCEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCUCEN_AW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCUCEN_AW::CONST_1)
    }
}
#[doc = "WDT Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<WDTCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCEN` writer - WDT Clock Enable"]
pub type WDTCEN_W<'a, const O: u8> = crate::BitWriter<'a, CLKSET_SPEC, O, WDTCEN_AW>;
impl<'a, const O: u8> WDTCEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WDTCEN_AW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WDTCEN_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbcen(&mut self) -> USBCEN_W<0> {
        USBCEN_W::new(self)
    }
    #[doc = "Bit 1 - MMC Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mmccen(&mut self) -> MMCCEN_W<1> {
        MMCCEN_W::new(self)
    }
    #[doc = "Bit 2 - Ethernet Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eth0cen(&mut self) -> ETH0CEN_W<2> {
        ETH0CEN_W::new(self)
    }
    #[doc = "Bit 4 - CCU Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccucen(&mut self) -> CCUCEN_W<4> {
        CCUCEN_W::new(self)
    }
    #[doc = "Bit 5 - WDT Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcen(&mut self) -> WDTCEN_W<5> {
        WDTCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLK Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkset](index.html) module"]
pub struct CLKSET_SPEC;
impl crate::RegisterSpec for CLKSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clkset::W](W) writer structure"]
impl crate::Writable for CLKSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSET to value 0"]
impl crate::Resettable for CLKSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
