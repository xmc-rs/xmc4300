#[doc = "Register `CLKCLR` writer"]
pub struct W(crate::W<CLKCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCLR_SPEC>;
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
impl From<crate::W<CLKCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct USBCDI_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBCDI_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBCDI_AW::CONST_0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBCDI_AW::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "MMC Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MMCCDI_W<'a> {
    w: &'a mut W,
}
impl<'a> MMCCDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMCCDI_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MMCCDI_AW::CONST_0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MMCCDI_AW::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Ethernet Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct ETH0CDI_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH0CDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH0CDI_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ETH0CDI_AW::CONST_0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ETH0CDI_AW::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "CCU Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct CCUCDI_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUCDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCUCDI_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCUCDI_AW::CONST_0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCUCDI_AW::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "WDT Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct WDTCDI_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTCDI_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WDTCDI_AW::CONST_0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WDTCDI_AW::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Disable"]
    #[inline(always)]
    pub fn usbcdi(&mut self) -> USBCDI_W {
        USBCDI_W { w: self }
    }
    #[doc = "Bit 1 - MMC Clock Disable"]
    #[inline(always)]
    pub fn mmccdi(&mut self) -> MMCCDI_W {
        MMCCDI_W { w: self }
    }
    #[doc = "Bit 2 - Ethernet Clock Disable"]
    #[inline(always)]
    pub fn eth0cdi(&mut self) -> ETH0CDI_W {
        ETH0CDI_W { w: self }
    }
    #[doc = "Bit 4 - CCU Clock Disable"]
    #[inline(always)]
    pub fn ccucdi(&mut self) -> CCUCDI_W {
        CCUCDI_W { w: self }
    }
    #[doc = "Bit 5 - WDT Clock Disable"]
    #[inline(always)]
    pub fn wdtcdi(&mut self) -> WDTCDI_W {
        WDTCDI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLK Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkclr](index.html) module"]
pub struct CLKCLR_SPEC;
impl crate::RegisterSpec for CLKCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clkclr::W](W) writer structure"]
impl crate::Writable for CLKCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKCLR to value 0"]
impl crate::Resettable for CLKCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
