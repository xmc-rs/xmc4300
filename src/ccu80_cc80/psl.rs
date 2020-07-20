#[doc = "Reader of register PSL"]
pub type R = crate::R<u32, super::PSL>;
#[doc = "Writer for register PSL"]
pub type W = crate::W<u32, super::PSL>;
#[doc = "Register PSL `reset()`'s with value 0"]
impl crate::ResetValue for super::PSL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output Passive Level for CCU8x.OUTy0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL11_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL11_A> for bool {
    #[inline(always)]
    fn from(variant: PSL11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PSL11`"]
pub type PSL11_R = crate::R<bool, PSL11_A>;
impl PSL11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL11_A {
        match self.bits {
            false => PSL11_A::VALUE1,
            true => PSL11_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL11_A::VALUE2
    }
}
#[doc = "Write proxy for field `PSL11`"]
pub struct PSL11_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL11_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL11_A::VALUE2)
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
#[doc = "Output Passive Level for CCU8x.OUTy1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL12_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL12_A> for bool {
    #[inline(always)]
    fn from(variant: PSL12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PSL12`"]
pub type PSL12_R = crate::R<bool, PSL12_A>;
impl PSL12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL12_A {
        match self.bits {
            false => PSL12_A::VALUE1,
            true => PSL12_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL12_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL12_A::VALUE2
    }
}
#[doc = "Write proxy for field `PSL12`"]
pub struct PSL12_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL12_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL12_A::VALUE2)
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
#[doc = "Output Passive Level for CCU8x.OUTy2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL21_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL21_A> for bool {
    #[inline(always)]
    fn from(variant: PSL21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PSL21`"]
pub type PSL21_R = crate::R<bool, PSL21_A>;
impl PSL21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL21_A {
        match self.bits {
            false => PSL21_A::VALUE1,
            true => PSL21_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL21_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL21_A::VALUE2
    }
}
#[doc = "Write proxy for field `PSL21`"]
pub struct PSL21_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL21_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL21_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Output Passive Level for CCU8x.OUTy3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL22_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL22_A> for bool {
    #[inline(always)]
    fn from(variant: PSL22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PSL22`"]
pub type PSL22_R = crate::R<bool, PSL22_A>;
impl PSL22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL22_A {
        match self.bits {
            false => PSL22_A::VALUE1,
            true => PSL22_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL22_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL22_A::VALUE2
    }
}
#[doc = "Write proxy for field `PSL22`"]
pub struct PSL22_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL22_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL22_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Output Passive Level for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn psl11(&self) -> PSL11_R {
        PSL11_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output Passive Level for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn psl12(&self) -> PSL12_R {
        PSL12_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output Passive Level for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn psl21(&self) -> PSL21_R {
        PSL21_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Passive Level for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn psl22(&self) -> PSL22_R {
        PSL22_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Passive Level for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn psl11(&mut self) -> PSL11_W {
        PSL11_W { w: self }
    }
    #[doc = "Bit 1 - Output Passive Level for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn psl12(&mut self) -> PSL12_W {
        PSL12_W { w: self }
    }
    #[doc = "Bit 2 - Output Passive Level for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn psl21(&mut self) -> PSL21_W {
        PSL21_W { w: self }
    }
    #[doc = "Bit 3 - Output Passive Level for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn psl22(&mut self) -> PSL22_W {
        PSL22_W { w: self }
    }
}
