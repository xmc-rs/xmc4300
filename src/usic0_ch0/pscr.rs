#[doc = "Writer for register PSCR"]
pub type W = crate::W<u32, super::PSCR>;
#[doc = "Register PSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear Status Flag 0 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST0_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2,
}
impl From<CST0_AW> for bool {
    #[inline(always)]
    fn from(variant: CST0_AW) -> Self {
        match variant {
            CST0_AW::VALUE1 => false,
            CST0_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CST0`"]
pub struct CST0_W<'a> {
    w: &'a mut W,
}
impl<'a> CST0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CST0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST0_AW::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST0_AW::VALUE2)
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
#[doc = "Clear Status Flag 1 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST1_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2,
}
impl From<CST1_AW> for bool {
    #[inline(always)]
    fn from(variant: CST1_AW) -> Self {
        match variant {
            CST1_AW::VALUE1 => false,
            CST1_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CST1`"]
pub struct CST1_W<'a> {
    w: &'a mut W,
}
impl<'a> CST1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CST1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST1_AW::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST1_AW::VALUE2)
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
#[doc = "Clear Status Flag 2 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST2_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2,
}
impl From<CST2_AW> for bool {
    #[inline(always)]
    fn from(variant: CST2_AW) -> Self {
        match variant {
            CST2_AW::VALUE1 => false,
            CST2_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CST2`"]
pub struct CST2_W<'a> {
    w: &'a mut W,
}
impl<'a> CST2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CST2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST2_AW::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST2_AW::VALUE2)
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
#[doc = "Clear Status Flag 3 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST3_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2,
}
impl From<CST3_AW> for bool {
    #[inline(always)]
    fn from(variant: CST3_AW) -> Self {
        match variant {
            CST3_AW::VALUE1 => false,
            CST3_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CST3`"]
pub struct CST3_W<'a> {
    w: &'a mut W,
}
impl<'a> CST3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CST3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST3_AW::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST3_AW::VALUE2)
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
#[doc = "Clear Status Flag 4 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST4_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2,
}
impl From<CST4_AW> for bool {
    #[inline(always)]
    fn from(variant: CST4_AW) -> Self {
        match variant {
            CST4_AW::VALUE1 => false,
            CST4_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CST4`"]
pub struct CST4_W<'a> {
    w: &'a mut W,
}
impl<'a> CST4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CST4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST4_AW::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST4_AW::VALUE2)
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
#[doc = "Clear Status Flag 5 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST5_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2,
}
impl From<CST5_AW> for bool {
    #[inline(always)]
    fn from(variant: CST5_AW) -> Self {
        match variant {
            CST5_AW::VALUE1 => false,
            CST5_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CST5`"]
pub struct CST5_W<'a> {
    w: &'a mut W,
}
impl<'a> CST5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CST5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST5_AW::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST5_AW::VALUE2)
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
#[doc = "Clear Status Flag 6 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST6_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2,
}
impl From<CST6_AW> for bool {
    #[inline(always)]
    fn from(variant: CST6_AW) -> Self {
        match variant {
            CST6_AW::VALUE1 => false,
            CST6_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CST6`"]
pub struct CST6_W<'a> {
    w: &'a mut W,
}
impl<'a> CST6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CST6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST6_AW::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST6_AW::VALUE2)
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
#[doc = "Clear Status Flag 7 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST7_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2,
}
impl From<CST7_AW> for bool {
    #[inline(always)]
    fn from(variant: CST7_AW) -> Self {
        match variant {
            CST7_AW::VALUE1 => false,
            CST7_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CST7`"]
pub struct CST7_W<'a> {
    w: &'a mut W,
}
impl<'a> CST7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CST7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST7_AW::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST7_AW::VALUE2)
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
#[doc = "Clear Status Flag 8 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST8_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2,
}
impl From<CST8_AW> for bool {
    #[inline(always)]
    fn from(variant: CST8_AW) -> Self {
        match variant {
            CST8_AW::VALUE1 => false,
            CST8_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CST8`"]
pub struct CST8_W<'a> {
    w: &'a mut W,
}
impl<'a> CST8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CST8_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST8_AW::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST8_AW::VALUE2)
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
#[doc = "Clear Status Flag 9 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST9_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2,
}
impl From<CST9_AW> for bool {
    #[inline(always)]
    fn from(variant: CST9_AW) -> Self {
        match variant {
            CST9_AW::VALUE1 => false,
            CST9_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CST9`"]
pub struct CST9_W<'a> {
    w: &'a mut W,
}
impl<'a> CST9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CST9_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST9_AW::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST9_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Clear Receiver Start Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSIF_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.RSIF is cleared."]
    VALUE2,
}
impl From<CRSIF_AW> for bool {
    #[inline(always)]
    fn from(variant: CRSIF_AW) -> Self {
        match variant {
            CRSIF_AW::VALUE1 => false,
            CRSIF_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CRSIF`"]
pub struct CRSIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRSIF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRSIF_AW::VALUE1)
    }
    #[doc = "Flag PSR.RSIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRSIF_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Clear Data Lost Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDLIF_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.DLIF is cleared."]
    VALUE2,
}
impl From<CDLIF_AW> for bool {
    #[inline(always)]
    fn from(variant: CDLIF_AW) -> Self {
        match variant {
            CDLIF_AW::VALUE1 => false,
            CDLIF_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CDLIF`"]
pub struct CDLIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDLIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDLIF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDLIF_AW::VALUE1)
    }
    #[doc = "Flag PSR.DLIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDLIF_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Clear Transmit Shift Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIF_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.TSIF is cleared."]
    VALUE2,
}
impl From<CTSIF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSIF_AW) -> Self {
        match variant {
            CTSIF_AW::VALUE1 => false,
            CTSIF_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CTSIF`"]
pub struct CTSIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSIF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTSIF_AW::VALUE1)
    }
    #[doc = "Flag PSR.TSIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTSIF_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Clear Transmit Buffer Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTBIF_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.TBIF is cleared."]
    VALUE2,
}
impl From<CTBIF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTBIF_AW) -> Self {
        match variant {
            CTBIF_AW::VALUE1 => false,
            CTBIF_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CTBIF`"]
pub struct CTBIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTBIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTBIF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTBIF_AW::VALUE1)
    }
    #[doc = "Flag PSR.TBIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTBIF_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Clear Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRIF_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.RIF is cleared."]
    VALUE2,
}
impl From<CRIF_AW> for bool {
    #[inline(always)]
    fn from(variant: CRIF_AW) -> Self {
        match variant {
            CRIF_AW::VALUE1 => false,
            CRIF_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CRIF`"]
pub struct CRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRIF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRIF_AW::VALUE1)
    }
    #[doc = "Flag PSR.RIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRIF_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Clear Alternative Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAIF_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.AIF is cleared."]
    VALUE2,
}
impl From<CAIF_AW> for bool {
    #[inline(always)]
    fn from(variant: CAIF_AW) -> Self {
        match variant {
            CAIF_AW::VALUE1 => false,
            CAIF_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CAIF`"]
pub struct CAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAIF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CAIF_AW::VALUE1)
    }
    #[doc = "Flag PSR.AIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CAIF_AW::VALUE2)
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
#[doc = "Clear Baud Rate Generator Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBRGIF_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Flag PSR.BRGIF is cleared."]
    VALUE2,
}
impl From<CBRGIF_AW> for bool {
    #[inline(always)]
    fn from(variant: CBRGIF_AW) -> Self {
        match variant {
            CBRGIF_AW::VALUE1 => false,
            CBRGIF_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CBRGIF`"]
pub struct CBRGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBRGIF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CBRGIF_AW::VALUE1)
    }
    #[doc = "Flag PSR.BRGIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CBRGIF_AW::VALUE2)
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
impl W {
    #[doc = "Bit 0 - Clear Status Flag 0 in PSR"]
    #[inline(always)]
    pub fn cst0(&mut self) -> CST0_W {
        CST0_W { w: self }
    }
    #[doc = "Bit 1 - Clear Status Flag 1 in PSR"]
    #[inline(always)]
    pub fn cst1(&mut self) -> CST1_W {
        CST1_W { w: self }
    }
    #[doc = "Bit 2 - Clear Status Flag 2 in PSR"]
    #[inline(always)]
    pub fn cst2(&mut self) -> CST2_W {
        CST2_W { w: self }
    }
    #[doc = "Bit 3 - Clear Status Flag 3 in PSR"]
    #[inline(always)]
    pub fn cst3(&mut self) -> CST3_W {
        CST3_W { w: self }
    }
    #[doc = "Bit 4 - Clear Status Flag 4 in PSR"]
    #[inline(always)]
    pub fn cst4(&mut self) -> CST4_W {
        CST4_W { w: self }
    }
    #[doc = "Bit 5 - Clear Status Flag 5 in PSR"]
    #[inline(always)]
    pub fn cst5(&mut self) -> CST5_W {
        CST5_W { w: self }
    }
    #[doc = "Bit 6 - Clear Status Flag 6 in PSR"]
    #[inline(always)]
    pub fn cst6(&mut self) -> CST6_W {
        CST6_W { w: self }
    }
    #[doc = "Bit 7 - Clear Status Flag 7 in PSR"]
    #[inline(always)]
    pub fn cst7(&mut self) -> CST7_W {
        CST7_W { w: self }
    }
    #[doc = "Bit 8 - Clear Status Flag 8 in PSR"]
    #[inline(always)]
    pub fn cst8(&mut self) -> CST8_W {
        CST8_W { w: self }
    }
    #[doc = "Bit 9 - Clear Status Flag 9 in PSR"]
    #[inline(always)]
    pub fn cst9(&mut self) -> CST9_W {
        CST9_W { w: self }
    }
    #[doc = "Bit 10 - Clear Receiver Start Indication Flag"]
    #[inline(always)]
    pub fn crsif(&mut self) -> CRSIF_W {
        CRSIF_W { w: self }
    }
    #[doc = "Bit 11 - Clear Data Lost Indication Flag"]
    #[inline(always)]
    pub fn cdlif(&mut self) -> CDLIF_W {
        CDLIF_W { w: self }
    }
    #[doc = "Bit 12 - Clear Transmit Shift Indication Flag"]
    #[inline(always)]
    pub fn ctsif(&mut self) -> CTSIF_W {
        CTSIF_W { w: self }
    }
    #[doc = "Bit 13 - Clear Transmit Buffer Indication Flag"]
    #[inline(always)]
    pub fn ctbif(&mut self) -> CTBIF_W {
        CTBIF_W { w: self }
    }
    #[doc = "Bit 14 - Clear Receive Indication Flag"]
    #[inline(always)]
    pub fn crif(&mut self) -> CRIF_W {
        CRIF_W { w: self }
    }
    #[doc = "Bit 15 - Clear Alternative Receive Indication Flag"]
    #[inline(always)]
    pub fn caif(&mut self) -> CAIF_W {
        CAIF_W { w: self }
    }
    #[doc = "Bit 16 - Clear Baud Rate Generator Indication Flag"]
    #[inline(always)]
    pub fn cbrgif(&mut self) -> CBRGIF_W {
        CBRGIF_W { w: self }
    }
}
