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
#[doc = "Output Passive Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL_A> for bool {
    #[inline(always)]
    fn from(variant: PSL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PSL`"]
pub type PSL_R = crate::R<bool, PSL_A>;
impl PSL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL_A {
        match self.bits {
            false => PSL_A::VALUE1,
            true => PSL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL_A::VALUE2
    }
}
#[doc = "Write proxy for field `PSL`"]
pub struct PSL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Output Passive Level"]
    #[inline(always)]
    pub fn psl(&self) -> PSL_R {
        PSL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Passive Level"]
    #[inline(always)]
    pub fn psl(&mut self) -> PSL_W {
        PSL_W { w: self }
    }
}
