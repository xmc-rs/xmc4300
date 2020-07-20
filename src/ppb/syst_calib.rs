#[doc = "Reader of register SYST_CALIB"]
pub type R = crate::R<u32, super::SYST_CALIB>;
#[doc = "Writer for register SYST_CALIB"]
pub type W = crate::W<u32, super::SYST_CALIB>;
#[doc = "Register SYST_CALIB `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::SYST_CALIB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Reader of field `TENMS`"]
pub type TENMS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TENMS`"]
pub struct TENMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TENMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Ten Milliseconds Skewed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEW_A {
    #[doc = "0: TENMS value is exact"]
    VALUE1 = 0,
    #[doc = "1: TENMS value is inexact, or not given."]
    VALUE2 = 1,
}
impl From<SKEW_A> for bool {
    #[inline(always)]
    fn from(variant: SKEW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SKEW`"]
pub type SKEW_R = crate::R<bool, SKEW_A>;
impl SKEW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKEW_A {
        match self.bits {
            false => SKEW_A::VALUE1,
            true => SKEW_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SKEW_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SKEW_A::VALUE2
    }
}
#[doc = "Write proxy for field `SKEW`"]
pub struct SKEW_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SKEW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TENMS value is exact"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SKEW_A::VALUE1)
    }
    #[doc = "TENMS value is inexact, or not given."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SKEW_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "No Reference Clock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOREF_A {
    #[doc = "0: reference clock provided"]
    VALUE1 = 0,
    #[doc = "1: no reference clock provided."]
    VALUE2 = 1,
}
impl From<NOREF_A> for bool {
    #[inline(always)]
    fn from(variant: NOREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NOREF`"]
pub type NOREF_R = crate::R<bool, NOREF_A>;
impl NOREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOREF_A {
        match self.bits {
            false => NOREF_A::VALUE1,
            true => NOREF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NOREF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NOREF_A::VALUE2
    }
}
#[doc = "Write proxy for field `NOREF`"]
pub struct NOREF_W<'a> {
    w: &'a mut W,
}
impl<'a> NOREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOREF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "reference clock provided"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NOREF_A::VALUE1)
    }
    #[doc = "no reference clock provided."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NOREF_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Ten Milliseconds Reload Value"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 30 - Ten Milliseconds Skewed"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - No Reference Clock"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Ten Milliseconds Reload Value"]
    #[inline(always)]
    pub fn tenms(&mut self) -> TENMS_W {
        TENMS_W { w: self }
    }
    #[doc = "Bit 30 - Ten Milliseconds Skewed"]
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W {
        SKEW_W { w: self }
    }
    #[doc = "Bit 31 - No Reference Clock"]
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W {
        NOREF_W { w: self }
    }
}
