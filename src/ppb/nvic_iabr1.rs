#[doc = "Reader of register NVIC_IABR1"]
pub type R = crate::R<u32, super::NVIC_IABR1>;
#[doc = "Writer for register NVIC_IABR1"]
pub type W = crate::W<u32, super::NVIC_IABR1>;
#[doc = "Register NVIC_IABR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IABR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt active flags:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum ACTIVE_A {
    #[doc = "0: interrupt not active"]
    VALUE1 = 0,
    #[doc = "1: interrupt active"]
    VALUE2 = 1,
}
impl From<ACTIVE_A> for u32 {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACTIVE`"]
pub type ACTIVE_R = crate::R<u32, ACTIVE_A>;
impl ACTIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, ACTIVE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ACTIVE_A::VALUE1),
            1 => Val(ACTIVE_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACTIVE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACTIVE_A::VALUE2
    }
}
#[doc = "Write proxy for field `ACTIVE`"]
pub struct ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTIVE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "interrupt not active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACTIVE_A::VALUE1)
    }
    #[doc = "interrupt active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACTIVE_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt active flags:"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt active flags:"]
    #[inline(always)]
    pub fn active(&mut self) -> ACTIVE_W {
        ACTIVE_W { w: self }
    }
}
