#[doc = "Reader of register CEFLAG"]
pub type R = crate::R<u32, super::CEFLAG>;
#[doc = "Writer for register CEFLAG"]
pub type W = crate::W<u32, super::CEFLAG>;
#[doc = "Register CEFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::CEFLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel Event for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV0_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV0_A> for bool {
    #[inline(always)]
    fn from(variant: CEV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEV0`"]
pub type CEV0_R = crate::R<bool, CEV0_A>;
impl CEV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEV0_A {
        match self.bits {
            false => CEV0_A::VALUE1,
            true => CEV0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV0_A::VALUE2
    }
}
#[doc = "Write proxy for field `CEV0`"]
pub struct CEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV0_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV0_A::VALUE2)
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
#[doc = "Channel Event for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV1_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV1_A> for bool {
    #[inline(always)]
    fn from(variant: CEV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEV1`"]
pub type CEV1_R = crate::R<bool, CEV1_A>;
impl CEV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEV1_A {
        match self.bits {
            false => CEV1_A::VALUE1,
            true => CEV1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV1_A::VALUE2
    }
}
#[doc = "Write proxy for field `CEV1`"]
pub struct CEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV1_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV1_A::VALUE2)
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
#[doc = "Channel Event for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV2_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV2_A> for bool {
    #[inline(always)]
    fn from(variant: CEV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEV2`"]
pub type CEV2_R = crate::R<bool, CEV2_A>;
impl CEV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEV2_A {
        match self.bits {
            false => CEV2_A::VALUE1,
            true => CEV2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV2_A::VALUE2
    }
}
#[doc = "Write proxy for field `CEV2`"]
pub struct CEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV2_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV2_A::VALUE2)
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
#[doc = "Channel Event for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV3_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV3_A> for bool {
    #[inline(always)]
    fn from(variant: CEV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEV3`"]
pub type CEV3_R = crate::R<bool, CEV3_A>;
impl CEV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEV3_A {
        match self.bits {
            false => CEV3_A::VALUE1,
            true => CEV3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV3_A::VALUE2
    }
}
#[doc = "Write proxy for field `CEV3`"]
pub struct CEV3_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV3_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV3_A::VALUE2)
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
#[doc = "Channel Event for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV4_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV4_A> for bool {
    #[inline(always)]
    fn from(variant: CEV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEV4`"]
pub type CEV4_R = crate::R<bool, CEV4_A>;
impl CEV4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEV4_A {
        match self.bits {
            false => CEV4_A::VALUE1,
            true => CEV4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV4_A::VALUE2
    }
}
#[doc = "Write proxy for field `CEV4`"]
pub struct CEV4_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV4_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV4_A::VALUE2)
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
#[doc = "Channel Event for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV5_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV5_A> for bool {
    #[inline(always)]
    fn from(variant: CEV5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEV5`"]
pub type CEV5_R = crate::R<bool, CEV5_A>;
impl CEV5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEV5_A {
        match self.bits {
            false => CEV5_A::VALUE1,
            true => CEV5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV5_A::VALUE2
    }
}
#[doc = "Write proxy for field `CEV5`"]
pub struct CEV5_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV5_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV5_A::VALUE2)
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
#[doc = "Channel Event for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV6_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV6_A> for bool {
    #[inline(always)]
    fn from(variant: CEV6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEV6`"]
pub type CEV6_R = crate::R<bool, CEV6_A>;
impl CEV6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEV6_A {
        match self.bits {
            false => CEV6_A::VALUE1,
            true => CEV6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV6_A::VALUE2
    }
}
#[doc = "Write proxy for field `CEV6`"]
pub struct CEV6_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV6_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV6_A::VALUE2)
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
#[doc = "Channel Event for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV7_A {
    #[doc = "0: No channel event"]
    VALUE1 = 0,
    #[doc = "1: A channel event has occurred"]
    VALUE2 = 1,
}
impl From<CEV7_A> for bool {
    #[inline(always)]
    fn from(variant: CEV7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEV7`"]
pub type CEV7_R = crate::R<bool, CEV7_A>;
impl CEV7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEV7_A {
        match self.bits {
            false => CEV7_A::VALUE1,
            true => CEV7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV7_A::VALUE2
    }
}
#[doc = "Write proxy for field `CEV7`"]
pub struct CEV7_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV7_A::VALUE1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV7_A::VALUE2)
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
    #[doc = "Bit 0 - Channel Event for Channel 0"]
    #[inline(always)]
    pub fn cev0(&self) -> CEV0_R {
        CEV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Event for Channel 1"]
    #[inline(always)]
    pub fn cev1(&self) -> CEV1_R {
        CEV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel Event for Channel 2"]
    #[inline(always)]
    pub fn cev2(&self) -> CEV2_R {
        CEV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel Event for Channel 3"]
    #[inline(always)]
    pub fn cev3(&self) -> CEV3_R {
        CEV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel Event for Channel 4"]
    #[inline(always)]
    pub fn cev4(&self) -> CEV4_R {
        CEV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel Event for Channel 5"]
    #[inline(always)]
    pub fn cev5(&self) -> CEV5_R {
        CEV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel Event for Channel 6"]
    #[inline(always)]
    pub fn cev6(&self) -> CEV6_R {
        CEV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel Event for Channel 7"]
    #[inline(always)]
    pub fn cev7(&self) -> CEV7_R {
        CEV7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Event for Channel 0"]
    #[inline(always)]
    pub fn cev0(&mut self) -> CEV0_W {
        CEV0_W { w: self }
    }
    #[doc = "Bit 1 - Channel Event for Channel 1"]
    #[inline(always)]
    pub fn cev1(&mut self) -> CEV1_W {
        CEV1_W { w: self }
    }
    #[doc = "Bit 2 - Channel Event for Channel 2"]
    #[inline(always)]
    pub fn cev2(&mut self) -> CEV2_W {
        CEV2_W { w: self }
    }
    #[doc = "Bit 3 - Channel Event for Channel 3"]
    #[inline(always)]
    pub fn cev3(&mut self) -> CEV3_W {
        CEV3_W { w: self }
    }
    #[doc = "Bit 4 - Channel Event for Channel 4"]
    #[inline(always)]
    pub fn cev4(&mut self) -> CEV4_W {
        CEV4_W { w: self }
    }
    #[doc = "Bit 5 - Channel Event for Channel 5"]
    #[inline(always)]
    pub fn cev5(&mut self) -> CEV5_W {
        CEV5_W { w: self }
    }
    #[doc = "Bit 6 - Channel Event for Channel 6"]
    #[inline(always)]
    pub fn cev6(&mut self) -> CEV6_W {
        CEV6_W { w: self }
    }
    #[doc = "Bit 7 - Channel Event for Channel 7"]
    #[inline(always)]
    pub fn cev7(&mut self) -> CEV7_W {
        CEV7_W { w: self }
    }
}
