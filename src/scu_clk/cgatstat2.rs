#[doc = "Reader of register CGATSTAT2"]
pub type R = crate::R<u32, super::CGATSTAT2>;
#[doc = "WDT Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDT`"]
pub type WDT_R = crate::R<bool, WDT_A>;
impl WDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::CONST_0,
            true => WDT_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == WDT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == WDT_A::CONST_1
    }
}
#[doc = "ETH0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<ETH0_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETH0`"]
pub type ETH0_R = crate::R<bool, ETH0_A>;
impl ETH0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETH0_A {
        match self.bits {
            false => ETH0_A::CONST_0,
            true => ETH0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ETH0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ETH0_A::CONST_1
    }
}
#[doc = "DMA0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<DMA0_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA0`"]
pub type DMA0_R = crate::R<bool, DMA0_A>;
impl DMA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_A {
        match self.bits {
            false => DMA0_A::CONST_0,
            true => DMA0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == DMA0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == DMA0_A::CONST_1
    }
}
#[doc = "FCE Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCE_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<FCE_A> for bool {
    #[inline(always)]
    fn from(variant: FCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCE`"]
pub type FCE_R = crate::R<bool, FCE_A>;
impl FCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCE_A {
        match self.bits {
            false => FCE_A::CONST_0,
            true => FCE_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == FCE_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == FCE_A::CONST_1
    }
}
#[doc = "USB Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<USB_A> for bool {
    #[inline(always)]
    fn from(variant: USB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USB`"]
pub type USB_R = crate::R<bool, USB_A>;
impl USB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_A {
        match self.bits {
            false => USB_A::CONST_0,
            true => USB_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == USB_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == USB_A::CONST_1
    }
}
#[doc = "ECAT0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<ECAT0_A> for bool {
    #[inline(always)]
    fn from(variant: ECAT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECAT0`"]
pub type ECAT0_R = crate::R<bool, ECAT0_A>;
impl ECAT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECAT0_A {
        match self.bits {
            false => ECAT0_A::CONST_0,
            true => ECAT0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ECAT0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ECAT0_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 1 - WDT Gating Status"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ETH0 Gating Status"]
    #[inline(always)]
    pub fn eth0(&self) -> ETH0_R {
        ETH0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA0 Gating Status"]
    #[inline(always)]
    pub fn dma0(&self) -> DMA0_R {
        DMA0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FCE Gating Status"]
    #[inline(always)]
    pub fn fce(&self) -> FCE_R {
        FCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB Gating Status"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ECAT0 Gating Status"]
    #[inline(always)]
    pub fn ecat0(&self) -> ECAT0_R {
        ECAT0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
