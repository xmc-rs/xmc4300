#[doc = "Reader of register CHENREG"]
pub type R = crate::R<u32, super::CHENREG>;
#[doc = "Writer for register CHENREG"]
pub type W = crate::W<u32, super::CHENREG>;
#[doc = "Register CHENREG `reset()`'s with value 0"]
impl crate::ResetValue for super::CHENREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `WE_CH`"]
pub struct WE_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Enables/Disables the channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH_A {
    #[doc = "0: Disable the Channel"]
    VALUE1 = 0,
    #[doc = "1: Enable the Channel"]
    VALUE2 = 1,
}
impl From<CH_A> for u8 {
    #[inline(always)]
    fn from(variant: CH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CH`"]
pub type CH_R = crate::R<u8, CH_A>;
impl CH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH_A::VALUE1),
            1 => Val(CH_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH_A::VALUE2
    }
}
#[doc = "Write proxy for field `CH`"]
pub struct CH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable the Channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH_A::VALUE1)
    }
    #[doc = "Enable the Channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Channel enable write enable"]
    #[inline(always)]
    pub fn we_ch(&mut self) -> WE_CH_W {
        WE_CH_W { w: self }
    }
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline(always)]
    pub fn ch(&mut self) -> CH_W {
        CH_W { w: self }
    }
}
