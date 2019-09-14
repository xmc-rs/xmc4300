#[doc = "Reader of register SDMMC_CON"]
pub type R = crate::R<u32, super::SDMMC_CON>;
#[doc = "Writer for register SDMMC_CON"]
pub type W = crate::W<u32, super::SDMMC_CON>;
#[doc = "Register SDMMC_CON `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_CON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SDMMC Write Protection Input Multiplexer Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPSEL_A {
    #[doc = "0: P1.1 input pin selected"]
    VALUE1,
    #[doc = "1: Software bit WPVAL is selected"]
    VALUE2,
}
impl From<WPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WPSEL_A) -> Self {
        match variant {
            WPSEL_A::VALUE1 => false,
            WPSEL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WPSEL`"]
pub type WPSEL_R = crate::R<bool, WPSEL_A>;
impl WPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPSEL_A {
        match self.bits {
            false => WPSEL_A::VALUE1,
            true => WPSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `WPSEL`"]
pub struct WPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "P1.1 input pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WPSEL_A::VALUE1)
    }
    #[doc = "Software bit WPVAL is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WPSEL_A::VALUE2)
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
#[doc = "SDMMC Write Protect Software Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPSVAL_A {
    #[doc = "0: No write protection"]
    VALUE1,
    #[doc = "1: Write protection active"]
    VALUE2,
}
impl From<WPSVAL_A> for bool {
    #[inline(always)]
    fn from(variant: WPSVAL_A) -> Self {
        match variant {
            WPSVAL_A::VALUE1 => false,
            WPSVAL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WPSVAL`"]
pub type WPSVAL_R = crate::R<bool, WPSVAL_A>;
impl WPSVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPSVAL_A {
        match self.bits {
            false => WPSVAL_A::VALUE1,
            true => WPSVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPSVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPSVAL_A::VALUE2
    }
}
#[doc = "Write proxy for field `WPSVAL`"]
pub struct WPSVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> WPSVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPSVAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WPSVAL_A::VALUE1)
    }
    #[doc = "Write protection active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WPSVAL_A::VALUE2)
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
#[doc = "SDMMC Card Detection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDSEL_A {
    #[doc = "0: P1.10 input pin selected"]
    VALUE1,
    #[doc = "1: Software bit CDSVAL is selected"]
    VALUE2,
}
impl From<CDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CDSEL_A) -> Self {
        match variant {
            CDSEL_A::VALUE1 => false,
            CDSEL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CDSEL`"]
pub type CDSEL_R = crate::R<bool, CDSEL_A>;
impl CDSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSEL_A {
        match self.bits {
            false => CDSEL_A::VALUE1,
            true => CDSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `CDSEL`"]
pub struct CDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "P1.10 input pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE1)
    }
    #[doc = "Software bit CDSVAL is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "SDMMC Write Protect Software Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDSVAL_A {
    #[doc = "0: No card detected"]
    VALUE1,
    #[doc = "1: Card detected"]
    VALUE2,
}
impl From<CDSVAL_A> for bool {
    #[inline(always)]
    fn from(variant: CDSVAL_A) -> Self {
        match variant {
            CDSVAL_A::VALUE1 => false,
            CDSVAL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CDSVAL`"]
pub type CDSVAL_R = crate::R<bool, CDSVAL_A>;
impl CDSVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSVAL_A {
        match self.bits {
            false => CDSVAL_A::VALUE1,
            true => CDSVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDSVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDSVAL_A::VALUE2
    }
}
#[doc = "Write proxy for field `CDSVAL`"]
pub struct CDSVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDSVAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No card detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSVAL_A::VALUE1)
    }
    #[doc = "Card detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSVAL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline(always)]
    pub fn wpsel(&self) -> WPSEL_R {
        WPSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn wpsval(&self) -> WPSVAL_R {
        WPSVAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline(always)]
    pub fn cdsel(&self) -> CDSEL_R {
        CDSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn cdsval(&self) -> CDSVAL_R {
        CDSVAL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline(always)]
    pub fn wpsel(&mut self) -> WPSEL_W {
        WPSEL_W { w: self }
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn wpsval(&mut self) -> WPSVAL_W {
        WPSVAL_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline(always)]
    pub fn cdsel(&mut self) -> CDSEL_W {
        CDSEL_W { w: self }
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn cdsval(&mut self) -> CDSVAL_W {
        CDSVAL_W { w: self }
    }
}
