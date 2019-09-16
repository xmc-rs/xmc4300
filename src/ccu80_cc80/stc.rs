#[doc = "Reader of register STC"]
pub type R = crate::R<u32, super::STC>;
#[doc = "Writer for register STC"]
pub type W = crate::W<u32, super::STC>;
#[doc = "Register STC `reset()`'s with value 0"]
impl crate::ResetValue for super::STC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Cascaded shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSE_A {
    #[doc = "0: Cascaded shadow transfer disabled"]
    VALUE1,
    #[doc = "1: Cascaded shadow transfer enabled"]
    VALUE2,
}
impl From<CSE_A> for bool {
    #[inline(always)]
    fn from(variant: CSE_A) -> Self {
        match variant {
            CSE_A::VALUE1 => false,
            CSE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CSE`"]
pub type CSE_R = crate::R<bool, CSE_A>;
impl CSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSE_A {
        match self.bits {
            false => CSE_A::VALUE1,
            true => CSE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CSE_A::VALUE2
    }
}
#[doc = "Write proxy for field `CSE`"]
pub struct CSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cascaded shadow transfer disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSE_A::VALUE1)
    }
    #[doc = "Cascaded shadow transfer enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSE_A::VALUE2)
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
#[doc = "Shadow transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STM_A {
    #[doc = "0: Shadow transfer is done in Period Match and One match."]
    VALUE1,
    #[doc = "1: Shadow transfer is done only in Period Match."]
    VALUE2,
    #[doc = "2: Shadow transfer is done only in One Match."]
    VALUE3,
}
impl From<STM_A> for u8 {
    #[inline(always)]
    fn from(variant: STM_A) -> Self {
        match variant {
            STM_A::VALUE1 => 0,
            STM_A::VALUE2 => 1,
            STM_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `STM`"]
pub type STM_R = crate::R<u8, STM_A>;
impl STM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STM_A::VALUE1),
            1 => Val(STM_A::VALUE2),
            2 => Val(STM_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STM_A::VALUE3
    }
}
#[doc = "Write proxy for field `STM`"]
pub struct STM_W<'a> {
    w: &'a mut W,
}
impl<'a> STM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Shadow transfer is done in Period Match and One match."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STM_A::VALUE1)
    }
    #[doc = "Shadow transfer is done only in Period Match."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STM_A::VALUE2)
    }
    #[doc = "Shadow transfer is done only in One Match."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STM_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cascaded shadow transfer enable"]
    #[inline(always)]
    pub fn cse(&self) -> CSE_R {
        CSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Shadow transfer mode"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cascaded shadow transfer enable"]
    #[inline(always)]
    pub fn cse(&mut self) -> CSE_W {
        CSE_W { w: self }
    }
    #[doc = "Bits 1:2 - Shadow transfer mode"]
    #[inline(always)]
    pub fn stm(&mut self) -> STM_W {
        STM_W { w: self }
    }
}
