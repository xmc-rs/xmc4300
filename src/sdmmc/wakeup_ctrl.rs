#[doc = "Reader of register WAKEUP_CTRL"]
pub type R = crate::R<u8, super::WAKEUP_CTRL>;
#[doc = "Writer for register WAKEUP_CTRL"]
pub type W = crate::W<u8, super::WAKEUP_CTRL>;
#[doc = "Register WAKEUP_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKEUP_CTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wakeup Event Enable On SD Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_EVENT_EN_REM_A {
    #[doc = "0: Disable"]
    VALUE1,
    #[doc = "1: Enable"]
    VALUE2,
}
impl From<WAKEUP_EVENT_EN_REM_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_EVENT_EN_REM_A) -> Self {
        match variant {
            WAKEUP_EVENT_EN_REM_A::VALUE1 => false,
            WAKEUP_EVENT_EN_REM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WAKEUP_EVENT_EN_REM`"]
pub type WAKEUP_EVENT_EN_REM_R = crate::R<bool, WAKEUP_EVENT_EN_REM_A>;
impl WAKEUP_EVENT_EN_REM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_EVENT_EN_REM_A {
        match self.bits {
            false => WAKEUP_EVENT_EN_REM_A::VALUE1,
            true => WAKEUP_EVENT_EN_REM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAKEUP_EVENT_EN_REM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAKEUP_EVENT_EN_REM_A::VALUE2
    }
}
#[doc = "Write proxy for field `WAKEUP_EVENT_EN_REM`"]
pub struct WAKEUP_EVENT_EN_REM_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_EVENT_EN_REM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_EVENT_EN_REM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_REM_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_REM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Wakeup Event Enable On SD Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_EVENT_EN_INS_A {
    #[doc = "0: Disable"]
    VALUE1,
    #[doc = "1: Enable"]
    VALUE2,
}
impl From<WAKEUP_EVENT_EN_INS_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_EVENT_EN_INS_A) -> Self {
        match variant {
            WAKEUP_EVENT_EN_INS_A::VALUE1 => false,
            WAKEUP_EVENT_EN_INS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WAKEUP_EVENT_EN_INS`"]
pub type WAKEUP_EVENT_EN_INS_R = crate::R<bool, WAKEUP_EVENT_EN_INS_A>;
impl WAKEUP_EVENT_EN_INS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_EVENT_EN_INS_A {
        match self.bits {
            false => WAKEUP_EVENT_EN_INS_A::VALUE1,
            true => WAKEUP_EVENT_EN_INS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INS_A::VALUE2
    }
}
#[doc = "Write proxy for field `WAKEUP_EVENT_EN_INS`"]
pub struct WAKEUP_EVENT_EN_INS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_EVENT_EN_INS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_EVENT_EN_INS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INS_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Wakeup Event Enable On Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_EVENT_EN_INT_A {
    #[doc = "0: Disable"]
    VALUE1,
    #[doc = "1: Enable"]
    VALUE2,
}
impl From<WAKEUP_EVENT_EN_INT_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_EVENT_EN_INT_A) -> Self {
        match variant {
            WAKEUP_EVENT_EN_INT_A::VALUE1 => false,
            WAKEUP_EVENT_EN_INT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WAKEUP_EVENT_EN_INT`"]
pub type WAKEUP_EVENT_EN_INT_R = crate::R<bool, WAKEUP_EVENT_EN_INT_A>;
impl WAKEUP_EVENT_EN_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_EVENT_EN_INT_A {
        match self.bits {
            false => WAKEUP_EVENT_EN_INT_A::VALUE1,
            true => WAKEUP_EVENT_EN_INT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INT_A::VALUE2
    }
}
#[doc = "Write proxy for field `WAKEUP_EVENT_EN_INT`"]
pub struct WAKEUP_EVENT_EN_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_EVENT_EN_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_EVENT_EN_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INT_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INT_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wakeup_event_en_rem(&self) -> WAKEUP_EVENT_EN_REM_R {
        WAKEUP_EVENT_EN_REM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wakeup_event_en_ins(&self) -> WAKEUP_EVENT_EN_INS_R {
        WAKEUP_EVENT_EN_INS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wakeup_event_en_int(&self) -> WAKEUP_EVENT_EN_INT_R {
        WAKEUP_EVENT_EN_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wakeup_event_en_rem(&mut self) -> WAKEUP_EVENT_EN_REM_W {
        WAKEUP_EVENT_EN_REM_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wakeup_event_en_ins(&mut self) -> WAKEUP_EVENT_EN_INS_W {
        WAKEUP_EVENT_EN_INS_W { w: self }
    }
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wakeup_event_en_int(&mut self) -> WAKEUP_EVENT_EN_INT_W {
        WAKEUP_EVENT_EN_INT_W { w: self }
    }
}
