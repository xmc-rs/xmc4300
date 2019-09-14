#[doc = "Reader of register OSCHPCTRL"]
pub type R = crate::R<u32, super::OSCHPCTRL>;
#[doc = "Writer for register OSCHPCTRL"]
pub type W = crate::W<u32, super::OSCHPCTRL>;
#[doc = "Register OSCHPCTRL `reset()`'s with value 0x3c"]
impl crate::ResetValue for super::OSCHPCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3c
    }
}
#[doc = "XTAL1 Data Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X1DEN_A {
    #[doc = "0: Bit X1D is not updated"]
    CONST_0,
    #[doc = "1: Bit X1D can be updated"]
    CONST_1,
}
impl From<X1DEN_A> for bool {
    #[inline(always)]
    fn from(variant: X1DEN_A) -> Self {
        match variant {
            X1DEN_A::CONST_0 => false,
            X1DEN_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `X1DEN`"]
pub type X1DEN_R = crate::R<bool, X1DEN_A>;
impl X1DEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X1DEN_A {
        match self.bits {
            false => X1DEN_A::CONST_0,
            true => X1DEN_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == X1DEN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == X1DEN_A::CONST_1
    }
}
#[doc = "Write proxy for field `X1DEN`"]
pub struct X1DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> X1DEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: X1DEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bit X1D is not updated"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(X1DEN_A::CONST_0)
    }
    #[doc = "Bit X1D can be updated"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(X1DEN_A::CONST_1)
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
#[doc = "Shaper Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHBY_A {
    #[doc = "0: The shaper is not bypassed"]
    CONST_0,
    #[doc = "1: The shaper is bypassed"]
    CONST_1,
}
impl From<SHBY_A> for bool {
    #[inline(always)]
    fn from(variant: SHBY_A) -> Self {
        match variant {
            SHBY_A::CONST_0 => false,
            SHBY_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `SHBY`"]
pub type SHBY_R = crate::R<bool, SHBY_A>;
impl SHBY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHBY_A {
        match self.bits {
            false => SHBY_A::CONST_0,
            true => SHBY_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SHBY_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SHBY_A::CONST_1
    }
}
#[doc = "Write proxy for field `SHBY`"]
pub struct SHBY_W<'a> {
    w: &'a mut W,
}
impl<'a> SHBY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHBY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The shaper is not bypassed"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SHBY_A::CONST_0)
    }
    #[doc = "The shaper is bypassed"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SHBY_A::CONST_1)
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
#[doc = "Oscillator Gain Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAINSEL_A {
    #[doc = "0: The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    CONST_00,
    #[doc = "1: The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    CONST_01,
    #[doc = "2: The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    CONST_10,
    #[doc = "3: The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    CONST_11,
}
impl From<GAINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINSEL_A) -> Self {
        match variant {
            GAINSEL_A::CONST_00 => 0,
            GAINSEL_A::CONST_01 => 1,
            GAINSEL_A::CONST_10 => 2,
            GAINSEL_A::CONST_11 => 3,
        }
    }
}
#[doc = "Reader of field `GAINSEL`"]
pub type GAINSEL_R = crate::R<u8, GAINSEL_A>;
impl GAINSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAINSEL_A {
        match self.bits {
            0 => GAINSEL_A::CONST_00,
            1 => GAINSEL_A::CONST_01,
            2 => GAINSEL_A::CONST_10,
            3 => GAINSEL_A::CONST_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == GAINSEL_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == GAINSEL_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == GAINSEL_A::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == GAINSEL_A::CONST_11
    }
}
#[doc = "Write proxy for field `GAINSEL`"]
pub struct GAINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAINSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(GAINSEL_A::CONST_00)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(GAINSEL_A::CONST_01)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(GAINSEL_A::CONST_10)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    #[inline(always)]
    pub fn const_11(self) -> &'a mut W {
        self.variant(GAINSEL_A::CONST_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Oscillator Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    CONST_00,
    #[doc = "1: OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    CONST_01,
    #[doc = "2: External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    CONST_10,
    #[doc = "3: OSC is disabled. The oscillator Power-Saving Mode is entered."]
    CONST_11,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::CONST_00 => 0,
            MODE_A::CONST_01 => 1,
            MODE_A::CONST_10 => 2,
            MODE_A::CONST_11 => 3,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::CONST_00,
            1 => MODE_A::CONST_01,
            2 => MODE_A::CONST_10,
            3 => MODE_A::CONST_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == MODE_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == MODE_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == MODE_A::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == MODE_A::CONST_11
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(MODE_A::CONST_00)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(MODE_A::CONST_01)
    }
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(MODE_A::CONST_10)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    #[inline(always)]
    pub fn const_11(self) -> &'a mut W {
        self.variant(MODE_A::CONST_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `OSCVAL`"]
pub type OSCVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSCVAL`"]
pub struct OSCVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    pub fn x1den(&self) -> X1DEN_R {
        X1DEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    pub fn shby(&self) -> SHBY_R {
        SHBY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline(always)]
    pub fn gainsel(&self) -> GAINSEL_R {
        GAINSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    pub fn oscval(&self) -> OSCVAL_R {
        OSCVAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    pub fn x1den(&mut self) -> X1DEN_W {
        X1DEN_W { w: self }
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    pub fn shby(&mut self) -> SHBY_W {
        SHBY_W { w: self }
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline(always)]
    pub fn gainsel(&mut self) -> GAINSEL_W {
        GAINSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    pub fn oscval(&mut self) -> OSCVAL_W {
        OSCVAL_W { w: self }
    }
}
