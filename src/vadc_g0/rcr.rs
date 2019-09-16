#[doc = "Reader of register RCR[%s]"]
pub type R = crate::R<u32, super::RCR>;
#[doc = "Writer for register RCR[%s]"]
pub type W = crate::W<u32, super::RCR>;
#[doc = "Register RCR[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::RCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DRCTR`"]
pub type DRCTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRCTR`"]
pub struct DRCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DRCTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Data Modification Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMM_A {
    #[doc = "0: Standard data reduction (accumulation)"]
    VALUE1,
    #[doc = "1: Result filtering mode"]
    VALUE2,
    #[doc = "2: Difference mode"]
    VALUE3,
}
impl From<DMM_A> for u8 {
    #[inline(always)]
    fn from(variant: DMM_A) -> Self {
        match variant {
            DMM_A::VALUE1 => 0,
            DMM_A::VALUE2 => 1,
            DMM_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `DMM`"]
pub type DMM_R = crate::R<u8, DMM_A>;
impl DMM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMM_A::VALUE1),
            1 => Val(DMM_A::VALUE2),
            2 => Val(DMM_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DMM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DMM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DMM_A::VALUE3
    }
}
#[doc = "Write proxy for field `DMM`"]
pub struct DMM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Standard data reduction (accumulation)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMM_A::VALUE1)
    }
    #[doc = "Result filtering mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMM_A::VALUE2)
    }
    #[doc = "Difference mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DMM_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Wait-for-Read Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WFR_A {
    #[doc = "0: Overwrite mode"]
    VALUE1,
    #[doc = "1: Wait-for-read mode enabled for this register"]
    VALUE2,
}
impl From<WFR_A> for bool {
    #[inline(always)]
    fn from(variant: WFR_A) -> Self {
        match variant {
            WFR_A::VALUE1 => false,
            WFR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WFR`"]
pub type WFR_R = crate::R<bool, WFR_A>;
impl WFR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WFR_A {
        match self.bits {
            false => WFR_A::VALUE1,
            true => WFR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WFR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WFR_A::VALUE2
    }
}
#[doc = "Write proxy for field `WFR`"]
pub struct WFR_W<'a> {
    w: &'a mut W,
}
impl<'a> WFR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WFR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Overwrite mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WFR_A::VALUE1)
    }
    #[doc = "Wait-for-read mode enabled for this register"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WFR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "FIFO Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEN_A {
    #[doc = "0: Separate result register"]
    VALUE1,
    #[doc = "1: Part of a FIFO structure: copy each new valid result"]
    VALUE2,
}
impl From<FEN_A> for u8 {
    #[inline(always)]
    fn from(variant: FEN_A) -> Self {
        match variant {
            FEN_A::VALUE1 => 0,
            FEN_A::VALUE2 => 1,
        }
    }
}
#[doc = "Reader of field `FEN`"]
pub type FEN_R = crate::R<u8, FEN_A>;
impl FEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FEN_A::VALUE1),
            1 => Val(FEN_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `FEN`"]
pub struct FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Separate result register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FEN_A::VALUE1)
    }
    #[doc = "Part of a FIFO structure: copy each new valid result"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FEN_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Service Request Generation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRGEN_A {
    #[doc = "0: No service request"]
    VALUE1,
    #[doc = "1: Service request after a result event"]
    VALUE2,
}
impl From<SRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRGEN_A) -> Self {
        match variant {
            SRGEN_A::VALUE1 => false,
            SRGEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SRGEN`"]
pub type SRGEN_R = crate::R<bool, SRGEN_A>;
impl SRGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRGEN_A {
        match self.bits {
            false => SRGEN_A::VALUE1,
            true => SRGEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRGEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRGEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `SRGEN`"]
pub struct SRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No service request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRGEN_A::VALUE1)
    }
    #[doc = "Service request after a result event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRGEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - Data Reduction Control"]
    #[inline(always)]
    pub fn drctr(&self) -> DRCTR_R {
        DRCTR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Data Modification Mode"]
    #[inline(always)]
    pub fn dmm(&self) -> DMM_R {
        DMM_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Wait-for-Read Mode Enable"]
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - FIFO Mode Enable"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Service Request Generation Enable"]
    #[inline(always)]
    pub fn srgen(&self) -> SRGEN_R {
        SRGEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19 - Data Reduction Control"]
    #[inline(always)]
    pub fn drctr(&mut self) -> DRCTR_W {
        DRCTR_W { w: self }
    }
    #[doc = "Bits 20:21 - Data Modification Mode"]
    #[inline(always)]
    pub fn dmm(&mut self) -> DMM_W {
        DMM_W { w: self }
    }
    #[doc = "Bit 24 - Wait-for-Read Mode Enable"]
    #[inline(always)]
    pub fn wfr(&mut self) -> WFR_W {
        WFR_W { w: self }
    }
    #[doc = "Bits 25:26 - FIFO Mode Enable"]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W { w: self }
    }
    #[doc = "Bit 31 - Service Request Generation Enable"]
    #[inline(always)]
    pub fn srgen(&mut self) -> SRGEN_W {
        SRGEN_W { w: self }
    }
}
