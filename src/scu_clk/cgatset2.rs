#[doc = "Register `CGATSET2` writer"]
pub struct W(crate::W<CGATSET2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGATSET2_SPEC>;
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
impl From<crate::W<CGATSET2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGATSET2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WDT Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<WDT_AW> for bool {
    #[inline(always)]
    fn from(variant: WDT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` writer - WDT Gating Set"]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET2_SPEC, O, WDT_AW>;
impl<'a, const O: u8> WDT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WDT_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WDT_AW::CONST_1)
    }
}
#[doc = "ETH0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<ETH0_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0` writer - ETH0 Gating Set"]
pub type ETH0_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET2_SPEC, O, ETH0_AW>;
impl<'a, const O: u8> ETH0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ETH0_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ETH0_AW::CONST_1)
    }
}
#[doc = "DMA0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<DMA0_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0` writer - DMA0 Gating Set"]
pub type DMA0_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET2_SPEC, O, DMA0_AW>;
impl<'a, const O: u8> DMA0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(DMA0_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(DMA0_AW::CONST_1)
    }
}
#[doc = "FCE Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCE_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<FCE_AW> for bool {
    #[inline(always)]
    fn from(variant: FCE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCE` writer - FCE Gating Set"]
pub type FCE_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET2_SPEC, O, FCE_AW>;
impl<'a, const O: u8> FCE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(FCE_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(FCE_AW::CONST_1)
    }
}
#[doc = "USB Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<USB_AW> for bool {
    #[inline(always)]
    fn from(variant: USB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB` writer - USB Gating Set"]
pub type USB_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET2_SPEC, O, USB_AW>;
impl<'a, const O: u8> USB_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USB_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USB_AW::CONST_1)
    }
}
#[doc = "ECAT0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<ECAT0_AW> for bool {
    #[inline(always)]
    fn from(variant: ECAT0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0` writer - ECAT0 Gating Set"]
pub type ECAT0_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET2_SPEC, O, ECAT0_AW>;
impl<'a, const O: u8> ECAT0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ECAT0_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ECAT0_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<1> {
        WDT_W::new(self)
    }
    #[doc = "Bit 2 - ETH0 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn eth0(&mut self) -> ETH0_W<2> {
        ETH0_W::new(self)
    }
    #[doc = "Bit 4 - DMA0 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma0(&mut self) -> DMA0_W<4> {
        DMA0_W::new(self)
    }
    #[doc = "Bit 6 - FCE Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn fce(&mut self) -> FCE_W<6> {
        FCE_W::new(self)
    }
    #[doc = "Bit 7 - USB Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<7> {
        USB_W::new(self)
    }
    #[doc = "Bit 10 - ECAT0 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0(&mut self) -> ECAT0_W<10> {
        ECAT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral 2 Clock Gating Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatset2](index.html) module"]
pub struct CGATSET2_SPEC;
impl crate::RegisterSpec for CGATSET2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cgatset2::W](W) writer structure"]
impl crate::Writable for CGATSET2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGATSET2 to value 0"]
impl crate::Resettable for CGATSET2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
