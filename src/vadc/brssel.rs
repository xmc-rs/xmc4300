#[doc = "Reader of register BRSSEL[%s]"]
pub type R = crate::R<u32, super::BRSSEL>;
#[doc = "Writer for register BRSSEL[%s]"]
pub type W = crate::W<u32, super::BRSSEL>;
#[doc = "Register BRSSEL[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::BRSSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG0_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2,
}
impl From<CHSELG0_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG0_A) -> Self {
        match variant {
            CHSELG0_A::VALUE1 => false,
            CHSELG0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHSELG0`"]
pub type CHSELG0_R = crate::R<bool, CHSELG0_A>;
impl CHSELG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG0_A {
        match self.bits {
            false => CHSELG0_A::VALUE1,
            true => CHSELG0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG0_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHSELG0`"]
pub struct CHSELG0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG0_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG0_A::VALUE2)
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
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG1_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2,
}
impl From<CHSELG1_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG1_A) -> Self {
        match variant {
            CHSELG1_A::VALUE1 => false,
            CHSELG1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHSELG1`"]
pub type CHSELG1_R = crate::R<bool, CHSELG1_A>;
impl CHSELG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG1_A {
        match self.bits {
            false => CHSELG1_A::VALUE1,
            true => CHSELG1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG1_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHSELG1`"]
pub struct CHSELG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG1_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG1_A::VALUE2)
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
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG2_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2,
}
impl From<CHSELG2_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG2_A) -> Self {
        match variant {
            CHSELG2_A::VALUE1 => false,
            CHSELG2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHSELG2`"]
pub type CHSELG2_R = crate::R<bool, CHSELG2_A>;
impl CHSELG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG2_A {
        match self.bits {
            false => CHSELG2_A::VALUE1,
            true => CHSELG2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG2_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHSELG2`"]
pub struct CHSELG2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG2_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG2_A::VALUE2)
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
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG3_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2,
}
impl From<CHSELG3_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG3_A) -> Self {
        match variant {
            CHSELG3_A::VALUE1 => false,
            CHSELG3_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHSELG3`"]
pub type CHSELG3_R = crate::R<bool, CHSELG3_A>;
impl CHSELG3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG3_A {
        match self.bits {
            false => CHSELG3_A::VALUE1,
            true => CHSELG3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG3_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHSELG3`"]
pub struct CHSELG3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG3_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG3_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG4_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2,
}
impl From<CHSELG4_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG4_A) -> Self {
        match variant {
            CHSELG4_A::VALUE1 => false,
            CHSELG4_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHSELG4`"]
pub type CHSELG4_R = crate::R<bool, CHSELG4_A>;
impl CHSELG4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG4_A {
        match self.bits {
            false => CHSELG4_A::VALUE1,
            true => CHSELG4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG4_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHSELG4`"]
pub struct CHSELG4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG4_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG4_A::VALUE2)
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
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG5_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2,
}
impl From<CHSELG5_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG5_A) -> Self {
        match variant {
            CHSELG5_A::VALUE1 => false,
            CHSELG5_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHSELG5`"]
pub type CHSELG5_R = crate::R<bool, CHSELG5_A>;
impl CHSELG5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG5_A {
        match self.bits {
            false => CHSELG5_A::VALUE1,
            true => CHSELG5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG5_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHSELG5`"]
pub struct CHSELG5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG5_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG5_A::VALUE2)
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
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG6_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2,
}
impl From<CHSELG6_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG6_A) -> Self {
        match variant {
            CHSELG6_A::VALUE1 => false,
            CHSELG6_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHSELG6`"]
pub type CHSELG6_R = crate::R<bool, CHSELG6_A>;
impl CHSELG6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG6_A {
        match self.bits {
            false => CHSELG6_A::VALUE1,
            true => CHSELG6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG6_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHSELG6`"]
pub struct CHSELG6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG6_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG6_A::VALUE2)
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
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG7_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2,
}
impl From<CHSELG7_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG7_A) -> Self {
        match variant {
            CHSELG7_A::VALUE1 => false,
            CHSELG7_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHSELG7`"]
pub type CHSELG7_R = crate::R<bool, CHSELG7_A>;
impl CHSELG7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG7_A {
        match self.bits {
            false => CHSELG7_A::VALUE1,
            true => CHSELG7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG7_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHSELG7`"]
pub struct CHSELG7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG7_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG7_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg0(&self) -> CHSELG0_R {
        CHSELG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg1(&self) -> CHSELG1_R {
        CHSELG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg2(&self) -> CHSELG2_R {
        CHSELG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg3(&self) -> CHSELG3_R {
        CHSELG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg4(&self) -> CHSELG4_R {
        CHSELG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg5(&self) -> CHSELG5_R {
        CHSELG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg6(&self) -> CHSELG6_R {
        CHSELG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg7(&self) -> CHSELG7_R {
        CHSELG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg0(&mut self) -> CHSELG0_W {
        CHSELG0_W { w: self }
    }
    #[doc = "Bit 1 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg1(&mut self) -> CHSELG1_W {
        CHSELG1_W { w: self }
    }
    #[doc = "Bit 2 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg2(&mut self) -> CHSELG2_W {
        CHSELG2_W { w: self }
    }
    #[doc = "Bit 3 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg3(&mut self) -> CHSELG3_W {
        CHSELG3_W { w: self }
    }
    #[doc = "Bit 4 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg4(&mut self) -> CHSELG4_W {
        CHSELG4_W { w: self }
    }
    #[doc = "Bit 5 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg5(&mut self) -> CHSELG5_W {
        CHSELG5_W { w: self }
    }
    #[doc = "Bit 6 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg6(&mut self) -> CHSELG6_W {
        CHSELG6_W { w: self }
    }
    #[doc = "Bit 7 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg7(&mut self) -> CHSELG7_W {
        CHSELG7_W { w: self }
    }
}
