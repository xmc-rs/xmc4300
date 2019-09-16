#[doc = "Reader of register CLC"]
pub type R = crate::R<u32, super::CLC>;
#[doc = "Writer for register CLC"]
pub type W = crate::W<u32, super::CLC>;
#[doc = "Register CLC `reset()`'s with value 0x03"]
impl crate::ResetValue for super::CLC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Module Disable Request Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISR_A {
    #[doc = "0: On request: enable the module clock"]
    VALUE1,
    #[doc = "1: Off request: stop the module clock"]
    VALUE2,
}
impl From<DISR_A> for bool {
    #[inline(always)]
    fn from(variant: DISR_A) -> Self {
        match variant {
            DISR_A::VALUE1 => false,
            DISR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DISR`"]
pub type DISR_R = crate::R<bool, DISR_A>;
impl DISR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISR_A {
        match self.bits {
            false => DISR_A::VALUE1,
            true => DISR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DISR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DISR_A::VALUE2
    }
}
#[doc = "Write proxy for field `DISR`"]
pub struct DISR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "On request: enable the module clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DISR_A::VALUE1)
    }
    #[doc = "Off request: stop the module clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DISR_A::VALUE2)
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
#[doc = "Module Disable Status Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISS_A {
    #[doc = "0: Module clock is enabled"]
    VALUE1,
    #[doc = "1: Off: module is not clocked"]
    VALUE2,
}
impl From<DISS_A> for bool {
    #[inline(always)]
    fn from(variant: DISS_A) -> Self {
        match variant {
            DISS_A::VALUE1 => false,
            DISS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DISS`"]
pub type DISS_R = crate::R<bool, DISS_A>;
impl DISS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISS_A {
        match self.bits {
            false => DISS_A::VALUE1,
            true => DISS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DISS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DISS_A::VALUE2
    }
}
#[doc = "Sleep Mode Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDIS_A {
    #[doc = "0: Sleep mode request is enabled and functional"]
    VALUE1,
    #[doc = "1: Module disregards the sleep mode control signal"]
    VALUE2,
}
impl From<EDIS_A> for bool {
    #[inline(always)]
    fn from(variant: EDIS_A) -> Self {
        match variant {
            EDIS_A::VALUE1 => false,
            EDIS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EDIS`"]
pub type EDIS_R = crate::R<bool, EDIS_A>;
impl EDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDIS_A {
        match self.bits {
            false => EDIS_A::VALUE1,
            true => EDIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EDIS_A::VALUE2
    }
}
#[doc = "Write proxy for field `EDIS`"]
pub struct EDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sleep mode request is enabled and functional"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EDIS_A::VALUE1)
    }
    #[doc = "Module disregards the sleep mode control signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EDIS_A::VALUE2)
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
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&self) -> DISR_R {
        DISR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Module Disable Status Bit"]
    #[inline(always)]
    pub fn diss(&self) -> DISS_R {
        DISS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    pub fn edis(&self) -> EDIS_R {
        EDIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&mut self) -> DISR_W {
        DISR_W { w: self }
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    pub fn edis(&mut self) -> EDIS_W {
        EDIS_W { w: self }
    }
}
