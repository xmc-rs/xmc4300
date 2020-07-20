#[doc = "Reader of register DC_ACT"]
pub type R = crate::R<u8, super::DC_ACT>;
#[doc = "Writer for register DC_ACT"]
pub type W = crate::W<u8, super::DC_ACT>;
#[doc = "Register DC_ACT `reset()`'s with value 0"]
impl crate::ResetValue for super::DC_ACT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sync Out Unit activation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_OUT_A {
    #[doc = "0: Deactivated"]
    VALUE1 = 0,
    #[doc = "1: Activated"]
    VALUE2 = 1,
}
impl From<SYNC_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNC_OUT`"]
pub type SYNC_OUT_R = crate::R<bool, SYNC_OUT_A>;
impl SYNC_OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_OUT_A {
        match self.bits {
            false => SYNC_OUT_A::VALUE1,
            true => SYNC_OUT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_OUT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_OUT_A::VALUE2
    }
}
#[doc = "Write proxy for field `SYNC_OUT`"]
pub struct SYNC_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_OUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_OUT_A::VALUE1)
    }
    #[doc = "Activated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_OUT_A::VALUE2)
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
#[doc = "SYNC0 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_0_A {
    #[doc = "0: Deactivated"]
    VALUE1 = 0,
    #[doc = "1: SYNC0 pulse is generated"]
    VALUE2 = 1,
}
impl From<SYNC_0_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNC_0`"]
pub type SYNC_0_R = crate::R<bool, SYNC_0_A>;
impl SYNC_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_0_A {
        match self.bits {
            false => SYNC_0_A::VALUE1,
            true => SYNC_0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_0_A::VALUE2
    }
}
#[doc = "Write proxy for field `SYNC_0`"]
pub struct SYNC_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_0_A::VALUE1)
    }
    #[doc = "SYNC0 pulse is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_0_A::VALUE2)
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
#[doc = "SYNC1 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_1_A {
    #[doc = "0: Deactivated"]
    VALUE1 = 0,
    #[doc = "1: SYNC1 pulse is generated"]
    VALUE2 = 1,
}
impl From<SYNC_1_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNC_1`"]
pub type SYNC_1_R = crate::R<bool, SYNC_1_A>;
impl SYNC_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_1_A {
        match self.bits {
            false => SYNC_1_A::VALUE1,
            true => SYNC_1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_1_A::VALUE2
    }
}
#[doc = "Write proxy for field `SYNC_1`"]
pub struct SYNC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_1_A::VALUE1)
    }
    #[doc = "SYNC1 pulse is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_1_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Sync Out Unit activation"]
    #[inline(always)]
    pub fn sync_out(&self) -> SYNC_OUT_R {
        SYNC_OUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SYNC0 generation"]
    #[inline(always)]
    pub fn sync_0(&self) -> SYNC_0_R {
        SYNC_0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYNC1 generation"]
    #[inline(always)]
    pub fn sync_1(&self) -> SYNC_1_R {
        SYNC_1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync Out Unit activation"]
    #[inline(always)]
    pub fn sync_out(&mut self) -> SYNC_OUT_W {
        SYNC_OUT_W { w: self }
    }
    #[doc = "Bit 1 - SYNC0 generation"]
    #[inline(always)]
    pub fn sync_0(&mut self) -> SYNC_0_W {
        SYNC_0_W { w: self }
    }
    #[doc = "Bit 2 - SYNC1 generation"]
    #[inline(always)]
    pub fn sync_1(&mut self) -> SYNC_1_W {
        SYNC_1_W { w: self }
    }
}
