#[doc = "Reader of register PLLCON2"]
pub type R = crate::R<u32, super::PLLCON2>;
#[doc = "Writer for register PLLCON2"]
pub type W = crate::W<u32, super::PLLCON2>;
#[doc = "Register PLLCON2 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::PLLCON2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "P-Divider Input Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINSEL_A {
    #[doc = "0: PLL external oscillator selected"]
    CONST_0,
    #[doc = "1: Backup clock fofi selected"]
    CONST_1,
}
impl From<PINSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PINSEL_A) -> Self {
        match variant {
            PINSEL_A::CONST_0 => false,
            PINSEL_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PINSEL`"]
pub type PINSEL_R = crate::R<bool, PINSEL_A>;
impl PINSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINSEL_A {
        match self.bits {
            false => PINSEL_A::CONST_0,
            true => PINSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PINSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PINSEL_A::CONST_1
    }
}
#[doc = "Write proxy for field `PINSEL`"]
pub struct PINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PINSEL_A::CONST_0)
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PINSEL_A::CONST_1)
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
#[doc = "K1-Divider Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum K1INSEL_A {
    #[doc = "0: PLL external oscillator selected"]
    CONST_0,
    #[doc = "1: Backup clock fofi selected"]
    CONST_1,
}
impl From<K1INSEL_A> for bool {
    #[inline(always)]
    fn from(variant: K1INSEL_A) -> Self {
        match variant {
            K1INSEL_A::CONST_0 => false,
            K1INSEL_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `K1INSEL`"]
pub type K1INSEL_R = crate::R<bool, K1INSEL_A>;
impl K1INSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> K1INSEL_A {
        match self.bits {
            false => K1INSEL_A::CONST_0,
            true => K1INSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == K1INSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == K1INSEL_A::CONST_1
    }
}
#[doc = "Write proxy for field `K1INSEL`"]
pub struct K1INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> K1INSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: K1INSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(K1INSEL_A::CONST_0)
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(K1INSEL_A::CONST_1)
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
impl R {
    #[doc = "Bit 0 - P-Divider Input Selection"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - K1-Divider Input Selection"]
    #[inline(always)]
    pub fn k1insel(&self) -> K1INSEL_R {
        K1INSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P-Divider Input Selection"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W {
        PINSEL_W { w: self }
    }
    #[doc = "Bit 8 - K1-Divider Input Selection"]
    #[inline(always)]
    pub fn k1insel(&mut self) -> K1INSEL_W {
        K1INSEL_W { w: self }
    }
}
