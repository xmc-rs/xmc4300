#[doc = "Reader of register RUN_LED"]
pub type R = crate::R<u8, super::RUN_LED>;
#[doc = "Writer for register RUN_LED"]
pub type W = crate::W<u8, super::RUN_LED>;
#[doc = "Register RUN_LED `reset()`'s with value 0x0e"]
impl crate::ResetValue for super::RUN_LED {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0e
    }
}
#[doc = "LED Code\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LED_CODE_A {
    #[doc = "0: OFF - Init State"]
    VALUE1 = 0,
    #[doc = "1: Flash - SafeOp)"]
    VALUE2 = 1,
    #[doc = "13: Blinking - PreOp"]
    VALUE3 = 13,
    #[doc = "14: Flickering - Bootstrap"]
    VALUE4 = 14,
    #[doc = "15: On - Operational"]
    VALUE5 = 15,
}
impl From<LED_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LED_CODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LED_CODE`"]
pub type LED_CODE_R = crate::R<u8, LED_CODE_A>;
impl LED_CODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LED_CODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LED_CODE_A::VALUE1),
            1 => Val(LED_CODE_A::VALUE2),
            13 => Val(LED_CODE_A::VALUE3),
            14 => Val(LED_CODE_A::VALUE4),
            15 => Val(LED_CODE_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LED_CODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LED_CODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LED_CODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LED_CODE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == LED_CODE_A::VALUE5
    }
}
#[doc = "Write proxy for field `LED_CODE`"]
pub struct LED_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_CODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LED_CODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "OFF - Init State"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE1)
    }
    #[doc = "Flash - SafeOp)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE2)
    }
    #[doc = "Blinking - PreOp"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE3)
    }
    #[doc = "Flickering - Bootstrap"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE4)
    }
    #[doc = "On - Operational"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(LED_CODE_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Enable Override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_OVERR_A {
    #[doc = "0: Override disable"]
    VALUE1 = 0,
    #[doc = "1: Override enable"]
    VALUE2 = 1,
}
impl From<EN_OVERR_A> for bool {
    #[inline(always)]
    fn from(variant: EN_OVERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN_OVERR`"]
pub type EN_OVERR_R = crate::R<bool, EN_OVERR_A>;
impl EN_OVERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_OVERR_A {
        match self.bits {
            false => EN_OVERR_A::VALUE1,
            true => EN_OVERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EN_OVERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EN_OVERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `EN_OVERR`"]
pub struct EN_OVERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_OVERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_OVERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Override disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EN_OVERR_A::VALUE1)
    }
    #[doc = "Override enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EN_OVERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - LED Code"]
    #[inline(always)]
    pub fn led_code(&self) -> LED_CODE_R {
        LED_CODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline(always)]
    pub fn en_overr(&self) -> EN_OVERR_R {
        EN_OVERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - LED Code"]
    #[inline(always)]
    pub fn led_code(&mut self) -> LED_CODE_W {
        LED_CODE_W { w: self }
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline(always)]
    pub fn en_overr(&mut self) -> EN_OVERR_W {
        EN_OVERR_W { w: self }
    }
}
