#[doc = "Reader of register CLOCK_CTRL"]
pub type R = crate::R<u16, super::CLOCK_CTRL>;
#[doc = "Writer for register CLOCK_CTRL"]
pub type W = crate::W<u16, super::CLOCK_CTRL>;
#[doc = "Register CLOCK_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCK_CTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SDCLK Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDCLK_FREQ_SEL_A {
    #[doc = "0: base clock(10MHz-63MHz)"]
    VALUE1 = 0,
    #[doc = "1: base clock divided by 2"]
    VALUE2 = 1,
    #[doc = "16: base clock divided by 32"]
    VALUE3 = 16,
    #[doc = "2: base clock divided by 4"]
    VALUE4 = 2,
    #[doc = "4: base clock divided by 8"]
    VALUE5 = 4,
    #[doc = "8: base clock divided by 16"]
    VALUE6 = 8,
    #[doc = "32: base clock divided by 64"]
    VALUE7 = 32,
    #[doc = "64: base clock divided by 128"]
    VALUE8 = 64,
    #[doc = "128: base clock divided by 256"]
    VALUE9 = 128,
}
impl From<SDCLK_FREQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCLK_FREQ_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SDCLK_FREQ_SEL`"]
pub type SDCLK_FREQ_SEL_R = crate::R<u8, SDCLK_FREQ_SEL_A>;
impl SDCLK_FREQ_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SDCLK_FREQ_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SDCLK_FREQ_SEL_A::VALUE1),
            1 => Val(SDCLK_FREQ_SEL_A::VALUE2),
            16 => Val(SDCLK_FREQ_SEL_A::VALUE3),
            2 => Val(SDCLK_FREQ_SEL_A::VALUE4),
            4 => Val(SDCLK_FREQ_SEL_A::VALUE5),
            8 => Val(SDCLK_FREQ_SEL_A::VALUE6),
            32 => Val(SDCLK_FREQ_SEL_A::VALUE7),
            64 => Val(SDCLK_FREQ_SEL_A::VALUE8),
            128 => Val(SDCLK_FREQ_SEL_A::VALUE9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE9
    }
}
#[doc = "Write proxy for field `SDCLK_FREQ_SEL`"]
pub struct SDCLK_FREQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLK_FREQ_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCLK_FREQ_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "base clock(10MHz-63MHz)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SEL_A::VALUE1)
    }
    #[doc = "base clock divided by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SEL_A::VALUE2)
    }
    #[doc = "base clock divided by 32"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SEL_A::VALUE3)
    }
    #[doc = "base clock divided by 4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SEL_A::VALUE4)
    }
    #[doc = "base clock divided by 8"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SEL_A::VALUE5)
    }
    #[doc = "base clock divided by 16"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SEL_A::VALUE6)
    }
    #[doc = "base clock divided by 64"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SEL_A::VALUE7)
    }
    #[doc = "base clock divided by 128"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SEL_A::VALUE8)
    }
    #[doc = "base clock divided by 256"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SEL_A::VALUE9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
#[doc = "SD Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCLOCK_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<SDCLOCK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SDCLOCK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDCLOCK_EN`"]
pub type SDCLOCK_EN_R = crate::R<bool, SDCLOCK_EN_A>;
impl SDCLOCK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDCLOCK_EN_A {
        match self.bits {
            false => SDCLOCK_EN_A::VALUE1,
            true => SDCLOCK_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDCLOCK_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDCLOCK_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `SDCLOCK_EN`"]
pub struct SDCLOCK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLOCK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCLOCK_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDCLOCK_EN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDCLOCK_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Internal Clock Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERNAL_CLOCK_STABLE_A {
    #[doc = "0: Not Ready"]
    VALUE1 = 0,
    #[doc = "1: Ready"]
    VALUE2 = 1,
}
impl From<INTERNAL_CLOCK_STABLE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERNAL_CLOCK_STABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTERNAL_CLOCK_STABLE`"]
pub type INTERNAL_CLOCK_STABLE_R = crate::R<bool, INTERNAL_CLOCK_STABLE_A>;
impl INTERNAL_CLOCK_STABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERNAL_CLOCK_STABLE_A {
        match self.bits {
            false => INTERNAL_CLOCK_STABLE_A::VALUE1,
            true => INTERNAL_CLOCK_STABLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INTERNAL_CLOCK_STABLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INTERNAL_CLOCK_STABLE_A::VALUE2
    }
}
#[doc = "Internal Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERNAL_CLOCK_EN_A {
    #[doc = "0: Stop"]
    VALUE1 = 0,
    #[doc = "1: Oscillate"]
    VALUE2 = 1,
}
impl From<INTERNAL_CLOCK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: INTERNAL_CLOCK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTERNAL_CLOCK_EN`"]
pub type INTERNAL_CLOCK_EN_R = crate::R<bool, INTERNAL_CLOCK_EN_A>;
impl INTERNAL_CLOCK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERNAL_CLOCK_EN_A {
        match self.bits {
            false => INTERNAL_CLOCK_EN_A::VALUE1,
            true => INTERNAL_CLOCK_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INTERNAL_CLOCK_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INTERNAL_CLOCK_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `INTERNAL_CLOCK_EN`"]
pub struct INTERNAL_CLOCK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_CLOCK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERNAL_CLOCK_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INTERNAL_CLOCK_EN_A::VALUE1)
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(INTERNAL_CLOCK_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclk_freq_sel(&self) -> SDCLK_FREQ_SEL_R {
        SDCLK_FREQ_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclock_en(&self) -> SDCLOCK_EN_R {
        SDCLOCK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn internal_clock_stable(&self) -> INTERNAL_CLOCK_STABLE_R {
        INTERNAL_CLOCK_STABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn internal_clock_en(&self) -> INTERNAL_CLOCK_EN_R {
        INTERNAL_CLOCK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclk_freq_sel(&mut self) -> SDCLK_FREQ_SEL_W {
        SDCLK_FREQ_SEL_W { w: self }
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclock_en(&mut self) -> SDCLOCK_EN_W {
        SDCLOCK_EN_W { w: self }
    }
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn internal_clock_en(&mut self) -> INTERNAL_CLOCK_EN_W {
        INTERNAL_CLOCK_EN_W { w: self }
    }
}
