#[doc = "Reader of register GLOBEVNP"]
pub type R = crate::R<u32, super::GLOBEVNP>;
#[doc = "Writer for register GLOBEVNP"]
pub type W = crate::W<u32, super::GLOBEVNP>;
#[doc = "Register GLOBEVNP `reset()`'s with value 0"]
impl crate::ResetValue for super::GLOBEVNP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Service Request Node Pointer Backgr. Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEV0NP_A {
    #[doc = "0: Select shared service request line 0 of common service request group 0"]
    VALUE1 = 0,
    #[doc = "3: Select shared service request line 3 of common service request group 0"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0 of common service request group 1"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3 of common service request group 1"]
    VALUE4 = 7,
}
impl From<SEV0NP_A> for u8 {
    #[inline(always)]
    fn from(variant: SEV0NP_A) -> Self {
        variant as _
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
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE1)
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
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
#[doc = "Service Request Node Pointer Backgr. Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REV0NP_A {
    #[doc = "0: Select shared service request line 0 of common service request group 0"]
    VALUE1 = 0,
    #[doc = "3: Select shared service request line 3 of common service request group 0"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0 of common service request group 1"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3 of common service request group 1"]
    VALUE4 = 7,
}
impl From<REV0NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV0NP_A) -> Self {
        variant as _
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
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV0NP_A::VALUE1)
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV0NP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Backgr. Source"]
    #[inline(always)]
    pub fn sev0np(&self) -> SEV0NP_R {
        SEV0NP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Backgr. Result"]
    #[inline(always)]
    pub fn rev0np(&self) -> REV0NP_R {
        REV0NP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Backgr. Source"]
    #[inline(always)]
    pub fn sev0np(&mut self) -> SEV0NP_W {
        SEV0NP_W { w: self }
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Backgr. Result"]
    #[inline(always)]
    pub fn rev0np(&mut self) -> REV0NP_W {
        REV0NP_W { w: self }
    }
}
