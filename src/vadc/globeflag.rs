#[doc = "Reader of register GLOBEFLAG"]
pub type R = crate::R<u32, super::GLOBEFLAG>;
#[doc = "Writer for register GLOBEFLAG"]
pub type W = crate::W<u32, super::GLOBEFLAG>;
#[doc = "Register GLOBEFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::GLOBEFLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Source Event (Background)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVGLB_A {
    #[doc = "0: No source event"]
    VALUE1,
    #[doc = "1: A source event has occurred"]
    VALUE2,
}
impl From<SEVGLB_A> for bool {
    #[inline(always)]
    fn from(variant: SEVGLB_A) -> Self {
        match variant {
            SEVGLB_A::VALUE1 => false,
            SEVGLB_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SEVGLB`"]
pub type SEVGLB_R = crate::R<bool, SEVGLB_A>;
impl SEVGLB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEVGLB_A {
        match self.bits {
            false => SEVGLB_A::VALUE1,
            true => SEVGLB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEVGLB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEVGLB_A::VALUE2
    }
}
#[doc = "Write proxy for field `SEVGLB`"]
pub struct SEVGLB_W<'a> {
    w: &'a mut W,
}
impl<'a> SEVGLB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEVGLB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No source event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEVGLB_A::VALUE1)
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEVGLB_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Global Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVGLB_A {
    #[doc = "0: No result event"]
    VALUE1,
    #[doc = "1: New result was stored in register GLOBRES"]
    VALUE2,
}
impl From<REVGLB_A> for bool {
    #[inline(always)]
    fn from(variant: REVGLB_A) -> Self {
        match variant {
            REVGLB_A::VALUE1 => false,
            REVGLB_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `REVGLB`"]
pub type REVGLB_R = crate::R<bool, REVGLB_A>;
impl REVGLB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REVGLB_A {
        match self.bits {
            false => REVGLB_A::VALUE1,
            true => REVGLB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REVGLB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REVGLB_A::VALUE2
    }
}
#[doc = "Write proxy for field `REVGLB`"]
pub struct REVGLB_W<'a> {
    w: &'a mut W,
}
impl<'a> REVGLB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REVGLB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REVGLB_A::VALUE1)
    }
    #[doc = "New result was stored in register GLOBRES"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REVGLB_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Clear Source Event (Background)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVGLBCLR_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Clear the source event flag SEVGLB"]
    VALUE2,
}
impl From<SEVGLBCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SEVGLBCLR_AW) -> Self {
        match variant {
            SEVGLBCLR_AW::VALUE1 => false,
            SEVGLBCLR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `SEVGLBCLR`"]
pub struct SEVGLBCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEVGLBCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEVGLBCLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEVGLBCLR_AW::VALUE1)
    }
    #[doc = "Clear the source event flag SEVGLB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEVGLBCLR_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Clear Global Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVGLBCLR_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Clear the result event flag REVGLB"]
    VALUE2,
}
impl From<REVGLBCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: REVGLBCLR_AW) -> Self {
        match variant {
            REVGLBCLR_AW::VALUE1 => false,
            REVGLBCLR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `REVGLBCLR`"]
pub struct REVGLBCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> REVGLBCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REVGLBCLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REVGLBCLR_AW::VALUE1)
    }
    #[doc = "Clear the result event flag REVGLB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REVGLBCLR_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Source Event (Background)"]
    #[inline(always)]
    pub fn sevglb(&self) -> SEVGLB_R {
        SEVGLB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Global Result Event"]
    #[inline(always)]
    pub fn revglb(&self) -> REVGLB_R {
        REVGLB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Event (Background)"]
    #[inline(always)]
    pub fn sevglb(&mut self) -> SEVGLB_W {
        SEVGLB_W { w: self }
    }
    #[doc = "Bit 8 - Global Result Event"]
    #[inline(always)]
    pub fn revglb(&mut self) -> REVGLB_W {
        REVGLB_W { w: self }
    }
    #[doc = "Bit 16 - Clear Source Event (Background)"]
    #[inline(always)]
    pub fn sevglbclr(&mut self) -> SEVGLBCLR_W {
        SEVGLBCLR_W { w: self }
    }
    #[doc = "Bit 24 - Clear Global Result Event"]
    #[inline(always)]
    pub fn revglbclr(&mut self) -> REVGLBCLR_W {
        REVGLBCLR_W { w: self }
    }
}
