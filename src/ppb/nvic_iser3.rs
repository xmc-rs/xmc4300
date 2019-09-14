#[doc = "Reader of register NVIC_ISER3"]
pub type R = crate::R<u32, super::NVIC_ISER3>;
#[doc = "Writer for register NVIC_ISER3"]
pub type W = crate::W<u32, super::NVIC_ISER3>;
#[doc = "Register NVIC_ISER3 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ISER3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt set-enable bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETENA_A {
    #[doc = "0: interrupt disabled"]
    VALUE3,
    #[doc = "1: interrupt enabled."]
    VALUE4,
}
impl From<SETENA_A> for u32 {
    #[inline(always)]
    fn from(variant: SETENA_A) -> Self {
        match variant {
            SETENA_A::VALUE3 => 0,
            SETENA_A::VALUE4 => 1,
        }
    }
}
#[doc = "Reader of field `SETENA`"]
pub type SETENA_R = crate::R<u32, SETENA_A>;
impl SETENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, SETENA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETENA_A::VALUE3),
            1 => Val(SETENA_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SETENA_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SETENA_A::VALUE4
    }
}
#[doc = "Write proxy for field `SETENA`"]
pub struct SETENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETENA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SETENA_A::VALUE3)
    }
    #[doc = "interrupt enabled."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SETENA_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-enable bits"]
    #[inline(always)]
    pub fn setena(&self) -> SETENA_R {
        SETENA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-enable bits"]
    #[inline(always)]
    pub fn setena(&mut self) -> SETENA_W {
        SETENA_W { w: self }
    }
}
