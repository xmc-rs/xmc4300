#[doc = "Register `REVNP0` reader"]
pub struct R(crate::R<REVNP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REVNP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REVNP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REVNP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REVNP0` writer"]
pub struct W(crate::W<REVNP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REVNP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<REVNP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REVNP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REV0NP` reader - Service Request Node Pointer Result Event i"]
pub type REV0NP_R = crate::FieldReader<u8, REV0NP_A>;
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV0NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV0NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV0NP_A) -> Self {
        variant as _
    }
}
impl REV0NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REV0NP_A> {
        match self.bits {
            0 => Some(REV0NP_A::VALUE1),
            3 => Some(REV0NP_A::VALUE2),
            4 => Some(REV0NP_A::VALUE3),
            7 => Some(REV0NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `REV0NP` writer - Service Request Node Pointer Result Event i"]
pub type REV0NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVNP0_SPEC, u8, REV0NP_A, 4, O>;
impl<'a, const O: u8> REV0NP_W<'a, O> {
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
}
#[doc = "Field `REV1NP` reader - Service Request Node Pointer Result Event i"]
pub type REV1NP_R = crate::FieldReader<u8, REV1NP_A>;
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV1NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV1NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV1NP_A) -> Self {
        variant as _
    }
}
impl REV1NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REV1NP_A> {
        match self.bits {
            0 => Some(REV1NP_A::VALUE1),
            3 => Some(REV1NP_A::VALUE2),
            4 => Some(REV1NP_A::VALUE3),
            7 => Some(REV1NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `REV1NP` writer - Service Request Node Pointer Result Event i"]
pub type REV1NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVNP0_SPEC, u8, REV1NP_A, 4, O>;
impl<'a, const O: u8> REV1NP_W<'a, O> {
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
}
#[doc = "Field `REV2NP` reader - Service Request Node Pointer Result Event i"]
pub type REV2NP_R = crate::FieldReader<u8, REV2NP_A>;
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV2NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV2NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV2NP_A) -> Self {
        variant as _
    }
}
impl REV2NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REV2NP_A> {
        match self.bits {
            0 => Some(REV2NP_A::VALUE1),
            3 => Some(REV2NP_A::VALUE2),
            4 => Some(REV2NP_A::VALUE3),
            7 => Some(REV2NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `REV2NP` writer - Service Request Node Pointer Result Event i"]
pub type REV2NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVNP0_SPEC, u8, REV2NP_A, 4, O>;
impl<'a, const O: u8> REV2NP_W<'a, O> {
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
}
#[doc = "Field `REV3NP` reader - Service Request Node Pointer Result Event i"]
pub type REV3NP_R = crate::FieldReader<u8, REV3NP_A>;
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV3NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV3NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV3NP_A) -> Self {
        variant as _
    }
}
impl REV3NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REV3NP_A> {
        match self.bits {
            0 => Some(REV3NP_A::VALUE1),
            3 => Some(REV3NP_A::VALUE2),
            4 => Some(REV3NP_A::VALUE3),
            7 => Some(REV3NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `REV3NP` writer - Service Request Node Pointer Result Event i"]
pub type REV3NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVNP0_SPEC, u8, REV3NP_A, 4, O>;
impl<'a, const O: u8> REV3NP_W<'a, O> {
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
}
#[doc = "Field `REV4NP` reader - Service Request Node Pointer Result Event i"]
pub type REV4NP_R = crate::FieldReader<u8, REV4NP_A>;
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV4NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV4NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV4NP_A) -> Self {
        variant as _
    }
}
impl REV4NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REV4NP_A> {
        match self.bits {
            0 => Some(REV4NP_A::VALUE1),
            3 => Some(REV4NP_A::VALUE2),
            4 => Some(REV4NP_A::VALUE3),
            7 => Some(REV4NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `REV4NP` writer - Service Request Node Pointer Result Event i"]
pub type REV4NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVNP0_SPEC, u8, REV4NP_A, 4, O>;
impl<'a, const O: u8> REV4NP_W<'a, O> {
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
}
#[doc = "Field `REV5NP` reader - Service Request Node Pointer Result Event i"]
pub type REV5NP_R = crate::FieldReader<u8, REV5NP_A>;
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV5NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV5NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV5NP_A) -> Self {
        variant as _
    }
}
impl REV5NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REV5NP_A> {
        match self.bits {
            0 => Some(REV5NP_A::VALUE1),
            3 => Some(REV5NP_A::VALUE2),
            4 => Some(REV5NP_A::VALUE3),
            7 => Some(REV5NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `REV5NP` writer - Service Request Node Pointer Result Event i"]
pub type REV5NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVNP0_SPEC, u8, REV5NP_A, 4, O>;
impl<'a, const O: u8> REV5NP_W<'a, O> {
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
}
#[doc = "Field `REV6NP` reader - Service Request Node Pointer Result Event i"]
pub type REV6NP_R = crate::FieldReader<u8, REV6NP_A>;
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV6NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV6NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV6NP_A) -> Self {
        variant as _
    }
}
impl REV6NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REV6NP_A> {
        match self.bits {
            0 => Some(REV6NP_A::VALUE1),
            3 => Some(REV6NP_A::VALUE2),
            4 => Some(REV6NP_A::VALUE3),
            7 => Some(REV6NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `REV6NP` writer - Service Request Node Pointer Result Event i"]
pub type REV6NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVNP0_SPEC, u8, REV6NP_A, 4, O>;
impl<'a, const O: u8> REV6NP_W<'a, O> {
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
}
#[doc = "Field `REV7NP` reader - Service Request Node Pointer Result Event i"]
pub type REV7NP_R = crate::FieldReader<u8, REV7NP_A>;
#[doc = "Service Request Node Pointer Result Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV7NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<REV7NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV7NP_A) -> Self {
        variant as _
    }
}
impl REV7NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REV7NP_A> {
        match self.bits {
            0 => Some(REV7NP_A::VALUE1),
            3 => Some(REV7NP_A::VALUE2),
            4 => Some(REV7NP_A::VALUE3),
            7 => Some(REV7NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `REV7NP` writer - Service Request Node Pointer Result Event i"]
pub type REV7NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVNP0_SPEC, u8, REV7NP_A, 4, O>;
impl<'a, const O: u8> REV7NP_W<'a, O> {
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
    #[must_use]
    pub fn rev0np(&mut self) -> REV0NP_W<0> {
        REV0NP_W::new(self)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev1np(&mut self) -> REV1NP_W<4> {
        REV1NP_W::new(self)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev2np(&mut self) -> REV2NP_W<8> {
        REV2NP_W::new(self)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev3np(&mut self) -> REV3NP_W<12> {
        REV3NP_W::new(self)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev4np(&mut self) -> REV4NP_W<16> {
        REV4NP_W::new(self)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev5np(&mut self) -> REV5NP_W<20> {
        REV5NP_W::new(self)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev6np(&mut self) -> REV6NP_W<24> {
        REV6NP_W::new(self)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline(always)]
    #[must_use]
    pub fn rev7np(&mut self) -> REV7NP_W<28> {
        REV7NP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Result Event Node Pointer Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revnp0](index.html) module"]
pub struct REVNP0_SPEC;
impl crate::RegisterSpec for REVNP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [revnp0::R](R) reader structure"]
impl crate::Readable for REVNP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [revnp0::W](W) writer structure"]
impl crate::Writable for REVNP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REVNP0 to value 0"]
impl crate::Resettable for REVNP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
