#[doc = "Reader of register LNEN"]
pub type R = crate::R<u32, super::LNEN>;
#[doc = "Writer for register LNEN"]
pub type W = crate::W<u32, super::LNEN>;
#[doc = "Register LNEN `reset()`'s with value 0"]
impl crate::ResetValue for super::LNEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Line 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN0_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN0_A> for bool {
    #[inline(always)]
    fn from(variant: LN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LN0`"]
pub type LN0_R = crate::R<bool, LN0_A>;
impl LN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN0_A {
        match self.bits {
            false => LN0_A::CONST_0,
            true => LN0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN0_A::CONST_1
    }
}
#[doc = "Write proxy for field `LN0`"]
pub struct LN0_W<'a> {
    w: &'a mut W,
}
impl<'a> LN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LN0_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LN0_A::CONST_1)
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
#[doc = "Line 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN1_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN1_A> for bool {
    #[inline(always)]
    fn from(variant: LN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LN1`"]
pub type LN1_R = crate::R<bool, LN1_A>;
impl LN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN1_A {
        match self.bits {
            false => LN1_A::CONST_0,
            true => LN1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN1_A::CONST_1
    }
}
#[doc = "Write proxy for field `LN1`"]
pub struct LN1_W<'a> {
    w: &'a mut W,
}
impl<'a> LN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LN1_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LN1_A::CONST_1)
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
#[doc = "Line 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN2_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN2_A> for bool {
    #[inline(always)]
    fn from(variant: LN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LN2`"]
pub type LN2_R = crate::R<bool, LN2_A>;
impl LN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN2_A {
        match self.bits {
            false => LN2_A::CONST_0,
            true => LN2_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN2_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN2_A::CONST_1
    }
}
#[doc = "Write proxy for field `LN2`"]
pub struct LN2_W<'a> {
    w: &'a mut W,
}
impl<'a> LN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LN2_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LN2_A::CONST_1)
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
#[doc = "Line 3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN3_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN3_A> for bool {
    #[inline(always)]
    fn from(variant: LN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LN3`"]
pub type LN3_R = crate::R<bool, LN3_A>;
impl LN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN3_A {
        match self.bits {
            false => LN3_A::CONST_0,
            true => LN3_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN3_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN3_A::CONST_1
    }
}
#[doc = "Write proxy for field `LN3`"]
pub struct LN3_W<'a> {
    w: &'a mut W,
}
impl<'a> LN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LN3_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LN3_A::CONST_1)
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
#[doc = "Line 4 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN4_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN4_A> for bool {
    #[inline(always)]
    fn from(variant: LN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LN4`"]
pub type LN4_R = crate::R<bool, LN4_A>;
impl LN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN4_A {
        match self.bits {
            false => LN4_A::CONST_0,
            true => LN4_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN4_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN4_A::CONST_1
    }
}
#[doc = "Write proxy for field `LN4`"]
pub struct LN4_W<'a> {
    w: &'a mut W,
}
impl<'a> LN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LN4_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LN4_A::CONST_1)
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
#[doc = "Line 5 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN5_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN5_A> for bool {
    #[inline(always)]
    fn from(variant: LN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LN5`"]
pub type LN5_R = crate::R<bool, LN5_A>;
impl LN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN5_A {
        match self.bits {
            false => LN5_A::CONST_0,
            true => LN5_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN5_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN5_A::CONST_1
    }
}
#[doc = "Write proxy for field `LN5`"]
pub struct LN5_W<'a> {
    w: &'a mut W,
}
impl<'a> LN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LN5_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LN5_A::CONST_1)
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
#[doc = "Line 6 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN6_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN6_A> for bool {
    #[inline(always)]
    fn from(variant: LN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LN6`"]
pub type LN6_R = crate::R<bool, LN6_A>;
impl LN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN6_A {
        match self.bits {
            false => LN6_A::CONST_0,
            true => LN6_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN6_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN6_A::CONST_1
    }
}
#[doc = "Write proxy for field `LN6`"]
pub struct LN6_W<'a> {
    w: &'a mut W,
}
impl<'a> LN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LN6_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LN6_A::CONST_1)
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
#[doc = "Line 7 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN7_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN7_A> for bool {
    #[inline(always)]
    fn from(variant: LN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LN7`"]
pub type LN7_R = crate::R<bool, LN7_A>;
impl LN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN7_A {
        match self.bits {
            false => LN7_A::CONST_0,
            true => LN7_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN7_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN7_A::CONST_1
    }
}
#[doc = "Write proxy for field `LN7`"]
pub struct LN7_W<'a> {
    w: &'a mut W,
}
impl<'a> LN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LN7_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LN7_A::CONST_1)
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
    #[doc = "Bit 0 - Line 0 Enable"]
    #[inline(always)]
    pub fn ln0(&self) -> LN0_R {
        LN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Line 1 Enable"]
    #[inline(always)]
    pub fn ln1(&self) -> LN1_R {
        LN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Line 2 Enable"]
    #[inline(always)]
    pub fn ln2(&self) -> LN2_R {
        LN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Line 3 Enable"]
    #[inline(always)]
    pub fn ln3(&self) -> LN3_R {
        LN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Line 4 Enable"]
    #[inline(always)]
    pub fn ln4(&self) -> LN4_R {
        LN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Line 5 Enable"]
    #[inline(always)]
    pub fn ln5(&self) -> LN5_R {
        LN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Line 6 Enable"]
    #[inline(always)]
    pub fn ln6(&self) -> LN6_R {
        LN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Line 7 Enable"]
    #[inline(always)]
    pub fn ln7(&self) -> LN7_R {
        LN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Line 0 Enable"]
    #[inline(always)]
    pub fn ln0(&mut self) -> LN0_W {
        LN0_W { w: self }
    }
    #[doc = "Bit 1 - Line 1 Enable"]
    #[inline(always)]
    pub fn ln1(&mut self) -> LN1_W {
        LN1_W { w: self }
    }
    #[doc = "Bit 2 - Line 2 Enable"]
    #[inline(always)]
    pub fn ln2(&mut self) -> LN2_W {
        LN2_W { w: self }
    }
    #[doc = "Bit 3 - Line 3 Enable"]
    #[inline(always)]
    pub fn ln3(&mut self) -> LN3_W {
        LN3_W { w: self }
    }
    #[doc = "Bit 4 - Line 4 Enable"]
    #[inline(always)]
    pub fn ln4(&mut self) -> LN4_W {
        LN4_W { w: self }
    }
    #[doc = "Bit 5 - Line 5 Enable"]
    #[inline(always)]
    pub fn ln5(&mut self) -> LN5_W {
        LN5_W { w: self }
    }
    #[doc = "Bit 6 - Line 6 Enable"]
    #[inline(always)]
    pub fn ln6(&mut self) -> LN6_W {
        LN6_W { w: self }
    }
    #[doc = "Bit 7 - Line 7 Enable"]
    #[inline(always)]
    pub fn ln7(&mut self) -> LN7_W {
        LN7_W { w: self }
    }
}
