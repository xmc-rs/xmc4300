#[doc = "Reader of register REVNP0"]
pub type R = crate::R<u32, super::REVNP0>;
#[doc = "Writer for register REVNP0"]
pub type W = crate::W<u32, super::REVNP0>;
#[doc = "Register REVNP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::REVNP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV0NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
}
impl From<REV0NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV0NP_A) -> Self {
        match variant {
            REV0NP_A::VALUE1 => 0,
            REV0NP_A::VALUE2 => 3,
            REV0NP_A::VALUE3 => 4,
            REV0NP_A::VALUE4 => 7,
        }
    }
}
#[doc = "Reader of field `REV0NP`"]
pub type REV0NP_R = crate::R<u8, REV0NP_A>;
impl REV0NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REV0NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REV0NP_A::VALUE1),
            3 => Val(REV0NP_A::VALUE2),
            4 => Val(REV0NP_A::VALUE3),
            7 => Val(REV0NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV0NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV0NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV0NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV0NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `REV0NP`"]
pub struct REV0NP_W<'a> {
    w: &'a mut W,
}
impl<'a> REV0NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV0NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV0NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV0NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV1NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
}
impl From<REV1NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV1NP_A) -> Self {
        match variant {
            REV1NP_A::VALUE1 => 0,
            REV1NP_A::VALUE2 => 3,
            REV1NP_A::VALUE3 => 4,
            REV1NP_A::VALUE4 => 7,
        }
    }
}
#[doc = "Reader of field `REV1NP`"]
pub type REV1NP_R = crate::R<u8, REV1NP_A>;
impl REV1NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REV1NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REV1NP_A::VALUE1),
            3 => Val(REV1NP_A::VALUE2),
            4 => Val(REV1NP_A::VALUE3),
            7 => Val(REV1NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV1NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV1NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV1NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV1NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `REV1NP`"]
pub struct REV1NP_W<'a> {
    w: &'a mut W,
}
impl<'a> REV1NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV1NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV1NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV1NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV1NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV1NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV2NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
}
impl From<REV2NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV2NP_A) -> Self {
        match variant {
            REV2NP_A::VALUE1 => 0,
            REV2NP_A::VALUE2 => 3,
            REV2NP_A::VALUE3 => 4,
            REV2NP_A::VALUE4 => 7,
        }
    }
}
#[doc = "Reader of field `REV2NP`"]
pub type REV2NP_R = crate::R<u8, REV2NP_A>;
impl REV2NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REV2NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REV2NP_A::VALUE1),
            3 => Val(REV2NP_A::VALUE2),
            4 => Val(REV2NP_A::VALUE3),
            7 => Val(REV2NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV2NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV2NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV2NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV2NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `REV2NP`"]
pub struct REV2NP_W<'a> {
    w: &'a mut W,
}
impl<'a> REV2NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV2NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV2NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV2NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV2NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV2NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV3NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
}
impl From<REV3NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV3NP_A) -> Self {
        match variant {
            REV3NP_A::VALUE1 => 0,
            REV3NP_A::VALUE2 => 3,
            REV3NP_A::VALUE3 => 4,
            REV3NP_A::VALUE4 => 7,
        }
    }
}
#[doc = "Reader of field `REV3NP`"]
pub type REV3NP_R = crate::R<u8, REV3NP_A>;
impl REV3NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REV3NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REV3NP_A::VALUE1),
            3 => Val(REV3NP_A::VALUE2),
            4 => Val(REV3NP_A::VALUE3),
            7 => Val(REV3NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV3NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV3NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV3NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV3NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `REV3NP`"]
pub struct REV3NP_W<'a> {
    w: &'a mut W,
}
impl<'a> REV3NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV3NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV3NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV3NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV3NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV3NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV4NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
}
impl From<REV4NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV4NP_A) -> Self {
        match variant {
            REV4NP_A::VALUE1 => 0,
            REV4NP_A::VALUE2 => 3,
            REV4NP_A::VALUE3 => 4,
            REV4NP_A::VALUE4 => 7,
        }
    }
}
#[doc = "Reader of field `REV4NP`"]
pub type REV4NP_R = crate::R<u8, REV4NP_A>;
impl REV4NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REV4NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REV4NP_A::VALUE1),
            3 => Val(REV4NP_A::VALUE2),
            4 => Val(REV4NP_A::VALUE3),
            7 => Val(REV4NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV4NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV4NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV4NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV4NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `REV4NP`"]
pub struct REV4NP_W<'a> {
    w: &'a mut W,
}
impl<'a> REV4NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV4NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV4NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV4NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV4NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV4NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV5NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
}
impl From<REV5NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV5NP_A) -> Self {
        match variant {
            REV5NP_A::VALUE1 => 0,
            REV5NP_A::VALUE2 => 3,
            REV5NP_A::VALUE3 => 4,
            REV5NP_A::VALUE4 => 7,
        }
    }
}
#[doc = "Reader of field `REV5NP`"]
pub type REV5NP_R = crate::R<u8, REV5NP_A>;
impl REV5NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REV5NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REV5NP_A::VALUE1),
            3 => Val(REV5NP_A::VALUE2),
            4 => Val(REV5NP_A::VALUE3),
            7 => Val(REV5NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV5NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV5NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV5NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV5NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `REV5NP`"]
pub struct REV5NP_W<'a> {
    w: &'a mut W,
}
impl<'a> REV5NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV5NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV5NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV5NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV5NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV5NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV6NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
}
impl From<REV6NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV6NP_A) -> Self {
        match variant {
            REV6NP_A::VALUE1 => 0,
            REV6NP_A::VALUE2 => 3,
            REV6NP_A::VALUE3 => 4,
            REV6NP_A::VALUE4 => 7,
        }
    }
}
#[doc = "Reader of field `REV6NP`"]
pub type REV6NP_R = crate::R<u8, REV6NP_A>;
impl REV6NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REV6NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REV6NP_A::VALUE1),
            3 => Val(REV6NP_A::VALUE2),
            4 => Val(REV6NP_A::VALUE3),
            7 => Val(REV6NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV6NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV6NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV6NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV6NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `REV6NP`"]
pub struct REV6NP_W<'a> {
    w: &'a mut W,
}
impl<'a> REV6NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV6NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV6NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV6NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV6NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV6NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV7NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
}
impl From<REV7NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV7NP_A) -> Self {
        match variant {
            REV7NP_A::VALUE1 => 0,
            REV7NP_A::VALUE2 => 3,
            REV7NP_A::VALUE3 => 4,
            REV7NP_A::VALUE4 => 7,
        }
    }
}
#[doc = "Reader of field `REV7NP`"]
pub type REV7NP_R = crate::R<u8, REV7NP_A>;
impl REV7NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REV7NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REV7NP_A::VALUE1),
            3 => Val(REV7NP_A::VALUE2),
            4 => Val(REV7NP_A::VALUE3),
            7 => Val(REV7NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV7NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV7NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV7NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV7NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `REV7NP`"]
pub struct REV7NP_W<'a> {
    w: &'a mut W,
}
impl<'a> REV7NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV7NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV7NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV7NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV7NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV7NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev0np(&self) -> REV0NP_R {
        REV0NP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev1np(&self) -> REV1NP_R {
        REV1NP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev2np(&self) -> REV2NP_R {
        REV2NP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev3np(&self) -> REV3NP_R {
        REV3NP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev4np(&self) -> REV4NP_R {
        REV4NP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev5np(&self) -> REV5NP_R {
        REV5NP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev6np(&self) -> REV6NP_R {
        REV6NP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev7np(&self) -> REV7NP_R {
        REV7NP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev0np(&mut self) -> REV0NP_W {
        REV0NP_W { w: self }
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev1np(&mut self) -> REV1NP_W {
        REV1NP_W { w: self }
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev2np(&mut self) -> REV2NP_W {
        REV2NP_W { w: self }
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev3np(&mut self) -> REV3NP_W {
        REV3NP_W { w: self }
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev4np(&mut self) -> REV4NP_W {
        REV4NP_W { w: self }
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev5np(&mut self) -> REV5NP_W {
        REV5NP_W { w: self }
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev6np(&mut self) -> REV6NP_W {
        REV6NP_W { w: self }
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    pub fn rev7np(&mut self) -> REV7NP_W {
        REV7NP_W { w: self }
    }
}
