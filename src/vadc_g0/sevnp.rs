#[doc = "Reader of register SEVNP"]
pub type R = crate::R<u32, super::SEVNP>;
#[doc = "Writer for register SEVNP"]
pub type W = crate::W<u32, super::SEVNP>;
#[doc = "Register SEVNP `reset()`'s with value 0"]
impl crate::ResetValue for super::SEVNP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Service Request Node Pointer Source Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEV0NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
}
impl From<SEV0NP_A> for u8 {
    #[inline(always)]
    fn from(variant: SEV0NP_A) -> Self {
        match variant {
            SEV0NP_A::VALUE1 => 0,
            SEV0NP_A::VALUE2 => 3,
            SEV0NP_A::VALUE3 => 4,
            SEV0NP_A::VALUE4 => 7,
        }
    }
}
#[doc = "Reader of field `SEV0NP`"]
pub type SEV0NP_R = crate::R<u8, SEV0NP_A>;
impl SEV0NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEV0NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEV0NP_A::VALUE1),
            3 => Val(SEV0NP_A::VALUE2),
            4 => Val(SEV0NP_A::VALUE3),
            7 => Val(SEV0NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEV0NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEV0NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SEV0NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SEV0NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `SEV0NP`"]
pub struct SEV0NP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEV0NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEV0NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Service Request Node Pointer Source Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEV1NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2,
    #[doc = "4: Select shared service request line 0"]
    VALUE3,
    #[doc = "7: Select shared service request line 3"]
    VALUE4,
}
impl From<SEV1NP_A> for u8 {
    #[inline(always)]
    fn from(variant: SEV1NP_A) -> Self {
        match variant {
            SEV1NP_A::VALUE1 => 0,
            SEV1NP_A::VALUE2 => 3,
            SEV1NP_A::VALUE3 => 4,
            SEV1NP_A::VALUE4 => 7,
        }
    }
}
#[doc = "Reader of field `SEV1NP`"]
pub type SEV1NP_R = crate::R<u8, SEV1NP_A>;
impl SEV1NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEV1NP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEV1NP_A::VALUE1),
            3 => Val(SEV1NP_A::VALUE2),
            4 => Val(SEV1NP_A::VALUE3),
            7 => Val(SEV1NP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEV1NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEV1NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SEV1NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SEV1NP_A::VALUE4
    }
}
#[doc = "Write proxy for field `SEV1NP`"]
pub struct SEV1NP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEV1NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEV1NP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV1NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV1NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SEV1NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SEV1NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    pub fn sev0np(&self) -> SEV0NP_R {
        SEV0NP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    pub fn sev1np(&self) -> SEV1NP_R {
        SEV1NP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    pub fn sev0np(&mut self) -> SEV0NP_W {
        SEV0NP_W { w: self }
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    pub fn sev1np(&mut self) -> SEV1NP_W {
        SEV1NP_W { w: self }
    }
}
