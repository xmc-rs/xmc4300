#[doc = "Reader of register DSLEEPCR"]
pub type R = crate::R<u32, super::DSLEEPCR>;
#[doc = "Writer for register DSLEEPCR"]
pub type W = crate::W<u32, super::DSLEEPCR>;
#[doc = "Register DSLEEPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSLEEPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSSEL_A {
    #[doc = "0: fOFI clock"]
    CONST_0,
    #[doc = "1: fPLL clock"]
    CONST_1,
}
impl From<SYSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSSEL_A) -> Self {
        match variant {
            SYSSEL_A::CONST_0 => false,
            SYSSEL_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `SYSSEL`"]
pub type SYSSEL_R = crate::R<bool, SYSSEL_A>;
impl SYSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSSEL_A {
        match self.bits {
            false => SYSSEL_A::CONST_0,
            true => SYSSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SYSSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SYSSEL_A::CONST_1
    }
}
#[doc = "Write proxy for field `SYSSEL`"]
pub struct SYSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SYSSEL_A::CONST_0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SYSSEL_A::CONST_1)
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
#[doc = "Flash Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPDN_A {
    #[doc = "1: Flash power down module"]
    CONST_1,
    #[doc = "0: No effect"]
    CONST_0,
}
impl From<FPDN_A> for bool {
    #[inline(always)]
    fn from(variant: FPDN_A) -> Self {
        match variant {
            FPDN_A::CONST_1 => true,
            FPDN_A::CONST_0 => false,
        }
    }
}
#[doc = "Reader of field `FPDN`"]
pub type FPDN_R = crate::R<bool, FPDN_A>;
impl FPDN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPDN_A {
        match self.bits {
            true => FPDN_A::CONST_1,
            false => FPDN_A::CONST_0,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == FPDN_A::CONST_1
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == FPDN_A::CONST_0
    }
}
#[doc = "Write proxy for field `FPDN`"]
pub struct FPDN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPDN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash power down module"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(FPDN_A::CONST_1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(FPDN_A::CONST_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "PLL Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLPDN_A {
    #[doc = "1: Switch off main PLL"]
    CONST_1,
    #[doc = "0: No effect"]
    CONST_0,
}
impl From<PLLPDN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPDN_A) -> Self {
        match variant {
            PLLPDN_A::CONST_1 => true,
            PLLPDN_A::CONST_0 => false,
        }
    }
}
#[doc = "Reader of field `PLLPDN`"]
pub type PLLPDN_R = crate::R<bool, PLLPDN_A>;
impl PLLPDN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLPDN_A {
        match self.bits {
            true => PLLPDN_A::CONST_1,
            false => PLLPDN_A::CONST_0,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PLLPDN_A::CONST_1
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PLLPDN_A::CONST_0
    }
}
#[doc = "Write proxy for field `PLLPDN`"]
pub struct PLLPDN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPDN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLPDN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Switch off main PLL"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PLLPDN_A::CONST_1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PLLPDN_A::CONST_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "VCO Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOPDN_A {
    #[doc = "1: Switch off VCO of main PLL"]
    CONST_1,
    #[doc = "0: No effect"]
    CONST_0,
}
impl From<VCOPDN_A> for bool {
    #[inline(always)]
    fn from(variant: VCOPDN_A) -> Self {
        match variant {
            VCOPDN_A::CONST_1 => true,
            VCOPDN_A::CONST_0 => false,
        }
    }
}
#[doc = "Reader of field `VCOPDN`"]
pub type VCOPDN_R = crate::R<bool, VCOPDN_A>;
impl VCOPDN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOPDN_A {
        match self.bits {
            true => VCOPDN_A::CONST_1,
            false => VCOPDN_A::CONST_0,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VCOPDN_A::CONST_1
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VCOPDN_A::CONST_0
    }
}
#[doc = "Write proxy for field `VCOPDN`"]
pub struct VCOPDN_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOPDN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOPDN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Switch off VCO of main PLL"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VCOPDN_A::CONST_1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VCOPDN_A::CONST_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "USB Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCR_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<USBCR_A> for bool {
    #[inline(always)]
    fn from(variant: USBCR_A) -> Self {
        match variant {
            USBCR_A::CONST_0 => false,
            USBCR_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `USBCR`"]
pub type USBCR_R = crate::R<bool, USBCR_A>;
impl USBCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBCR_A {
        match self.bits {
            false => USBCR_A::CONST_0,
            true => USBCR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == USBCR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == USBCR_A::CONST_1
    }
}
#[doc = "Write proxy for field `USBCR`"]
pub struct USBCR_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBCR_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "MMC Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCCR_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<MMCCR_A> for bool {
    #[inline(always)]
    fn from(variant: MMCCR_A) -> Self {
        match variant {
            MMCCR_A::CONST_0 => false,
            MMCCR_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MMCCR`"]
pub type MMCCR_R = crate::R<bool, MMCCR_A>;
impl MMCCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMCCR_A {
        match self.bits {
            false => MMCCR_A::CONST_0,
            true => MMCCR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == MMCCR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MMCCR_A::CONST_1
    }
}
#[doc = "Write proxy for field `MMCCR`"]
pub struct MMCCR_W<'a> {
    w: &'a mut W,
}
impl<'a> MMCCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMCCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MMCCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MMCCR_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Ethernet Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0CR_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<ETH0CR_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0CR_A) -> Self {
        match variant {
            ETH0CR_A::CONST_0 => false,
            ETH0CR_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `ETH0CR`"]
pub type ETH0CR_R = crate::R<bool, ETH0CR_A>;
impl ETH0CR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETH0CR_A {
        match self.bits {
            false => ETH0CR_A::CONST_0,
            true => ETH0CR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ETH0CR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ETH0CR_A::CONST_1
    }
}
#[doc = "Write proxy for field `ETH0CR`"]
pub struct ETH0CR_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH0CR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH0CR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ETH0CR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ETH0CR_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "CCU Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUCR_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<CCUCR_A> for bool {
    #[inline(always)]
    fn from(variant: CCUCR_A) -> Self {
        match variant {
            CCUCR_A::CONST_0 => false,
            CCUCR_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `CCUCR`"]
pub type CCUCR_R = crate::R<bool, CCUCR_A>;
impl CCUCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUCR_A {
        match self.bits {
            false => CCUCR_A::CONST_0,
            true => CCUCR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == CCUCR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CCUCR_A::CONST_1
    }
}
#[doc = "Write proxy for field `CCUCR`"]
pub struct CCUCR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCUCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCUCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCUCR_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "WDT Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCR_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<WDTCR_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCR_A) -> Self {
        match variant {
            WDTCR_A::CONST_0 => false,
            WDTCR_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `WDTCR`"]
pub type WDTCR_R = crate::R<bool, WDTCR_A>;
impl WDTCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTCR_A {
        match self.bits {
            false => WDTCR_A::CONST_0,
            true => WDTCR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == WDTCR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == WDTCR_A::CONST_1
    }
}
#[doc = "Write proxy for field `WDTCR`"]
pub struct WDTCR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WDTCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WDTCR_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    pub fn fpdn(&self) -> FPDN_R {
        FPDN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    pub fn pllpdn(&self) -> PLLPDN_R {
        PLLPDN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    pub fn vcopdn(&self) -> VCOPDN_R {
        VCOPDN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn usbcr(&self) -> USBCR_R {
        USBCR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn mmccr(&self) -> MMCCR_R {
        MMCCR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Ethernet Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn eth0cr(&self) -> ETH0CR_R {
        ETH0CR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CCU Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn ccucr(&self) -> CCUCR_R {
        CCUCR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - WDT Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn wdtcr(&self) -> WDTCR_R {
        WDTCR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&mut self) -> SYSSEL_W {
        SYSSEL_W { w: self }
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    pub fn fpdn(&mut self) -> FPDN_W {
        FPDN_W { w: self }
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    pub fn pllpdn(&mut self) -> PLLPDN_W {
        PLLPDN_W { w: self }
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    pub fn vcopdn(&mut self) -> VCOPDN_W {
        VCOPDN_W { w: self }
    }
    #[doc = "Bit 16 - USB Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn usbcr(&mut self) -> USBCR_W {
        USBCR_W { w: self }
    }
    #[doc = "Bit 17 - MMC Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn mmccr(&mut self) -> MMCCR_W {
        MMCCR_W { w: self }
    }
    #[doc = "Bit 18 - Ethernet Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn eth0cr(&mut self) -> ETH0CR_W {
        ETH0CR_W { w: self }
    }
    #[doc = "Bit 20 - CCU Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn ccucr(&mut self) -> CCUCR_W {
        CCUCR_W { w: self }
    }
    #[doc = "Bit 21 - WDT Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn wdtcr(&mut self) -> WDTCR_W {
        WDTCR_W { w: self }
    }
}
