#[doc = "Reader of register EMUXCTR"]
pub type R = crate::R<u32, super::EMUXCTR>;
#[doc = "Writer for register EMUXCTR"]
pub type W = crate::W<u32, super::EMUXCTR>;
#[doc = "Register EMUXCTR `reset()`'s with value 0"]
impl crate::ResetValue for super::EMUXCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EMUXSET`"]
pub type EMUXSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EMUXSET`"]
pub struct EMUXSET_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUXSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `EMUXACT`"]
pub type EMUXACT_R = crate::R<u8, u8>;
#[doc = "Reader of field `EMUXCH`"]
pub type EMUXCH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EMUXCH`"]
pub struct EMUXCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUXCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "External Multiplexer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMUXMODE_A {
    #[doc = "0: Software control (no hardware action)"]
    VALUE1 = 0,
    #[doc = "1: Steady mode (use EMUXSET value)"]
    VALUE2 = 1,
    #[doc = "2: Single-step mode"]
    VALUE3 = 2,
    #[doc = "3: Sequence mode"]
    VALUE4 = 3,
}
impl From<EMUXMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMUXMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EMUXMODE`"]
pub type EMUXMODE_R = crate::R<u8, EMUXMODE_A>;
impl EMUXMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMUXMODE_A {
        match self.bits {
            0 => EMUXMODE_A::VALUE1,
            1 => EMUXMODE_A::VALUE2,
            2 => EMUXMODE_A::VALUE3,
            3 => EMUXMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMUXMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMUXMODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EMUXMODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EMUXMODE_A::VALUE4
    }
}
#[doc = "Write proxy for field `EMUXMODE`"]
pub struct EMUXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUXMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMUXMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Software control (no hardware action)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMUXMODE_A::VALUE1)
    }
    #[doc = "Steady mode (use EMUXSET value)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMUXMODE_A::VALUE2)
    }
    #[doc = "Single-step mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EMUXMODE_A::VALUE3)
    }
    #[doc = "Sequence mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EMUXMODE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "External Multiplexer Coding Scheme\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMXCOD_A {
    #[doc = "0: Output the channel number in binary code"]
    VALUE1 = 0,
    #[doc = "1: Output the channel number in Gray code"]
    VALUE2 = 1,
}
impl From<EMXCOD_A> for bool {
    #[inline(always)]
    fn from(variant: EMXCOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMXCOD`"]
pub type EMXCOD_R = crate::R<bool, EMXCOD_A>;
impl EMXCOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMXCOD_A {
        match self.bits {
            false => EMXCOD_A::VALUE1,
            true => EMXCOD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMXCOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMXCOD_A::VALUE2
    }
}
#[doc = "Write proxy for field `EMXCOD`"]
pub struct EMXCOD_W<'a> {
    w: &'a mut W,
}
impl<'a> EMXCOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMXCOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output the channel number in binary code"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMXCOD_A::VALUE1)
    }
    #[doc = "Output the channel number in Gray code"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMXCOD_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "External Multiplexer Sample Time Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMXST_A {
    #[doc = "0: Use STCE whenever the setting changes"]
    VALUE1 = 0,
    #[doc = "1: Use STCE for each conversion of an external channel"]
    VALUE2 = 1,
}
impl From<EMXST_A> for bool {
    #[inline(always)]
    fn from(variant: EMXST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMXST`"]
pub type EMXST_R = crate::R<bool, EMXST_A>;
impl EMXST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMXST_A {
        match self.bits {
            false => EMXST_A::VALUE1,
            true => EMXST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMXST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMXST_A::VALUE2
    }
}
#[doc = "Write proxy for field `EMXST`"]
pub struct EMXST_W<'a> {
    w: &'a mut W,
}
impl<'a> EMXST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMXST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use STCE whenever the setting changes"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMXST_A::VALUE1)
    }
    #[doc = "Use STCE for each conversion of an external channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMXST_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "External Multiplexer Channel Selection Style\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMXCSS_A {
    #[doc = "0: Channel number: Bitfield EMUXCH selects an arbitrary channel"]
    VALUE1 = 0,
    #[doc = "1: Channel enable: Each bit of bitfield EMUXCH selects the associated channel for EMUX control"]
    VALUE2 = 1,
}
impl From<EMXCSS_A> for bool {
    #[inline(always)]
    fn from(variant: EMXCSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMXCSS`"]
pub type EMXCSS_R = crate::R<bool, EMXCSS_A>;
impl EMXCSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMXCSS_A {
        match self.bits {
            false => EMXCSS_A::VALUE1,
            true => EMXCSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMXCSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMXCSS_A::VALUE2
    }
}
#[doc = "Write Control for EMUX Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMXWC_AW {
    #[doc = "0: No write access to EMUX cfg."]
    VALUE1 = 0,
    #[doc = "1: Bitfields EMXMODE, EMXCOD, EMXST, EMXCSS can be written"]
    VALUE2 = 1,
}
impl From<EMXWC_AW> for bool {
    #[inline(always)]
    fn from(variant: EMXWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `EMXWC`"]
pub struct EMXWC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMXWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMXWC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write access to EMUX cfg."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMXWC_AW::VALUE1)
    }
    #[doc = "Bitfields EMXMODE, EMXCOD, EMXST, EMXCSS can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMXWC_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - External Multiplexer Start Selection"]
    #[inline(always)]
    pub fn emuxset(&self) -> EMUXSET_R {
        EMUXSET_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - External Multiplexer Actual Selection"]
    #[inline(always)]
    pub fn emuxact(&self) -> EMUXACT_R {
        EMUXACT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:25 - External Multiplexer Channel Select"]
    #[inline(always)]
    pub fn emuxch(&self) -> EMUXCH_R {
        EMUXCH_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:27 - External Multiplexer Mode"]
    #[inline(always)]
    pub fn emuxmode(&self) -> EMUXMODE_R {
        EMUXMODE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - External Multiplexer Coding Scheme"]
    #[inline(always)]
    pub fn emxcod(&self) -> EMXCOD_R {
        EMXCOD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - External Multiplexer Sample Time Control"]
    #[inline(always)]
    pub fn emxst(&self) -> EMXST_R {
        EMXST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - External Multiplexer Channel Selection Style"]
    #[inline(always)]
    pub fn emxcss(&self) -> EMXCSS_R {
        EMXCSS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Multiplexer Start Selection"]
    #[inline(always)]
    pub fn emuxset(&mut self) -> EMUXSET_W {
        EMUXSET_W { w: self }
    }
    #[doc = "Bits 16:25 - External Multiplexer Channel Select"]
    #[inline(always)]
    pub fn emuxch(&mut self) -> EMUXCH_W {
        EMUXCH_W { w: self }
    }
    #[doc = "Bits 26:27 - External Multiplexer Mode"]
    #[inline(always)]
    pub fn emuxmode(&mut self) -> EMUXMODE_W {
        EMUXMODE_W { w: self }
    }
    #[doc = "Bit 28 - External Multiplexer Coding Scheme"]
    #[inline(always)]
    pub fn emxcod(&mut self) -> EMXCOD_W {
        EMXCOD_W { w: self }
    }
    #[doc = "Bit 29 - External Multiplexer Sample Time Control"]
    #[inline(always)]
    pub fn emxst(&mut self) -> EMXST_W {
        EMXST_W { w: self }
    }
    #[doc = "Bit 31 - Write Control for EMUX Configuration"]
    #[inline(always)]
    pub fn emxwc(&mut self) -> EMXWC_W {
        EMXWC_W { w: self }
    }
}
