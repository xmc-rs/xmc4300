#[doc = "Reader of register CPACR"]
pub type R = crate::R<u32, super::CPACR>;
#[doc = "Writer for register CPACR"]
pub type W = crate::W<u32, super::CPACR>;
#[doc = "Register CPACR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Access privileges for coprocessor 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP10_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    VALUE1 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP fault."]
    VALUE2 = 1,
    #[doc = "3: Full access."]
    VALUE4 = 3,
}
impl From<CP10_A> for u8 {
    #[inline(always)]
    fn from(variant: CP10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP10`"]
pub type CP10_R = crate::R<u8, CP10_A>;
impl CP10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP10_A::VALUE1),
            1 => Val(CP10_A::VALUE2),
            3 => Val(CP10_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CP10_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CP10_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CP10_A::VALUE4
    }
}
#[doc = "Write proxy for field `CP10`"]
pub struct CP10_W<'a> {
    w: &'a mut W,
}
impl<'a> CP10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CP10_A::VALUE1)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CP10_A::VALUE2)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CP10_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP11_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    VALUE1 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP fault."]
    VALUE2 = 1,
    #[doc = "3: Full access."]
    VALUE4 = 3,
}
impl From<CP11_A> for u8 {
    #[inline(always)]
    fn from(variant: CP11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP11`"]
pub type CP11_R = crate::R<u8, CP11_A>;
impl CP11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP11_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP11_A::VALUE1),
            1 => Val(CP11_A::VALUE2),
            3 => Val(CP11_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CP11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CP11_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CP11_A::VALUE4
    }
}
#[doc = "Write proxy for field `CP11`"]
pub struct CP11_W<'a> {
    w: &'a mut W,
}
impl<'a> CP11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CP11_A::VALUE1)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CP11_A::VALUE2)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CP11_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    pub fn cp10(&mut self) -> CP10_W {
        CP10_W { w: self }
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    pub fn cp11(&mut self) -> CP11_W {
        CP11_W { w: self }
    }
}
