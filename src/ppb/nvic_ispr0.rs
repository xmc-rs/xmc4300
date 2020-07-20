#[doc = "Reader of register NVIC_ISPR0"]
pub type R = crate::R<u32, super::NVIC_ISPR0>;
#[doc = "Writer for register NVIC_ISPR0"]
pub type W = crate::W<u32, super::NVIC_ISPR0>;
#[doc = "Register NVIC_ISPR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ISPR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt set-pending bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SETPEND_A {
    #[doc = "0: interrupt is not pending"]
    VALUE3 = 0,
    #[doc = "1: interrupt is pending."]
    VALUE4 = 1,
}
impl From<SETPEND_A> for u32 {
    #[inline(always)]
    fn from(variant: SETPEND_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SETPEND`"]
pub type SETPEND_R = crate::R<u32, SETPEND_A>;
impl SETPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, SETPEND_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETPEND_A::VALUE3),
            1 => Val(SETPEND_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SETPEND_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SETPEND_A::VALUE4
    }
}
#[doc = "Write proxy for field `SETPEND`"]
pub struct SETPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETPEND_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SETPEND_A::VALUE3)
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SETPEND_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    pub fn setpend(&self) -> SETPEND_R {
        SETPEND_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    pub fn setpend(&mut self) -> SETPEND_W {
        SETPEND_W { w: self }
    }
}
