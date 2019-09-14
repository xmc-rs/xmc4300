#[doc = "Reader of register DC_LATCH0_CONT"]
pub type R = crate::R<u8, super::DC_LATCH0_CONT>;
#[doc = "Writer for register DC_LATCH0_CONT"]
pub type W = crate::W<u8, super::DC_LATCH0_CONT>;
#[doc = "Register DC_LATCH0_CONT `reset()`'s with value 0"]
impl crate::ResetValue for super::DC_LATCH0_CONT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Latch0 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L0_POS_A {
    #[doc = "0: Continuous Latch active"]
    VALUE1,
    #[doc = "1: Single event (only first event active)"]
    VALUE2,
}
impl From<L0_POS_A> for bool {
    #[inline(always)]
    fn from(variant: L0_POS_A) -> Self {
        match variant {
            L0_POS_A::VALUE1 => false,
            L0_POS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `L0_POS`"]
pub type L0_POS_R = crate::R<bool, L0_POS_A>;
impl L0_POS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L0_POS_A {
        match self.bits {
            false => L0_POS_A::VALUE1,
            true => L0_POS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L0_POS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L0_POS_A::VALUE2
    }
}
#[doc = "Write proxy for field `L0_POS`"]
pub struct L0_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> L0_POS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L0_POS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(L0_POS_A::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(L0_POS_A::VALUE2)
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
#[doc = "Latch0 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L0_NEG_A {
    #[doc = "0: Continuous Latch active"]
    VALUE1,
    #[doc = "1: Single event (only first event active)"]
    VALUE2,
}
impl From<L0_NEG_A> for bool {
    #[inline(always)]
    fn from(variant: L0_NEG_A) -> Self {
        match variant {
            L0_NEG_A::VALUE1 => false,
            L0_NEG_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `L0_NEG`"]
pub type L0_NEG_R = crate::R<bool, L0_NEG_A>;
impl L0_NEG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L0_NEG_A {
        match self.bits {
            false => L0_NEG_A::VALUE1,
            true => L0_NEG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L0_NEG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L0_NEG_A::VALUE2
    }
}
#[doc = "Write proxy for field `L0_NEG`"]
pub struct L0_NEG_W<'a> {
    w: &'a mut W,
}
impl<'a> L0_NEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L0_NEG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(L0_NEG_A::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(L0_NEG_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Latch0 positive edge"]
    #[inline(always)]
    pub fn l0_pos(&self) -> L0_POS_R {
        L0_POS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Latch0 negative edge"]
    #[inline(always)]
    pub fn l0_neg(&self) -> L0_NEG_R {
        L0_NEG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latch0 positive edge"]
    #[inline(always)]
    pub fn l0_pos(&mut self) -> L0_POS_W {
        L0_POS_W { w: self }
    }
    #[doc = "Bit 1 - Latch0 negative edge"]
    #[inline(always)]
    pub fn l0_neg(&mut self) -> L0_NEG_W {
        L0_NEG_W { w: self }
    }
}
