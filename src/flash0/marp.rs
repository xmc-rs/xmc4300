#[doc = "Reader of register MARP"]
pub type R = crate::R<u32, super::MARP>;
#[doc = "Writer for register MARP"]
pub type W = crate::W<u32, super::MARP>;
#[doc = "Register MARP `reset()`'s with value 0"]
impl crate::ResetValue for super::MARP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PFLASH Margin Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MARGIN_A {
    #[doc = "0: Standard (default) margin."]
    VALUE1 = 0,
    #[doc = "1: Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    VALUE2 = 1,
    #[doc = "4: Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    VALUE3 = 4,
}
impl From<MARGIN_A> for u8 {
    #[inline(always)]
    fn from(variant: MARGIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MARGIN`"]
pub type MARGIN_R = crate::R<u8, MARGIN_A>;
impl MARGIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MARGIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MARGIN_A::VALUE1),
            1 => Val(MARGIN_A::VALUE2),
            4 => Val(MARGIN_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MARGIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MARGIN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MARGIN_A::VALUE3
    }
}
#[doc = "Write proxy for field `MARGIN`"]
pub struct MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MARGIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MARGIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Standard (default) margin."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MARGIN_A::VALUE1)
    }
    #[doc = "Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MARGIN_A::VALUE2)
    }
    #[doc = "Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MARGIN_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "PFLASH Double-Bit Error Trap Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRAPDIS_A {
    #[doc = "0: If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    VALUE1 = 0,
    #[doc = "1: The double-bit error trap is disabled. Shall be used only during margin check"]
    VALUE2 = 1,
}
impl From<TRAPDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TRAPDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRAPDIS`"]
pub type TRAPDIS_R = crate::R<bool, TRAPDIS_A>;
impl TRAPDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRAPDIS_A {
        match self.bits {
            false => TRAPDIS_A::VALUE1,
            true => TRAPDIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRAPDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRAPDIS_A::VALUE2
    }
}
#[doc = "Write proxy for field `TRAPDIS`"]
pub struct TRAPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAPDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRAPDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRAPDIS_A::VALUE1)
    }
    #[doc = "The double-bit error trap is disabled. Shall be used only during margin check"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRAPDIS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PFLASH Margin Selection"]
    #[inline(always)]
    pub fn margin(&self) -> MARGIN_R {
        MARGIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PFLASH Double-Bit Error Trap Disable"]
    #[inline(always)]
    pub fn trapdis(&self) -> TRAPDIS_R {
        TRAPDIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PFLASH Margin Selection"]
    #[inline(always)]
    pub fn margin(&mut self) -> MARGIN_W {
        MARGIN_W { w: self }
    }
    #[doc = "Bit 15 - PFLASH Double-Bit Error Trap Disable"]
    #[inline(always)]
    pub fn trapdis(&mut self) -> TRAPDIS_W {
        TRAPDIS_W { w: self }
    }
}
