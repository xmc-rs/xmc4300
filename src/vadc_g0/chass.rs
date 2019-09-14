#[doc = "Reader of register CHASS"]
pub type R = crate::R<u32, super::CHASS>;
#[doc = "Writer for register CHASS"]
pub type W = crate::W<u32, super::CHASS>;
#[doc = "Register CHASS `reset()`'s with value 0"]
impl crate::ResetValue for super::CHASS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Assignment for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH0_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2,
}
impl From<ASSCH0_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH0_A) -> Self {
        match variant {
            ASSCH0_A::VALUE1 => false,
            ASSCH0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ASSCH0`"]
pub type ASSCH0_R = crate::R<bool, ASSCH0_A>;
impl ASSCH0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH0_A {
        match self.bits {
            false => ASSCH0_A::VALUE1,
            true => ASSCH0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH0_A::VALUE2
    }
}
#[doc = "Write proxy for field `ASSCH0`"]
pub struct ASSCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH0_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH0_A::VALUE2)
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
#[doc = "Assignment for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH1_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2,
}
impl From<ASSCH1_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH1_A) -> Self {
        match variant {
            ASSCH1_A::VALUE1 => false,
            ASSCH1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ASSCH1`"]
pub type ASSCH1_R = crate::R<bool, ASSCH1_A>;
impl ASSCH1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH1_A {
        match self.bits {
            false => ASSCH1_A::VALUE1,
            true => ASSCH1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH1_A::VALUE2
    }
}
#[doc = "Write proxy for field `ASSCH1`"]
pub struct ASSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH1_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH1_A::VALUE2)
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
#[doc = "Assignment for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH2_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2,
}
impl From<ASSCH2_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH2_A) -> Self {
        match variant {
            ASSCH2_A::VALUE1 => false,
            ASSCH2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ASSCH2`"]
pub type ASSCH2_R = crate::R<bool, ASSCH2_A>;
impl ASSCH2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH2_A {
        match self.bits {
            false => ASSCH2_A::VALUE1,
            true => ASSCH2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH2_A::VALUE2
    }
}
#[doc = "Write proxy for field `ASSCH2`"]
pub struct ASSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH2_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH2_A::VALUE2)
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
#[doc = "Assignment for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH3_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2,
}
impl From<ASSCH3_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH3_A) -> Self {
        match variant {
            ASSCH3_A::VALUE1 => false,
            ASSCH3_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ASSCH3`"]
pub type ASSCH3_R = crate::R<bool, ASSCH3_A>;
impl ASSCH3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH3_A {
        match self.bits {
            false => ASSCH3_A::VALUE1,
            true => ASSCH3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH3_A::VALUE2
    }
}
#[doc = "Write proxy for field `ASSCH3`"]
pub struct ASSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH3_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH3_A::VALUE2)
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
#[doc = "Assignment for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH4_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2,
}
impl From<ASSCH4_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH4_A) -> Self {
        match variant {
            ASSCH4_A::VALUE1 => false,
            ASSCH4_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ASSCH4`"]
pub type ASSCH4_R = crate::R<bool, ASSCH4_A>;
impl ASSCH4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH4_A {
        match self.bits {
            false => ASSCH4_A::VALUE1,
            true => ASSCH4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH4_A::VALUE2
    }
}
#[doc = "Write proxy for field `ASSCH4`"]
pub struct ASSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH4_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH4_A::VALUE2)
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
#[doc = "Assignment for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH5_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2,
}
impl From<ASSCH5_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH5_A) -> Self {
        match variant {
            ASSCH5_A::VALUE1 => false,
            ASSCH5_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ASSCH5`"]
pub type ASSCH5_R = crate::R<bool, ASSCH5_A>;
impl ASSCH5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH5_A {
        match self.bits {
            false => ASSCH5_A::VALUE1,
            true => ASSCH5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH5_A::VALUE2
    }
}
#[doc = "Write proxy for field `ASSCH5`"]
pub struct ASSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH5_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH5_A::VALUE2)
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
#[doc = "Assignment for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH6_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2,
}
impl From<ASSCH6_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH6_A) -> Self {
        match variant {
            ASSCH6_A::VALUE1 => false,
            ASSCH6_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ASSCH6`"]
pub type ASSCH6_R = crate::R<bool, ASSCH6_A>;
impl ASSCH6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH6_A {
        match self.bits {
            false => ASSCH6_A::VALUE1,
            true => ASSCH6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH6_A::VALUE2
    }
}
#[doc = "Write proxy for field `ASSCH6`"]
pub struct ASSCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH6_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH6_A::VALUE2)
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
#[doc = "Assignment for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH7_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2,
}
impl From<ASSCH7_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH7_A) -> Self {
        match variant {
            ASSCH7_A::VALUE1 => false,
            ASSCH7_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ASSCH7`"]
pub type ASSCH7_R = crate::R<bool, ASSCH7_A>;
impl ASSCH7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH7_A {
        match self.bits {
            false => ASSCH7_A::VALUE1,
            true => ASSCH7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH7_A::VALUE2
    }
}
#[doc = "Write proxy for field `ASSCH7`"]
pub struct ASSCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH7_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH7_A::VALUE2)
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
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline(always)]
    pub fn assch0(&self) -> ASSCH0_R {
        ASSCH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline(always)]
    pub fn assch1(&self) -> ASSCH1_R {
        ASSCH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline(always)]
    pub fn assch2(&self) -> ASSCH2_R {
        ASSCH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline(always)]
    pub fn assch3(&self) -> ASSCH3_R {
        ASSCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline(always)]
    pub fn assch4(&self) -> ASSCH4_R {
        ASSCH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline(always)]
    pub fn assch5(&self) -> ASSCH5_R {
        ASSCH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline(always)]
    pub fn assch6(&self) -> ASSCH6_R {
        ASSCH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline(always)]
    pub fn assch7(&self) -> ASSCH7_R {
        ASSCH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline(always)]
    pub fn assch0(&mut self) -> ASSCH0_W {
        ASSCH0_W { w: self }
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline(always)]
    pub fn assch1(&mut self) -> ASSCH1_W {
        ASSCH1_W { w: self }
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline(always)]
    pub fn assch2(&mut self) -> ASSCH2_W {
        ASSCH2_W { w: self }
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline(always)]
    pub fn assch3(&mut self) -> ASSCH3_W {
        ASSCH3_W { w: self }
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline(always)]
    pub fn assch4(&mut self) -> ASSCH4_W {
        ASSCH4_W { w: self }
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline(always)]
    pub fn assch5(&mut self) -> ASSCH5_W {
        ASSCH5_W { w: self }
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline(always)]
    pub fn assch6(&mut self) -> ASSCH6_W {
        ASSCH6_W { w: self }
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline(always)]
    pub fn assch7(&mut self) -> ASSCH7_W {
        ASSCH7_W { w: self }
    }
}
