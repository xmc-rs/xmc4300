#[doc = "Register `REFCLR` writer"]
pub struct W(crate::W<REFCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<REFCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Result Event for Result Register 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV0_AW> for bool {
    #[inline(always)]
    fn from(variant: REV0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV0` writer - Clear Result Event for Result Register 0"]
pub struct REV0_W<'a> {
    w: &'a mut W,
}
impl<'a> REV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV0_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV0_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV1_AW> for bool {
    #[inline(always)]
    fn from(variant: REV1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV1` writer - Clear Result Event for Result Register 1"]
pub struct REV1_W<'a> {
    w: &'a mut W,
}
impl<'a> REV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV1_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV1_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV2_AW> for bool {
    #[inline(always)]
    fn from(variant: REV2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV2` writer - Clear Result Event for Result Register 2"]
pub struct REV2_W<'a> {
    w: &'a mut W,
}
impl<'a> REV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV2_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV2_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV3_AW> for bool {
    #[inline(always)]
    fn from(variant: REV3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV3` writer - Clear Result Event for Result Register 3"]
pub struct REV3_W<'a> {
    w: &'a mut W,
}
impl<'a> REV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV3_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV3_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV4_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV4_AW> for bool {
    #[inline(always)]
    fn from(variant: REV4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV4` writer - Clear Result Event for Result Register 4"]
pub struct REV4_W<'a> {
    w: &'a mut W,
}
impl<'a> REV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV4_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV4_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV5_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV5_AW> for bool {
    #[inline(always)]
    fn from(variant: REV5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV5` writer - Clear Result Event for Result Register 5"]
pub struct REV5_W<'a> {
    w: &'a mut W,
}
impl<'a> REV5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV5_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV5_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV6_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV6_AW> for bool {
    #[inline(always)]
    fn from(variant: REV6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV6` writer - Clear Result Event for Result Register 6"]
pub struct REV6_W<'a> {
    w: &'a mut W,
}
impl<'a> REV6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV6_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV6_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV7_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV7_AW> for bool {
    #[inline(always)]
    fn from(variant: REV7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV7` writer - Clear Result Event for Result Register 7"]
pub struct REV7_W<'a> {
    w: &'a mut W,
}
impl<'a> REV7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV7_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV7_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV8_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV8_AW> for bool {
    #[inline(always)]
    fn from(variant: REV8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV8` writer - Clear Result Event for Result Register 8"]
pub struct REV8_W<'a> {
    w: &'a mut W,
}
impl<'a> REV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV8_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV8_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV9_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV9_AW> for bool {
    #[inline(always)]
    fn from(variant: REV9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV9` writer - Clear Result Event for Result Register 9"]
pub struct REV9_W<'a> {
    w: &'a mut W,
}
impl<'a> REV9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV9_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV9_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV9_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV10_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV10_AW> for bool {
    #[inline(always)]
    fn from(variant: REV10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV10` writer - Clear Result Event for Result Register 10"]
pub struct REV10_W<'a> {
    w: &'a mut W,
}
impl<'a> REV10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV10_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV10_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV10_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV11_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV11_AW> for bool {
    #[inline(always)]
    fn from(variant: REV11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV11` writer - Clear Result Event for Result Register 11"]
pub struct REV11_W<'a> {
    w: &'a mut W,
}
impl<'a> REV11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV11_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV11_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV11_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV12_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV12_AW> for bool {
    #[inline(always)]
    fn from(variant: REV12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV12` writer - Clear Result Event for Result Register 12"]
pub struct REV12_W<'a> {
    w: &'a mut W,
}
impl<'a> REV12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV12_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV12_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV12_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV13_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV13_AW> for bool {
    #[inline(always)]
    fn from(variant: REV13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV13` writer - Clear Result Event for Result Register 13"]
pub struct REV13_W<'a> {
    w: &'a mut W,
}
impl<'a> REV13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV13_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV13_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV13_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV14_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV14_AW> for bool {
    #[inline(always)]
    fn from(variant: REV14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV14` writer - Clear Result Event for Result Register 14"]
pub struct REV14_W<'a> {
    w: &'a mut W,
}
impl<'a> REV14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV14_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV14_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV14_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Clear Result Event for Result Register 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV15_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV15_AW> for bool {
    #[inline(always)]
    fn from(variant: REV15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV15` writer - Clear Result Event for Result Register 15"]
pub struct REV15_W<'a> {
    w: &'a mut W,
}
impl<'a> REV15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV15_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV15_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV15_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear Result Event for Result Register 0"]
    #[inline(always)]
    pub fn rev0(&mut self) -> REV0_W {
        REV0_W { w: self }
    }
    #[doc = "Bit 1 - Clear Result Event for Result Register 1"]
    #[inline(always)]
    pub fn rev1(&mut self) -> REV1_W {
        REV1_W { w: self }
    }
    #[doc = "Bit 2 - Clear Result Event for Result Register 2"]
    #[inline(always)]
    pub fn rev2(&mut self) -> REV2_W {
        REV2_W { w: self }
    }
    #[doc = "Bit 3 - Clear Result Event for Result Register 3"]
    #[inline(always)]
    pub fn rev3(&mut self) -> REV3_W {
        REV3_W { w: self }
    }
    #[doc = "Bit 4 - Clear Result Event for Result Register 4"]
    #[inline(always)]
    pub fn rev4(&mut self) -> REV4_W {
        REV4_W { w: self }
    }
    #[doc = "Bit 5 - Clear Result Event for Result Register 5"]
    #[inline(always)]
    pub fn rev5(&mut self) -> REV5_W {
        REV5_W { w: self }
    }
    #[doc = "Bit 6 - Clear Result Event for Result Register 6"]
    #[inline(always)]
    pub fn rev6(&mut self) -> REV6_W {
        REV6_W { w: self }
    }
    #[doc = "Bit 7 - Clear Result Event for Result Register 7"]
    #[inline(always)]
    pub fn rev7(&mut self) -> REV7_W {
        REV7_W { w: self }
    }
    #[doc = "Bit 8 - Clear Result Event for Result Register 8"]
    #[inline(always)]
    pub fn rev8(&mut self) -> REV8_W {
        REV8_W { w: self }
    }
    #[doc = "Bit 9 - Clear Result Event for Result Register 9"]
    #[inline(always)]
    pub fn rev9(&mut self) -> REV9_W {
        REV9_W { w: self }
    }
    #[doc = "Bit 10 - Clear Result Event for Result Register 10"]
    #[inline(always)]
    pub fn rev10(&mut self) -> REV10_W {
        REV10_W { w: self }
    }
    #[doc = "Bit 11 - Clear Result Event for Result Register 11"]
    #[inline(always)]
    pub fn rev11(&mut self) -> REV11_W {
        REV11_W { w: self }
    }
    #[doc = "Bit 12 - Clear Result Event for Result Register 12"]
    #[inline(always)]
    pub fn rev12(&mut self) -> REV12_W {
        REV12_W { w: self }
    }
    #[doc = "Bit 13 - Clear Result Event for Result Register 13"]
    #[inline(always)]
    pub fn rev13(&mut self) -> REV13_W {
        REV13_W { w: self }
    }
    #[doc = "Bit 14 - Clear Result Event for Result Register 14"]
    #[inline(always)]
    pub fn rev14(&mut self) -> REV14_W {
        REV14_W { w: self }
    }
    #[doc = "Bit 15 - Clear Result Event for Result Register 15"]
    #[inline(always)]
    pub fn rev15(&mut self) -> REV15_W {
        REV15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Result Event Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refclr](index.html) module"]
pub struct REFCLR_SPEC;
impl crate::RegisterSpec for REFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [refclr::W](W) writer structure"]
impl crate::Writable for REFCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFCLR to value 0"]
impl crate::Resettable for REFCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
