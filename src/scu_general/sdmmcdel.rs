#[doc = "Reader of register SDMMCDEL"]
pub type R = crate::R<u32, super::SDMMCDEL>;
#[doc = "Writer for register SDMMCDEL"]
pub type W = crate::W<u32, super::SDMMCDEL>;
#[doc = "Register SDMMCDEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMCDEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable delay on the CMD/DAT out lines\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAPEN_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<TAPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TAPEN_A) -> Self {
        match variant {
            TAPEN_A::VALUE1 => false,
            TAPEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TAPEN`"]
pub type TAPEN_R = crate::R<bool, TAPEN_A>;
impl TAPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAPEN_A {
        match self.bits {
            false => TAPEN_A::VALUE1,
            true => TAPEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TAPEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TAPEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `TAPEN`"]
pub struct TAPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TAPEN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TAPEN_A::VALUE2)
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
#[doc = "Reader of field `TAPDEL`"]
pub type TAPDEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TAPDEL`"]
pub struct TAPDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPDEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable delay on the CMD/DAT out lines"]
    #[inline(always)]
    pub fn tapen(&self) -> TAPEN_R {
        TAPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Number of Delay Elements Select"]
    #[inline(always)]
    pub fn tapdel(&self) -> TAPDEL_R {
        TAPDEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable delay on the CMD/DAT out lines"]
    #[inline(always)]
    pub fn tapen(&mut self) -> TAPEN_W {
        TAPEN_W { w: self }
    }
    #[doc = "Bits 4:7 - Number of Delay Elements Select"]
    #[inline(always)]
    pub fn tapdel(&mut self) -> TAPDEL_W {
        TAPDEL_W { w: self }
    }
}
