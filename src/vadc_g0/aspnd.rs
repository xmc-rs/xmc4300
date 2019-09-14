#[doc = "Reader of register ASPND"]
pub type R = crate::R<u32, super::ASPND>;
#[doc = "Writer for register ASPND"]
pub type W = crate::W<u32, super::ASPND>;
#[doc = "Register ASPND `reset()`'s with value 0"]
impl crate::ResetValue for super::ASPND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND0_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPND0_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND0_A) -> Self {
        match variant {
            CHPND0_A::VALUE1 => false,
            CHPND0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPND0`"]
pub type CHPND0_R = crate::R<bool, CHPND0_A>;
impl CHPND0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND0_A {
        match self.bits {
            false => CHPND0_A::VALUE1,
            true => CHPND0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND0_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPND0`"]
pub struct CHPND0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPND0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPND0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND0_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND0_A::VALUE2)
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
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND1_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPND1_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND1_A) -> Self {
        match variant {
            CHPND1_A::VALUE1 => false,
            CHPND1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPND1`"]
pub type CHPND1_R = crate::R<bool, CHPND1_A>;
impl CHPND1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND1_A {
        match self.bits {
            false => CHPND1_A::VALUE1,
            true => CHPND1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND1_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPND1`"]
pub struct CHPND1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPND1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPND1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND1_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND1_A::VALUE2)
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
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND2_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPND2_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND2_A) -> Self {
        match variant {
            CHPND2_A::VALUE1 => false,
            CHPND2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPND2`"]
pub type CHPND2_R = crate::R<bool, CHPND2_A>;
impl CHPND2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND2_A {
        match self.bits {
            false => CHPND2_A::VALUE1,
            true => CHPND2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND2_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPND2`"]
pub struct CHPND2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPND2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPND2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND2_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND2_A::VALUE2)
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
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND3_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPND3_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND3_A) -> Self {
        match variant {
            CHPND3_A::VALUE1 => false,
            CHPND3_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPND3`"]
pub type CHPND3_R = crate::R<bool, CHPND3_A>;
impl CHPND3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND3_A {
        match self.bits {
            false => CHPND3_A::VALUE1,
            true => CHPND3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND3_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPND3`"]
pub struct CHPND3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPND3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPND3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND3_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND3_A::VALUE2)
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
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND4_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPND4_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND4_A) -> Self {
        match variant {
            CHPND4_A::VALUE1 => false,
            CHPND4_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPND4`"]
pub type CHPND4_R = crate::R<bool, CHPND4_A>;
impl CHPND4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND4_A {
        match self.bits {
            false => CHPND4_A::VALUE1,
            true => CHPND4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND4_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPND4`"]
pub struct CHPND4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPND4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPND4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND4_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND4_A::VALUE2)
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
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND5_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPND5_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND5_A) -> Self {
        match variant {
            CHPND5_A::VALUE1 => false,
            CHPND5_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPND5`"]
pub type CHPND5_R = crate::R<bool, CHPND5_A>;
impl CHPND5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND5_A {
        match self.bits {
            false => CHPND5_A::VALUE1,
            true => CHPND5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND5_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPND5`"]
pub struct CHPND5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPND5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPND5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND5_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND5_A::VALUE2)
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
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND6_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPND6_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND6_A) -> Self {
        match variant {
            CHPND6_A::VALUE1 => false,
            CHPND6_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPND6`"]
pub type CHPND6_R = crate::R<bool, CHPND6_A>;
impl CHPND6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND6_A {
        match self.bits {
            false => CHPND6_A::VALUE1,
            true => CHPND6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND6_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPND6`"]
pub struct CHPND6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPND6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPND6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND6_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND6_A::VALUE2)
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
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND7_A {
    #[doc = "0: Ignore this channel"]
    VALUE1,
    #[doc = "1: Request conversion of this channel"]
    VALUE2,
}
impl From<CHPND7_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND7_A) -> Self {
        match variant {
            CHPND7_A::VALUE1 => false,
            CHPND7_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CHPND7`"]
pub type CHPND7_R = crate::R<bool, CHPND7_A>;
impl CHPND7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND7_A {
        match self.bits {
            false => CHPND7_A::VALUE1,
            true => CHPND7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND7_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHPND7`"]
pub struct CHPND7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPND7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPND7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND7_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND7_A::VALUE2)
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
    #[doc = "Bit 0 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd0(&self) -> CHPND0_R {
        CHPND0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd1(&self) -> CHPND1_R {
        CHPND1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd2(&self) -> CHPND2_R {
        CHPND2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd3(&self) -> CHPND3_R {
        CHPND3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd4(&self) -> CHPND4_R {
        CHPND4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd5(&self) -> CHPND5_R {
        CHPND5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd6(&self) -> CHPND6_R {
        CHPND6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd7(&self) -> CHPND7_R {
        CHPND7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd0(&mut self) -> CHPND0_W {
        CHPND0_W { w: self }
    }
    #[doc = "Bit 1 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd1(&mut self) -> CHPND1_W {
        CHPND1_W { w: self }
    }
    #[doc = "Bit 2 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd2(&mut self) -> CHPND2_W {
        CHPND2_W { w: self }
    }
    #[doc = "Bit 3 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd3(&mut self) -> CHPND3_W {
        CHPND3_W { w: self }
    }
    #[doc = "Bit 4 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd4(&mut self) -> CHPND4_W {
        CHPND4_W { w: self }
    }
    #[doc = "Bit 5 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd5(&mut self) -> CHPND5_W {
        CHPND5_W { w: self }
    }
    #[doc = "Bit 6 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd6(&mut self) -> CHPND6_W {
        CHPND6_W { w: self }
    }
    #[doc = "Bit 7 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd7(&mut self) -> CHPND7_W {
        CHPND7_W { w: self }
    }
}
