#[doc = "Writer for register CLKSET"]
pub type W = crate::W<u32, super::CLKSET>;
#[doc = "Register CLKSET `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCEN_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable"]
    CONST_1,
}
impl From<USBCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: USBCEN_AW) -> Self {
        match variant {
            USBCEN_AW::CONST_0 => false,
            USBCEN_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `USBCEN`"]
pub struct USBCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBCEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "MMC Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCCEN_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable"]
    CONST_1,
}
impl From<MMCCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCCEN_AW) -> Self {
        match variant {
            MMCCEN_AW::CONST_0 => false,
            MMCCEN_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `MMCCEN`"]
pub struct MMCCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MMCCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMCCEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Ethernet Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0CEN_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable"]
    CONST_1,
}
impl From<ETH0CEN_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0CEN_AW) -> Self {
        match variant {
            ETH0CEN_AW::CONST_0 => false,
            ETH0CEN_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `ETH0CEN`"]
pub struct ETH0CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH0CEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH0CEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "CCU Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUCEN_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable"]
    CONST_1,
}
impl From<CCUCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: CCUCEN_AW) -> Self {
        match variant {
            CCUCEN_AW::CONST_0 => false,
            CCUCEN_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `CCUCEN`"]
pub struct CCUCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCUCEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "WDT Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCEN_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable"]
    CONST_1,
}
impl From<WDTCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTCEN_AW) -> Self {
        match variant {
            WDTCEN_AW::CONST_0 => false,
            WDTCEN_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `WDTCEN`"]
pub struct WDTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTCEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Enable"]
    #[inline(always)]
    pub fn usbcen(&mut self) -> USBCEN_W {
        USBCEN_W { w: self }
    }
    #[doc = "Bit 1 - MMC Clock Enable"]
    #[inline(always)]
    pub fn mmccen(&mut self) -> MMCCEN_W {
        MMCCEN_W { w: self }
    }
    #[doc = "Bit 2 - Ethernet Clock Enable"]
    #[inline(always)]
    pub fn eth0cen(&mut self) -> ETH0CEN_W {
        ETH0CEN_W { w: self }
    }
    #[doc = "Bit 4 - CCU Clock Enable"]
    #[inline(always)]
    pub fn ccucen(&mut self) -> CCUCEN_W {
        CCUCEN_W { w: self }
    }
    #[doc = "Bit 5 - WDT Clock Enable"]
    #[inline(always)]
    pub fn wdtcen(&mut self) -> WDTCEN_W {
        WDTCEN_W { w: self }
    }
}
