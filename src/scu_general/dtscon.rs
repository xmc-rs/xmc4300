#[doc = "Reader of register DTSCON"]
pub type R = crate::R<u32, super::DTSCON>;
#[doc = "Writer for register DTSCON"]
pub type W = crate::W<u32, super::DTSCON>;
#[doc = "Register DTSCON `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DTSCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Sensor Power Down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWD_A {
    #[doc = "0: The DTS is powered"]
    CONST_0,
    #[doc = "1: The DTS is not powered"]
    CONST_1,
}
impl From<PWD_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_A) -> Self {
        match variant {
            PWD_A::CONST_0 => false,
            PWD_A::CONST_1 => true,
        }
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
    #[doc = "The DTS is powered"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PWD_A::CONST_0)
    }
    #[doc = "The DTS is not powered"]
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
#[doc = "Sensor Measurement Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_AW {
    #[doc = "0: No DTS measurement is started"]
    CONST_0,
    #[doc = "1: DTS measurement is started"]
    CONST_1,
}
impl From<START_AW> for bool {
    #[inline(always)]
    fn from(variant: START_AW) -> Self {
        match variant {
            START_AW::CONST_0 => false,
            START_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DTS measurement is started"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(START_AW::CONST_0)
    }
    #[doc = "DTS measurement is started"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(START_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `OFFSET`"]
pub type OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFFSET`"]
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 4)) | (((value as u32) & 0x7f) << 4);
        self.w
    }
}
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 11)) | (((value as u32) & 0x3f) << 11);
        self.w
    }
}
#[doc = "Reader of field `REFTRIM`"]
pub type REFTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFTRIM`"]
pub struct REFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> REFTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `BGTRIM`"]
pub type BGTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BGTRIM`"]
pub struct BGTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BGTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline(always)]
    pub fn pwd(&self) -> PWD_R {
        PWD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline(always)]
    pub fn reftrim(&self) -> REFTRIM_R {
        REFTRIM_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline(always)]
    pub fn bgtrim(&self) -> BGTRIM_R {
        BGTRIM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline(always)]
    pub fn pwd(&mut self) -> PWD_W {
        PWD_W { w: self }
    }
    #[doc = "Bit 1 - Sensor Measurement Start"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline(always)]
    pub fn reftrim(&mut self) -> REFTRIM_W {
        REFTRIM_W { w: self }
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline(always)]
    pub fn bgtrim(&mut self) -> BGTRIM_W {
        BGTRIM_W { w: self }
    }
}
