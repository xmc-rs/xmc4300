#[doc = "Reader of register SYNCTR"]
pub type R = crate::R<u32, super::SYNCTR>;
#[doc = "Writer for register SYNCTR"]
pub type W = crate::W<u32, super::SYNCTR>;
#[doc = "Register SYNCTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STSEL_A {
    #[doc = "0: Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    VALUE1 = 0,
    #[doc = "1: Kernel is synchronization slave: Control information from input CI1"]
    VALUE2 = 1,
    #[doc = "2: Kernel is synchronization slave: Control information from input CI2"]
    VALUE3 = 2,
    #[doc = "3: Kernel is synchronization slave: Control information from input CI3"]
    VALUE4 = 3,
}
impl From<STSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: STSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STSEL`"]
pub type STSEL_R = crate::R<u8, STSEL_A>;
impl STSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STSEL_A {
        match self.bits {
            0 => STSEL_A::VALUE1,
            1 => STSEL_A::VALUE2,
            2 => STSEL_A::VALUE3,
            3 => STSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STSEL_A::VALUE4
    }
}
#[doc = "Write proxy for field `STSEL`"]
pub struct STSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STSEL_A::VALUE1)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STSEL_A::VALUE2)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STSEL_A::VALUE3)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STSEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVALR1_A {
    #[doc = "0: No ready input control"]
    VALUE1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    VALUE2 = 1,
}
impl From<EVALR1_A> for bool {
    #[inline(always)]
    fn from(variant: EVALR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVALR1`"]
pub type EVALR1_R = crate::R<bool, EVALR1_A>;
impl EVALR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVALR1_A {
        match self.bits {
            false => EVALR1_A::VALUE1,
            true => EVALR1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EVALR1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EVALR1_A::VALUE2
    }
}
#[doc = "Write proxy for field `EVALR1`"]
pub struct EVALR1_W<'a> {
    w: &'a mut W,
}
impl<'a> EVALR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVALR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EVALR1_A::VALUE1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EVALR1_A::VALUE2)
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
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVALR2_A {
    #[doc = "0: No ready input control"]
    VALUE1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    VALUE2 = 1,
}
impl From<EVALR2_A> for bool {
    #[inline(always)]
    fn from(variant: EVALR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVALR2`"]
pub type EVALR2_R = crate::R<bool, EVALR2_A>;
impl EVALR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVALR2_A {
        match self.bits {
            false => EVALR2_A::VALUE1,
            true => EVALR2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EVALR2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EVALR2_A::VALUE2
    }
}
#[doc = "Write proxy for field `EVALR2`"]
pub struct EVALR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EVALR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVALR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EVALR2_A::VALUE1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EVALR2_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVALR3_A {
    #[doc = "0: No ready input control"]
    VALUE1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    VALUE2 = 1,
}
impl From<EVALR3_A> for bool {
    #[inline(always)]
    fn from(variant: EVALR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVALR3`"]
pub type EVALR3_R = crate::R<bool, EVALR3_A>;
impl EVALR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVALR3_A {
        match self.bits {
            false => EVALR3_A::VALUE1,
            true => EVALR3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EVALR3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EVALR3_A::VALUE2
    }
}
#[doc = "Write proxy for field `EVALR3`"]
pub struct EVALR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EVALR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVALR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EVALR3_A::VALUE1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EVALR3_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Start Selection"]
    #[inline(always)]
    pub fn stsel(&self) -> STSEL_R {
        STSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr1(&self) -> EVALR1_R {
        EVALR1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr2(&self) -> EVALR2_R {
        EVALR2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr3(&self) -> EVALR3_R {
        EVALR3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Start Selection"]
    #[inline(always)]
    pub fn stsel(&mut self) -> STSEL_W {
        STSEL_W { w: self }
    }
    #[doc = "Bit 4 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr1(&mut self) -> EVALR1_W {
        EVALR1_W { w: self }
    }
    #[doc = "Bit 5 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr2(&mut self) -> EVALR2_W {
        EVALR2_W { w: self }
    }
    #[doc = "Bit 6 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr3(&mut self) -> EVALR3_W {
        EVALR3_W { w: self }
    }
}
