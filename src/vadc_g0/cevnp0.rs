#[doc = "Reader of register CEVNP0"]
pub type R = crate::R<u32, super::CEVNP0>;
#[doc = "Writer for register CEVNP0"]
pub type W = crate::W<u32, super::CEVNP0>;
#[doc = "Register CEVNP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CEVNP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEV0NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV0NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV0NP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEV0NP`"]
pub type CEV0NP_R = crate::R<u8, CEV0NP_A>;
impl CEV0NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CEV0NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CEV0NP_A::VALUE1),
            3 => Val(CEV0NP_A::VALUE2),
            4 => Val(CEV0NP_A::VALUE3),
            7 => Val(CEV0NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV0NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV0NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV0NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV0NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `CEV0NP`"]
pub struct CEV0NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV0NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV0NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV0NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV0NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEV1NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV1NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV1NP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEV1NP`"]
pub type CEV1NP_R = crate::R<u8, CEV1NP_A>;
impl CEV1NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CEV1NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CEV1NP_A::VALUE1),
            3 => Val(CEV1NP_A::VALUE2),
            4 => Val(CEV1NP_A::VALUE3),
            7 => Val(CEV1NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV1NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV1NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV1NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV1NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `CEV1NP`"]
pub struct CEV1NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV1NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV1NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV1NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV1NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV1NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV1NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEV2NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV2NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV2NP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEV2NP`"]
pub type CEV2NP_R = crate::R<u8, CEV2NP_A>;
impl CEV2NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CEV2NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CEV2NP_A::VALUE1),
            3 => Val(CEV2NP_A::VALUE2),
            4 => Val(CEV2NP_A::VALUE3),
            7 => Val(CEV2NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV2NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV2NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV2NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV2NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `CEV2NP`"]
pub struct CEV2NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV2NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV2NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV2NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV2NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV2NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV2NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEV3NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV3NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV3NP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEV3NP`"]
pub type CEV3NP_R = crate::R<u8, CEV3NP_A>;
impl CEV3NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CEV3NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CEV3NP_A::VALUE1),
            3 => Val(CEV3NP_A::VALUE2),
            4 => Val(CEV3NP_A::VALUE3),
            7 => Val(CEV3NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV3NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV3NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV3NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV3NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `CEV3NP`"]
pub struct CEV3NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV3NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV3NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV3NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV3NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV3NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV3NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEV4NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV4NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV4NP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEV4NP`"]
pub type CEV4NP_R = crate::R<u8, CEV4NP_A>;
impl CEV4NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CEV4NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CEV4NP_A::VALUE1),
            3 => Val(CEV4NP_A::VALUE2),
            4 => Val(CEV4NP_A::VALUE3),
            7 => Val(CEV4NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV4NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV4NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV4NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV4NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `CEV4NP`"]
pub struct CEV4NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV4NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV4NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV4NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV4NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV4NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV4NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEV5NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV5NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV5NP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEV5NP`"]
pub type CEV5NP_R = crate::R<u8, CEV5NP_A>;
impl CEV5NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CEV5NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CEV5NP_A::VALUE1),
            3 => Val(CEV5NP_A::VALUE2),
            4 => Val(CEV5NP_A::VALUE3),
            7 => Val(CEV5NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV5NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV5NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV5NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV5NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `CEV5NP`"]
pub struct CEV5NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV5NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV5NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV5NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV5NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV5NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV5NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEV6NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV6NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV6NP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEV6NP`"]
pub type CEV6NP_R = crate::R<u8, CEV6NP_A>;
impl CEV6NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CEV6NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CEV6NP_A::VALUE1),
            3 => Val(CEV6NP_A::VALUE2),
            4 => Val(CEV6NP_A::VALUE3),
            7 => Val(CEV6NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV6NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV6NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV6NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV6NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `CEV6NP`"]
pub struct CEV6NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV6NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV6NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV6NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV6NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV6NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV6NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEV7NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<CEV7NP_A> for u8 {
    #[inline(always)]
    fn from(variant: CEV7NP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEV7NP`"]
pub type CEV7NP_R = crate::R<u8, CEV7NP_A>;
impl CEV7NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CEV7NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CEV7NP_A::VALUE1),
            3 => Val(CEV7NP_A::VALUE2),
            4 => Val(CEV7NP_A::VALUE3),
            7 => Val(CEV7NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEV7NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEV7NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CEV7NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CEV7NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `CEV7NP`"]
pub struct CEV7NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV7NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV7NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV7NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV7NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV7NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV7NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev0np(&self) -> CEV0NP_R {
        CEV0NP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev1np(&self) -> CEV1NP_R {
        CEV1NP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev2np(&self) -> CEV2NP_R {
        CEV2NP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev3np(&self) -> CEV3NP_R {
        CEV3NP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev4np(&self) -> CEV4NP_R {
        CEV4NP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev5np(&self) -> CEV5NP_R {
        CEV5NP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev6np(&self) -> CEV6NP_R {
        CEV6NP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev7np(&self) -> CEV7NP_R {
        CEV7NP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev0np(&mut self) -> CEV0NP_W {
        CEV0NP_W { w: self }
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev1np(&mut self) -> CEV1NP_W {
        CEV1NP_W { w: self }
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev2np(&mut self) -> CEV2NP_W {
        CEV2NP_W { w: self }
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev3np(&mut self) -> CEV3NP_W {
        CEV3NP_W { w: self }
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev4np(&mut self) -> CEV4NP_W {
        CEV4NP_W { w: self }
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev5np(&mut self) -> CEV5NP_W {
        CEV5NP_W { w: self }
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev6np(&mut self) -> CEV6NP_W {
        CEV6NP_W { w: self }
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    pub fn cev7np(&mut self) -> CEV7NP_W {
        CEV7NP_W { w: self }
    }
}
