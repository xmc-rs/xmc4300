#[doc = "Register `CGATCLR2` writer"]
pub struct W(crate::W<CGATCLR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGATCLR2_SPEC>;
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
impl From<crate::W<CGATCLR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGATCLR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WDT Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<WDT_AW> for bool {
    #[inline(always)]
    fn from(variant: WDT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` writer - WDT Gating Clear"]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGATCLR2_SPEC, WDT_AW, O>;
impl<'a, const O: u8> WDT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WDT_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WDT_AW::CONST_1)
    }
}
#[doc = "ETH0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<ETH0_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0` writer - ETH0 Gating Clear"]
pub type ETH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGATCLR2_SPEC, ETH0_AW, O>;
impl<'a, const O: u8> ETH0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ETH0_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ETH0_AW::CONST_1)
    }
}
#[doc = "DMA0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<DMA0_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0` writer - DMA0 Gating Clear"]
pub type DMA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGATCLR2_SPEC, DMA0_AW, O>;
impl<'a, const O: u8> DMA0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(DMA0_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(DMA0_AW::CONST_1)
    }
}
#[doc = "FCE Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCE_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<FCE_AW> for bool {
    #[inline(always)]
    fn from(variant: FCE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCE` writer - FCE Gating Clear"]
pub type FCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGATCLR2_SPEC, FCE_AW, O>;
impl<'a, const O: u8> FCE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(FCE_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(FCE_AW::CONST_1)
    }
}
#[doc = "USB Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<USB_AW> for bool {
    #[inline(always)]
    fn from(variant: USB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB` writer - USB Gating Clear"]
pub type USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGATCLR2_SPEC, USB_AW, O>;
impl<'a, const O: u8> USB_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USB_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USB_AW::CONST_1)
    }
}
#[doc = "ECAT0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<ECAT0_AW> for bool {
    #[inline(always)]
    fn from(variant: ECAT0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0` writer - ECAT0 Gating Clear"]
pub type ECAT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGATCLR2_SPEC, ECAT0_AW, O>;
impl<'a, const O: u8> ECAT0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ECAT0_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ECAT0_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<1> {
        WDT_W::new(self)
    }
    #[doc = "Bit 2 - ETH0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eth0(&mut self) -> ETH0_W<2> {
        ETH0_W::new(self)
    }
    #[doc = "Bit 4 - DMA0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dma0(&mut self) -> DMA0_W<4> {
        DMA0_W::new(self)
    }
    #[doc = "Bit 6 - FCE Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fce(&mut self) -> FCE_W<6> {
        FCE_W::new(self)
    }
    #[doc = "Bit 7 - USB Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<7> {
        USB_W::new(self)
    }
    #[doc = "Bit 10 - ECAT0 Gating Clear"]
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
#[doc = "Peripheral 2 Clock Gating Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatclr2](index.html) module"]
pub struct CGATCLR2_SPEC;
impl crate::RegisterSpec for CGATCLR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cgatclr2::W](W) writer structure"]
impl crate::Writable for CGATCLR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGATCLR2 to value 0"]
impl crate::Resettable for CGATCLR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
