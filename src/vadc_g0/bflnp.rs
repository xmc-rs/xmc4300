#[doc = "Reader of register BFLNP"]
pub type R = crate::R<u32, super::BFLNP>;
#[doc = "Writer for register BFLNP"]
pub type W = crate::W<u32, super::BFLNP>;
#[doc = "Register BFLNP `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::BFLNP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL0NP_A {
    #[doc = "0: Select common bondary flag output 0"]
    VALUE1,
    #[doc = "3: Select common bondary flag output 3"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
    #[doc = "15: Disabled, no common output signal"]
    VALUE5,
}
impl From<BFL0NP_A> for u8 {
    #[inline(always)]
    fn from(variant: BFL0NP_A) -> Self {
        match variant {
            BFL0NP_A::VALUE1 => 0,
            BFL0NP_A::VALUE2 => 3,
            BFL0NP_A::VALUE3 => 4,
            BFL0NP_A::VALUE4 => 7,
            BFL0NP_A::VALUE5 => 15,
        }
    }
}
#[doc = "Reader of field `BFL0NP`"]
pub type BFL0NP_R = crate::R<u8, BFL0NP_A>;
impl BFL0NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BFL0NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BFL0NP_A::VALUE1),
            3 => Val(BFL0NP_A::VALUE2),
            4 => Val(BFL0NP_A::VALUE3),
            7 => Val(BFL0NP_A::VALUE4),
            15 => Val(BFL0NP_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL0NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL0NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFL0NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFL0NP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BFL0NP_A::VALUE5
    }
}
#[doc = "Write proxy for field `BFL0NP`"]
pub struct BFL0NP_W<'a> {
    w: &'a mut W,
}
impl<'a> BFL0NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFL0NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL0NP_A::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL0NP_A::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL0NP_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL1NP_A {
    #[doc = "0: Select common bondary flag output 0"]
    VALUE1,
    #[doc = "3: Select common bondary flag output 3"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
    #[doc = "15: Disabled, no common output signal"]
    VALUE5,
}
impl From<BFL1NP_A> for u8 {
    #[inline(always)]
    fn from(variant: BFL1NP_A) -> Self {
        match variant {
            BFL1NP_A::VALUE1 => 0,
            BFL1NP_A::VALUE2 => 3,
            BFL1NP_A::VALUE3 => 4,
            BFL1NP_A::VALUE4 => 7,
            BFL1NP_A::VALUE5 => 15,
        }
    }
}
#[doc = "Reader of field `BFL1NP`"]
pub type BFL1NP_R = crate::R<u8, BFL1NP_A>;
impl BFL1NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BFL1NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BFL1NP_A::VALUE1),
            3 => Val(BFL1NP_A::VALUE2),
            4 => Val(BFL1NP_A::VALUE3),
            7 => Val(BFL1NP_A::VALUE4),
            15 => Val(BFL1NP_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL1NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL1NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFL1NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFL1NP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BFL1NP_A::VALUE5
    }
}
#[doc = "Write proxy for field `BFL1NP`"]
pub struct BFL1NP_W<'a> {
    w: &'a mut W,
}
impl<'a> BFL1NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFL1NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL1NP_A::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL1NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL1NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL1NP_A::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL1NP_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL2NP_A {
    #[doc = "0: Select common bondary flag output 0"]
    VALUE1,
    #[doc = "3: Select common bondary flag output 3"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
    #[doc = "15: Disabled, no common output signal"]
    VALUE5,
}
impl From<BFL2NP_A> for u8 {
    #[inline(always)]
    fn from(variant: BFL2NP_A) -> Self {
        match variant {
            BFL2NP_A::VALUE1 => 0,
            BFL2NP_A::VALUE2 => 3,
            BFL2NP_A::VALUE3 => 4,
            BFL2NP_A::VALUE4 => 7,
            BFL2NP_A::VALUE5 => 15,
        }
    }
}
#[doc = "Reader of field `BFL2NP`"]
pub type BFL2NP_R = crate::R<u8, BFL2NP_A>;
impl BFL2NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BFL2NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BFL2NP_A::VALUE1),
            3 => Val(BFL2NP_A::VALUE2),
            4 => Val(BFL2NP_A::VALUE3),
            7 => Val(BFL2NP_A::VALUE4),
            15 => Val(BFL2NP_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL2NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL2NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFL2NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFL2NP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BFL2NP_A::VALUE5
    }
}
#[doc = "Write proxy for field `BFL2NP`"]
pub struct BFL2NP_W<'a> {
    w: &'a mut W,
}
impl<'a> BFL2NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFL2NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL2NP_A::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL2NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL2NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL2NP_A::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL2NP_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL3NP_A {
    #[doc = "0: Select common bondary flag output 0"]
    VALUE1,
    #[doc = "3: Select common bondary flag output 3"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
    #[doc = "15: Disabled, no common output signal"]
    VALUE5,
}
impl From<BFL3NP_A> for u8 {
    #[inline(always)]
    fn from(variant: BFL3NP_A) -> Self {
        match variant {
            BFL3NP_A::VALUE1 => 0,
            BFL3NP_A::VALUE2 => 3,
            BFL3NP_A::VALUE3 => 4,
            BFL3NP_A::VALUE4 => 7,
            BFL3NP_A::VALUE5 => 15,
        }
    }
}
#[doc = "Reader of field `BFL3NP`"]
pub type BFL3NP_R = crate::R<u8, BFL3NP_A>;
impl BFL3NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BFL3NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BFL3NP_A::VALUE1),
            3 => Val(BFL3NP_A::VALUE2),
            4 => Val(BFL3NP_A::VALUE3),
            7 => Val(BFL3NP_A::VALUE4),
            15 => Val(BFL3NP_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL3NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL3NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFL3NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFL3NP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BFL3NP_A::VALUE5
    }
}
#[doc = "Write proxy for field `BFL3NP`"]
pub struct BFL3NP_W<'a> {
    w: &'a mut W,
}
impl<'a> BFL3NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFL3NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL3NP_A::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL3NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL3NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL3NP_A::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL3NP_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl0np(&self) -> BFL0NP_R {
        BFL0NP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl1np(&self) -> BFL1NP_R {
        BFL1NP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl2np(&self) -> BFL2NP_R {
        BFL2NP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl3np(&self) -> BFL3NP_R {
        BFL3NP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl0np(&mut self) -> BFL0NP_W {
        BFL0NP_W { w: self }
    }
    #[doc = "Bits 4:7 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl1np(&mut self) -> BFL1NP_W {
        BFL1NP_W { w: self }
    }
    #[doc = "Bits 8:11 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl2np(&mut self) -> BFL2NP_W {
        BFL2NP_W { w: self }
    }
    #[doc = "Bits 12:15 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl3np(&mut self) -> BFL3NP_W {
        BFL3NP_W { w: self }
    }
}
