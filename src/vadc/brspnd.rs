#[doc = "Reader of register BRSPND[%s]"]
pub type R = crate::R<u32, super::BRSPND>;
#[doc = "Writer for register BRSPND[%s]"]
pub type W = crate::W<u32, super::BRSPND>;
#[doc = "Register BRSPND[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::BRSPND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG0_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPNDG0_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG0_A) -> Self {
        match variant {
            CHPNDG0_A::VALUE1 => false,
            CHPNDG0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPNDG0`"]
pub type CHPNDG0_R = crate::R<bool, CHPNDG0_A>;
impl CHPNDG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG0_A {
        match self.bits {
            false => CHPNDG0_A::VALUE1,
            true => CHPNDG0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG0_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPNDG0`"]
pub struct CHPNDG0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPNDG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPNDG0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG0_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG0_A::VALUE2)
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
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG1_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPNDG1_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG1_A) -> Self {
        match variant {
            CHPNDG1_A::VALUE1 => false,
            CHPNDG1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPNDG1`"]
pub type CHPNDG1_R = crate::R<bool, CHPNDG1_A>;
impl CHPNDG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG1_A {
        match self.bits {
            false => CHPNDG1_A::VALUE1,
            true => CHPNDG1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG1_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPNDG1`"]
pub struct CHPNDG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPNDG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPNDG1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG1_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG1_A::VALUE2)
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
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG2_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPNDG2_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG2_A) -> Self {
        match variant {
            CHPNDG2_A::VALUE1 => false,
            CHPNDG2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPNDG2`"]
pub type CHPNDG2_R = crate::R<bool, CHPNDG2_A>;
impl CHPNDG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG2_A {
        match self.bits {
            false => CHPNDG2_A::VALUE1,
            true => CHPNDG2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG2_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPNDG2`"]
pub struct CHPNDG2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPNDG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPNDG2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG2_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG2_A::VALUE2)
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
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG3_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPNDG3_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG3_A) -> Self {
        match variant {
            CHPNDG3_A::VALUE1 => false,
            CHPNDG3_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPNDG3`"]
pub type CHPNDG3_R = crate::R<bool, CHPNDG3_A>;
impl CHPNDG3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG3_A {
        match self.bits {
            false => CHPNDG3_A::VALUE1,
            true => CHPNDG3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG3_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPNDG3`"]
pub struct CHPNDG3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPNDG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPNDG3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG3_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG3_A::VALUE2)
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
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG4_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPNDG4_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG4_A) -> Self {
        match variant {
            CHPNDG4_A::VALUE1 => false,
            CHPNDG4_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPNDG4`"]
pub type CHPNDG4_R = crate::R<bool, CHPNDG4_A>;
impl CHPNDG4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG4_A {
        match self.bits {
            false => CHPNDG4_A::VALUE1,
            true => CHPNDG4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG4_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPNDG4`"]
pub struct CHPNDG4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPNDG4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPNDG4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG4_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG4_A::VALUE2)
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
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG5_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPNDG5_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG5_A) -> Self {
        match variant {
            CHPNDG5_A::VALUE1 => false,
            CHPNDG5_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPNDG5`"]
pub type CHPNDG5_R = crate::R<bool, CHPNDG5_A>;
impl CHPNDG5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG5_A {
        match self.bits {
            false => CHPNDG5_A::VALUE1,
            true => CHPNDG5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG5_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPNDG5`"]
pub struct CHPNDG5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPNDG5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPNDG5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG5_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG5_A::VALUE2)
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
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG6_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPNDG6_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG6_A) -> Self {
        match variant {
            CHPNDG6_A::VALUE1 => false,
            CHPNDG6_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPNDG6`"]
pub type CHPNDG6_R = crate::R<bool, CHPNDG6_A>;
impl CHPNDG6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG6_A {
        match self.bits {
            false => CHPNDG6_A::VALUE1,
            true => CHPNDG6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG6_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPNDG6`"]
pub struct CHPNDG6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPNDG6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPNDG6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG6_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG6_A::VALUE2)
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
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG7_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPNDG7_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG7_A) -> Self {
        match variant {
            CHPNDG7_A::VALUE1 => false,
            CHPNDG7_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPNDG7`"]
pub type CHPNDG7_R = crate::R<bool, CHPNDG7_A>;
impl CHPNDG7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG7_A {
        match self.bits {
            false => CHPNDG7_A::VALUE1,
            true => CHPNDG7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG7_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPNDG7`"]
pub struct CHPNDG7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPNDG7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPNDG7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG7_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG7_A::VALUE2)
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
    #[doc = "Bit 0 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg0(&self) -> CHPNDG0_R {
        CHPNDG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg1(&self) -> CHPNDG1_R {
        CHPNDG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg2(&self) -> CHPNDG2_R {
        CHPNDG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg3(&self) -> CHPNDG3_R {
        CHPNDG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg4(&self) -> CHPNDG4_R {
        CHPNDG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg5(&self) -> CHPNDG5_R {
        CHPNDG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg6(&self) -> CHPNDG6_R {
        CHPNDG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg7(&self) -> CHPNDG7_R {
        CHPNDG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg0(&mut self) -> CHPNDG0_W {
        CHPNDG0_W { w: self }
    }
    #[doc = "Bit 1 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg1(&mut self) -> CHPNDG1_W {
        CHPNDG1_W { w: self }
    }
    #[doc = "Bit 2 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg2(&mut self) -> CHPNDG2_W {
        CHPNDG2_W { w: self }
    }
    #[doc = "Bit 3 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg3(&mut self) -> CHPNDG3_W {
        CHPNDG3_W { w: self }
    }
    #[doc = "Bit 4 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg4(&mut self) -> CHPNDG4_W {
        CHPNDG4_W { w: self }
    }
    #[doc = "Bit 5 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg5(&mut self) -> CHPNDG5_W {
        CHPNDG5_W { w: self }
    }
    #[doc = "Bit 6 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg6(&mut self) -> CHPNDG6_W {
        CHPNDG6_W { w: self }
    }
    #[doc = "Bit 7 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg7(&mut self) -> CHPNDG7_W {
        CHPNDG7_W { w: self }
    }
}
