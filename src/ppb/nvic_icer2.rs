#[doc = "Reader of register NVIC_ICER2"]
pub type R = crate::R<u32, super::NVIC_ICER2>;
#[doc = "Writer for register NVIC_ICER2"]
pub type W = crate::W<u32, super::NVIC_ICER2>;
#[doc = "Register NVIC_ICER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ICER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt clear-enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRENA_A {
    #[doc = "0: interrupt disabled"]
    VALUE3,
    #[doc = "1: interrupt enabled."]
    VALUE4,
}
impl From<CLRENA_A> for u32 {
    #[inline(always)]
    fn from(variant: CLRENA_A) -> Self {
        match variant {
            CLRENA_A::VALUE3 => 0,
            CLRENA_A::VALUE4 => 1,
        }
    }
}
#[doc = "Reader of field `CLRENA`"]
pub type CLRENA_R = crate::R<u32, CLRENA_A>;
impl CLRENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CLRENA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLRENA_A::VALUE3),
            1 => Val(CLRENA_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLRENA_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLRENA_A::VALUE4
    }
}
#[doc = "Write proxy for field `CLRENA`"]
pub struct CLRENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRENA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLRENA_A::VALUE3)
    }
    #[doc = "interrupt enabled."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLRENA_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits."]
    #[inline(always)]
    pub fn clrena(&self) -> CLRENA_R {
        CLRENA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits."]
    #[inline(always)]
    pub fn clrena(&mut self) -> CLRENA_W {
        CLRENA_W { w: self }
    }
}
