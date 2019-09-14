#[doc = "Reader of register STS"]
pub type R = crate::R<u32, super::STS>;
#[doc = "Writer for register STS"]
pub type W = crate::W<u32, super::STS>;
#[doc = "Register STS `reset()`'s with value 0"]
impl crate::ResetValue for super::STS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Bufferable Write Access Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WERR_A {
    #[doc = "0: no write error occurred."]
    VALUE1,
    #[doc = "1: write error occurred, interrupt request is pending."]
    VALUE2,
}
impl From<WERR_A> for bool {
    #[inline(always)]
    fn from(variant: WERR_A) -> Self {
        match variant {
            WERR_A::VALUE1 => false,
            WERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WERR`"]
pub type WERR_R = crate::R<bool, WERR_A>;
impl WERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WERR_A {
        match self.bits {
            false => WERR_A::VALUE1,
            true => WERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `WERR`"]
pub struct WERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no write error occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WERR_A::VALUE1)
    }
    #[doc = "write error occurred, interrupt request is pending."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WERR_A::VALUE2)
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
    #[doc = "Bit 0 - Bufferable Write Access Error"]
    #[inline(always)]
    pub fn werr(&self) -> WERR_R {
        WERR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bufferable Write Access Error"]
    #[inline(always)]
    pub fn werr(&mut self) -> WERR_W {
        WERR_W { w: self }
    }
}
