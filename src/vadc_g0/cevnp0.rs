#[doc = "Register `CEVNP0` reader"]
pub struct R(crate::R<CEVNP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEVNP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEVNP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEVNP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEVNP0` writer"]
pub struct W(crate::W<CEVNP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEVNP0_SPEC>;
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
impl From<crate::W<CEVNP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEVNP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEV0NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV0NP_R = crate::FieldReader<u8, CEV0NP_A>;
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CEV0NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CEV0NP_A> {
        match self.bits {
            0 => Some(CEV0NP_A::VALUE1),
            3 => Some(CEV0NP_A::VALUE2),
            4 => Some(CEV0NP_A::VALUE3),
            7 => Some(CEV0NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `CEV0NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV0NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEVNP0_SPEC, u8, CEV0NP_A, 4, O>;
impl<'a, const O: u8> CEV0NP_W<'a, O> {
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
}
#[doc = "Field `CEV1NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV1NP_R = crate::FieldReader<u8, CEV1NP_A>;
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CEV1NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CEV1NP_A> {
        match self.bits {
            0 => Some(CEV1NP_A::VALUE1),
            3 => Some(CEV1NP_A::VALUE2),
            4 => Some(CEV1NP_A::VALUE3),
            7 => Some(CEV1NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `CEV1NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV1NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEVNP0_SPEC, u8, CEV1NP_A, 4, O>;
impl<'a, const O: u8> CEV1NP_W<'a, O> {
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
}
#[doc = "Field `CEV2NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV2NP_R = crate::FieldReader<u8, CEV2NP_A>;
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CEV2NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CEV2NP_A> {
        match self.bits {
            0 => Some(CEV2NP_A::VALUE1),
            3 => Some(CEV2NP_A::VALUE2),
            4 => Some(CEV2NP_A::VALUE3),
            7 => Some(CEV2NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `CEV2NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV2NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEVNP0_SPEC, u8, CEV2NP_A, 4, O>;
impl<'a, const O: u8> CEV2NP_W<'a, O> {
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
}
#[doc = "Field `CEV3NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV3NP_R = crate::FieldReader<u8, CEV3NP_A>;
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CEV3NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CEV3NP_A> {
        match self.bits {
            0 => Some(CEV3NP_A::VALUE1),
            3 => Some(CEV3NP_A::VALUE2),
            4 => Some(CEV3NP_A::VALUE3),
            7 => Some(CEV3NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `CEV3NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV3NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEVNP0_SPEC, u8, CEV3NP_A, 4, O>;
impl<'a, const O: u8> CEV3NP_W<'a, O> {
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
}
#[doc = "Field `CEV4NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV4NP_R = crate::FieldReader<u8, CEV4NP_A>;
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CEV4NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CEV4NP_A> {
        match self.bits {
            0 => Some(CEV4NP_A::VALUE1),
            3 => Some(CEV4NP_A::VALUE2),
            4 => Some(CEV4NP_A::VALUE3),
            7 => Some(CEV4NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `CEV4NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV4NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEVNP0_SPEC, u8, CEV4NP_A, 4, O>;
impl<'a, const O: u8> CEV4NP_W<'a, O> {
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
}
#[doc = "Field `CEV5NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV5NP_R = crate::FieldReader<u8, CEV5NP_A>;
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CEV5NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CEV5NP_A> {
        match self.bits {
            0 => Some(CEV5NP_A::VALUE1),
            3 => Some(CEV5NP_A::VALUE2),
            4 => Some(CEV5NP_A::VALUE3),
            7 => Some(CEV5NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `CEV5NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV5NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEVNP0_SPEC, u8, CEV5NP_A, 4, O>;
impl<'a, const O: u8> CEV5NP_W<'a, O> {
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
}
#[doc = "Field `CEV6NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV6NP_R = crate::FieldReader<u8, CEV6NP_A>;
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CEV6NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CEV6NP_A> {
        match self.bits {
            0 => Some(CEV6NP_A::VALUE1),
            3 => Some(CEV6NP_A::VALUE2),
            4 => Some(CEV6NP_A::VALUE3),
            7 => Some(CEV6NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `CEV6NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV6NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEVNP0_SPEC, u8, CEV6NP_A, 4, O>;
impl<'a, const O: u8> CEV6NP_W<'a, O> {
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
}
#[doc = "Field `CEV7NP` reader - Service Request Node Pointer Channel Event i"]
pub type CEV7NP_R = crate::FieldReader<u8, CEV7NP_A>;
#[doc = "Service Request Node Pointer Channel Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CEV7NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CEV7NP_A> {
        match self.bits {
            0 => Some(CEV7NP_A::VALUE1),
            3 => Some(CEV7NP_A::VALUE2),
            4 => Some(CEV7NP_A::VALUE3),
            7 => Some(CEV7NP_A::VALUE4),
            _ => None,
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
#[doc = "Field `CEV7NP` writer - Service Request Node Pointer Channel Event i"]
pub type CEV7NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEVNP0_SPEC, u8, CEV7NP_A, 4, O>;
impl<'a, const O: u8> CEV7NP_W<'a, O> {
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
    #[must_use]
    pub fn cev0np(&mut self) -> CEV0NP_W<0> {
        CEV0NP_W::new(self)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev1np(&mut self) -> CEV1NP_W<4> {
        CEV1NP_W::new(self)
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev2np(&mut self) -> CEV2NP_W<8> {
        CEV2NP_W::new(self)
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev3np(&mut self) -> CEV3NP_W<12> {
        CEV3NP_W::new(self)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev4np(&mut self) -> CEV4NP_W<16> {
        CEV4NP_W::new(self)
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev5np(&mut self) -> CEV5NP_W<20> {
        CEV5NP_W::new(self)
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev6np(&mut self) -> CEV6NP_W<24> {
        CEV6NP_W::new(self)
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Channel Event i"]
    #[inline(always)]
    #[must_use]
    pub fn cev7np(&mut self) -> CEV7NP_W<28> {
        CEV7NP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Event Node Pointer Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cevnp0](index.html) module"]
pub struct CEVNP0_SPEC;
impl crate::RegisterSpec for CEVNP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cevnp0::R](R) reader structure"]
impl crate::Readable for CEVNP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cevnp0::W](W) writer structure"]
impl crate::Writable for CEVNP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CEVNP0 to value 0"]
impl crate::Resettable for CEVNP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
