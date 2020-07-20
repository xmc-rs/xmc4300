#[doc = "Reader of register OSCSICTRL"]
pub type R = crate::R<u32, super::OSCSICTRL>;
#[doc = "Writer for register OSCSICTRL"]
pub type W = crate::W<u32, super::OSCSICTRL>;
#[doc = "Register OSCSICTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::OSCSICTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Turn OFF the fOSI Clock Source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWD_A {
    #[doc = "0: Enabled"]
    CONST_0 = 0,
    #[doc = "1: Disabled"]
    CONST_1 = 1,
}
impl From<PWD_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWD`"]
pub type PWD_R = crate::R<bool, PWD_A>;
impl PWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_A {
        match self.bits {
            false => PWD_A::CONST_0,
            true => PWD_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PWD_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PWD_A::CONST_1
    }
}
#[doc = "Write proxy for field `PWD`"]
pub struct PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PWD_A::CONST_0)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PWD_A::CONST_1)
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
    #[doc = "Bit 0 - Turn OFF the fOSI Clock Source"]
    #[inline(always)]
    pub fn pwd(&self) -> PWD_R {
        PWD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Turn OFF the fOSI Clock Source"]
    #[inline(always)]
    pub fn pwd(&mut self) -> PWD_W {
        PWD_W { w: self }
    }
}
