#[doc = "Reader of register PCON"]
pub type R = crate::R<u32, super::PCON>;
#[doc = "Writer for register PCON"]
pub type W = crate::W<u32, super::PCON>;
#[doc = "Register PCON `reset()`'s with value 0"]
impl crate::ResetValue for super::PCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Instruction Prefetch Buffer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBYP_A {
    #[doc = "0: Instruction prefetch buffer not bypassed."]
    CONST_0,
    #[doc = "1: Instruction prefetch buffer bypassed."]
    CONST_1,
}
impl From<IBYP_A> for bool {
    #[inline(always)]
    fn from(variant: IBYP_A) -> Self {
        match variant {
            IBYP_A::CONST_0 => false,
            IBYP_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `IBYP`"]
pub type IBYP_R = crate::R<bool, IBYP_A>;
impl IBYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBYP_A {
        match self.bits {
            false => IBYP_A::CONST_0,
            true => IBYP_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == IBYP_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == IBYP_A::CONST_1
    }
}
#[doc = "Write proxy for field `IBYP`"]
pub struct IBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> IBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(IBYP_A::CONST_0)
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(IBYP_A::CONST_1)
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
#[doc = "Instruction Prefetch Buffer Invalidate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IINV_AW {
    #[doc = "0: No effect."]
    CONST_0,
    #[doc = "1: Initiate invalidation of entire instruction cache."]
    CONST_1,
}
impl From<IINV_AW> for bool {
    #[inline(always)]
    fn from(variant: IINV_AW) -> Self {
        match variant {
            IINV_AW::CONST_0 => false,
            IINV_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `IINV`"]
pub struct IINV_W<'a> {
    w: &'a mut W,
}
impl<'a> IINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IINV_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(IINV_AW::CONST_0)
    }
    #[doc = "Initiate invalidation of entire instruction cache."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(IINV_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Data Buffer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBYP_A {
    #[doc = "0: Prefetch Data buffer not bypassed."]
    VALUE1,
    #[doc = "1: Prefetch Data buffer bypassed."]
    VALUE2,
}
impl From<DBYP_A> for bool {
    #[inline(always)]
    fn from(variant: DBYP_A) -> Self {
        match variant {
            DBYP_A::VALUE1 => false,
            DBYP_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DBYP`"]
pub type DBYP_R = crate::R<bool, DBYP_A>;
impl DBYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBYP_A {
        match self.bits {
            false => DBYP_A::VALUE1,
            true => DBYP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DBYP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DBYP_A::VALUE2
    }
}
#[doc = "Write proxy for field `DBYP`"]
pub struct DBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prefetch Data buffer not bypassed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DBYP_A::VALUE1)
    }
    #[doc = "Prefetch Data buffer bypassed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DBYP_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    pub fn ibyp(&self) -> IBYP_R {
        IBYP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Buffer Bypass"]
    #[inline(always)]
    pub fn dbyp(&self) -> DBYP_R {
        DBYP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    pub fn ibyp(&mut self) -> IBYP_W {
        IBYP_W { w: self }
    }
    #[doc = "Bit 1 - Instruction Prefetch Buffer Invalidate"]
    #[inline(always)]
    pub fn iinv(&mut self) -> IINV_W {
        IINV_W { w: self }
    }
    #[doc = "Bit 4 - Data Buffer Bypass"]
    #[inline(always)]
    pub fn dbyp(&mut self) -> DBYP_W {
        DBYP_W { w: self }
    }
}
