#[doc = "Register `CLKCLR` writer"]
pub type W = crate::W<CLKCLR_SPEC>;
#[doc = "USB Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCDI_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable clock"]
    CONST_1 = 1,
}
impl From<USBCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: USBCDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCDI` writer - USB Clock Disable"]
pub type USBCDI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USBCDI_AW>;
impl<'a, REG, const O: u8> USBCDI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USBCDI_AW::CONST_0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USBCDI_AW::CONST_1)
    }
}
#[doc = "MMC Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCCDI_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable clock"]
    CONST_1 = 1,
}
impl From<MMCCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCCDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCDI` writer - MMC Clock Disable"]
pub type MMCCDI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MMCCDI_AW>;
impl<'a, REG, const O: u8> MMCCDI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCDI_AW::CONST_0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCDI_AW::CONST_1)
    }
}
#[doc = "Ethernet Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0CDI_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable clock"]
    CONST_1 = 1,
}
impl From<ETH0CDI_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0CDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CDI` writer - Ethernet Clock Disable"]
pub type ETH0CDI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ETH0CDI_AW>;
impl<'a, REG, const O: u8> ETH0CDI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CDI_AW::CONST_0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CDI_AW::CONST_1)
    }
}
#[doc = "CCU Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCDI_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable clock"]
    CONST_1 = 1,
}
impl From<CCUCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: CCUCDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCDI` writer - CCU Clock Disable"]
pub type CCUCDI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCUCDI_AW>;
impl<'a, REG, const O: u8> CCUCDI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCDI_AW::CONST_0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCDI_AW::CONST_1)
    }
}
#[doc = "WDT Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCDI_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable clock"]
    CONST_1 = 1,
}
impl From<WDTCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTCDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCDI` writer - WDT Clock Disable"]
pub type WDTCDI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDTCDI_AW>;
impl<'a, REG, const O: u8> WDTCDI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCDI_AW::CONST_0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCDI_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usbcdi(&mut self) -> USBCDI_W<CLKCLR_SPEC, 0> {
        USBCDI_W::new(self)
    }
    #[doc = "Bit 1 - MMC Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mmccdi(&mut self) -> MMCCDI_W<CLKCLR_SPEC, 1> {
        MMCCDI_W::new(self)
    }
    #[doc = "Bit 2 - Ethernet Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eth0cdi(&mut self) -> ETH0CDI_W<CLKCLR_SPEC, 2> {
        ETH0CDI_W::new(self)
    }
    #[doc = "Bit 4 - CCU Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ccucdi(&mut self) -> CCUCDI_W<CLKCLR_SPEC, 4> {
        CCUCDI_W::new(self)
    }
    #[doc = "Bit 5 - WDT Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcdi(&mut self) -> WDTCDI_W<CLKCLR_SPEC, 5> {
        WDTCDI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CLK Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCLR_SPEC;
impl crate::RegisterSpec for CLKCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clkclr::W`](W) writer structure"]
impl crate::Writable for CLKCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCLR to value 0"]
impl crate::Resettable for CLKCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
