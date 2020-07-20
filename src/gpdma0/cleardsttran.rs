#[doc = "Writer for register CLEARDSTTRAN"]
pub type W = crate::W<u32, super::CLEARDSTTRAN>;
#[doc = "Register CLEARDSTTRAN `reset()`'s with value 0"]
impl crate::ResetValue for super::CLEARDSTTRAN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH0`"]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0_AW::VALUE2)
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
#[doc = "Clear Interrupt Status and Raw Status for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH1`"]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1_AW::VALUE2)
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
#[doc = "Clear Interrupt Status and Raw Status for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH2`"]
pub struct CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH3`"]
pub struct CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3_AW::VALUE2)
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
#[doc = "Clear Interrupt Status and Raw Status for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH4_AW> for bool {
    #[inline(always)]
    fn from(variant: CH4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH4`"]
pub struct CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH4_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH4_AW::VALUE2)
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
#[doc = "Clear Interrupt Status and Raw Status for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH5_AW> for bool {
    #[inline(always)]
    fn from(variant: CH5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH5`"]
pub struct CH5_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH5_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH5_AW::VALUE2)
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
#[doc = "Clear Interrupt Status and Raw Status for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH6_AW> for bool {
    #[inline(always)]
    fn from(variant: CH6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH6`"]
pub struct CH6_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH6_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH6_AW::VALUE2)
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
#[doc = "Clear Interrupt Status and Raw Status for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH7_AW> for bool {
    #[inline(always)]
    fn from(variant: CH7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH7`"]
pub struct CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH7_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH7_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear Interrupt Status and Raw Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bit 1 - Clear Interrupt Status and Raw Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bit 2 - Clear Interrupt Status and Raw Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bit 3 - Clear Interrupt Status and Raw Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W { w: self }
    }
    #[doc = "Bit 4 - Clear Interrupt Status and Raw Status for channel 4"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W {
        CH4_W { w: self }
    }
    #[doc = "Bit 5 - Clear Interrupt Status and Raw Status for channel 5"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W {
        CH5_W { w: self }
    }
    #[doc = "Bit 6 - Clear Interrupt Status and Raw Status for channel 6"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W {
        CH6_W { w: self }
    }
    #[doc = "Bit 7 - Clear Interrupt Status and Raw Status for channel 7"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W {
        CH7_W { w: self }
    }
}
