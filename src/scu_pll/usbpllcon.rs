#[doc = "Reader of register USBPLLCON"]
pub type R = crate::R<u32, super::USBPLLCON>;
#[doc = "Writer for register USBPLLCON"]
pub type W = crate::W<u32, super::USBPLLCON>;
#[doc = "Register USBPLLCON `reset()`'s with value 0x0001_0003"]
impl crate::ResetValue for super::USBPLLCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0003
    }
}
#[doc = "VCO Bypass\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOBYP_A {
    #[doc = "0: Normal operation, VCO is not bypassed"]
    CONST_0,
    #[doc = "1: Prescaler Mode, VCO is bypassed"]
    CONST_1,
}
impl From<VCOBYP_A> for bool {
    #[inline(always)]
    fn from(variant: VCOBYP_A) -> Self {
        match variant {
            VCOBYP_A::CONST_0 => false,
            VCOBYP_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `VCOBYP`"]
pub type VCOBYP_R = crate::R<bool, VCOBYP_A>;
impl VCOBYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOBYP_A {
        match self.bits {
            false => VCOBYP_A::CONST_0,
            true => VCOBYP_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VCOBYP_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VCOBYP_A::CONST_1
    }
}
#[doc = "Write proxy for field `VCOBYP`"]
pub struct VCOBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOBYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation, VCO is not bypassed"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VCOBYP_A::CONST_0)
    }
    #[doc = "Prescaler Mode, VCO is bypassed"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VCOBYP_A::CONST_1)
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
#[doc = "VCO Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOPWD_A {
    #[doc = "0: Normal behavior"]
    CONST_0,
    #[doc = "1: The VCO is put into a Power Saving Mode"]
    CONST_1,
}
impl From<VCOPWD_A> for bool {
    #[inline(always)]
    fn from(variant: VCOPWD_A) -> Self {
        match variant {
            VCOPWD_A::CONST_0 => false,
            VCOPWD_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `VCOPWD`"]
pub type VCOPWD_R = crate::R<bool, VCOPWD_A>;
impl VCOPWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOPWD_A {
        match self.bits {
            false => VCOPWD_A::CONST_0,
            true => VCOPWD_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VCOPWD_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VCOPWD_A::CONST_1
    }
}
#[doc = "Write proxy for field `VCOPWD`"]
pub struct VCOPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOPWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOPWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VCOPWD_A::CONST_0)
    }
    #[doc = "The VCO is put into a Power Saving Mode"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VCOPWD_A::CONST_1)
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
#[doc = "VCO Trim Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOTR_A {
    #[doc = "0: VCO bandwidth is operating in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    CONST_0,
    #[doc = "1: VCO bandwidth is operating in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    CONST_1,
}
impl From<VCOTR_A> for bool {
    #[inline(always)]
    fn from(variant: VCOTR_A) -> Self {
        match variant {
            VCOTR_A::CONST_0 => false,
            VCOTR_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `VCOTR`"]
pub type VCOTR_R = crate::R<bool, VCOTR_A>;
impl VCOTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOTR_A {
        match self.bits {
            false => VCOTR_A::CONST_0,
            true => VCOTR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VCOTR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VCOTR_A::CONST_1
    }
}
#[doc = "Write proxy for field `VCOTR`"]
pub struct VCOTR_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VCO bandwidth is operating in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VCOTR_A::CONST_0)
    }
    #[doc = "VCO bandwidth is operating in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VCOTR_A::CONST_1)
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
#[doc = "Disconnect Oscillator from VCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINDIS_A {
    #[doc = "0: Connect oscillator to the VCO part"]
    CONST_0,
    #[doc = "1: Disconnect oscillator from the VCO part."]
    CONST_1,
}
impl From<FINDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FINDIS_A) -> Self {
        match variant {
            FINDIS_A::CONST_0 => false,
            FINDIS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `FINDIS`"]
pub type FINDIS_R = crate::R<bool, FINDIS_A>;
impl FINDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINDIS_A {
        match self.bits {
            false => FINDIS_A::CONST_0,
            true => FINDIS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == FINDIS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == FINDIS_A::CONST_1
    }
}
#[doc = "Write proxy for field `FINDIS`"]
pub struct FINDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FINDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Connect oscillator to the VCO part"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(FINDIS_A::CONST_0)
    }
    #[doc = "Disconnect oscillator from the VCO part."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(FINDIS_A::CONST_1)
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
#[doc = "Oscillator Disconnect Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCDISCDIS_A {
    #[doc = "0: In case of a PLL loss-of-lock bit FINDIS is set"]
    CONST_0,
    #[doc = "1: In case of a PLL loss-of-lock bit FINDIS is cleared"]
    CONST_1,
}
impl From<OSCDISCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OSCDISCDIS_A) -> Self {
        match variant {
            OSCDISCDIS_A::CONST_0 => false,
            OSCDISCDIS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `OSCDISCDIS`"]
pub type OSCDISCDIS_R = crate::R<bool, OSCDISCDIS_A>;
impl OSCDISCDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCDISCDIS_A {
        match self.bits {
            false => OSCDISCDIS_A::CONST_0,
            true => OSCDISCDIS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == OSCDISCDIS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == OSCDISCDIS_A::CONST_1
    }
}
#[doc = "Write proxy for field `OSCDISCDIS`"]
pub struct OSCDISCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCDISCDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCDISCDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is set"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(OSCDISCDIS_A::CONST_0)
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is cleared"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(OSCDISCDIS_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `NDIV`"]
pub type NDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NDIV`"]
pub struct NDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> NDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "PLL Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLPWD_A {
    #[doc = "0: Normal behavior"]
    CONST_0,
    #[doc = "1: The complete PLL block is put into a Power Saving Mode. Only the Bypass Mode is active if previously selected."]
    CONST_1,
}
impl From<PLLPWD_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPWD_A) -> Self {
        match variant {
            PLLPWD_A::CONST_0 => false,
            PLLPWD_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PLLPWD`"]
pub type PLLPWD_R = crate::R<bool, PLLPWD_A>;
impl PLLPWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLPWD_A {
        match self.bits {
            false => PLLPWD_A::CONST_0,
            true => PLLPWD_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PLLPWD_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PLLPWD_A::CONST_1
    }
}
#[doc = "Write proxy for field `PLLPWD`"]
pub struct PLLPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLPWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PLLPWD_A::CONST_0)
    }
    #[doc = "The complete PLL block is put into a Power Saving Mode. Only the Bypass Mode is active if previously selected."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PLLPWD_A::CONST_1)
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
#[doc = "Write proxy for field `RESLD`"]
pub struct RESLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RESLD_W<'a> {
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
#[doc = "Reader of field `PDIV`"]
pub type PDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDIV`"]
pub struct PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    pub fn vcobyp(&self) -> VCOBYP_R {
        VCOBYP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    pub fn vcopwd(&self) -> VCOPWD_R {
        VCOPWD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    pub fn vcotr(&self) -> VCOTR_R {
        VCOTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    pub fn findis(&self) -> FINDIS_R {
        FINDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    pub fn oscdiscdis(&self) -> OSCDISCDIS_R {
        OSCDISCDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    pub fn pllpwd(&self) -> PLLPWD_R {
        PLLPWD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    pub fn vcobyp(&mut self) -> VCOBYP_W {
        VCOBYP_W { w: self }
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    pub fn vcopwd(&mut self) -> VCOPWD_W {
        VCOPWD_W { w: self }
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    pub fn vcotr(&mut self) -> VCOTR_W {
        VCOTR_W { w: self }
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    pub fn findis(&mut self) -> FINDIS_W {
        FINDIS_W { w: self }
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    pub fn oscdiscdis(&mut self) -> OSCDISCDIS_W {
        OSCDISCDIS_W { w: self }
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    pub fn ndiv(&mut self) -> NDIV_W {
        NDIV_W { w: self }
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    pub fn pllpwd(&mut self) -> PLLPWD_W {
        PLLPWD_W { w: self }
    }
    #[doc = "Bit 18 - Restart VCO Lock Detection"]
    #[inline(always)]
    pub fn resld(&mut self) -> RESLD_W {
        RESLD_W { w: self }
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W {
        PDIV_W { w: self }
    }
}
