#[doc = "Register `PCR` reader"]
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR` writer"]
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
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
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTR0` reader - Protocol Control Bit 0"]
pub struct CTR0_R(crate::FieldReader<bool, bool>);
impl CTR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR0` writer - Protocol Control Bit 0"]
pub struct CTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR0_W<'a> {
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
#[doc = "Field `CTR1` reader - Protocol Control Bit 1"]
pub struct CTR1_R(crate::FieldReader<bool, bool>);
impl CTR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR1` writer - Protocol Control Bit 1"]
pub struct CTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR1_W<'a> {
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
#[doc = "Field `CTR2` reader - Protocol Control Bit 2"]
pub struct CTR2_R(crate::FieldReader<bool, bool>);
impl CTR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR2` writer - Protocol Control Bit 2"]
pub struct CTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR2_W<'a> {
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
#[doc = "Field `CTR3` reader - Protocol Control Bit 3"]
pub struct CTR3_R(crate::FieldReader<bool, bool>);
impl CTR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR3` writer - Protocol Control Bit 3"]
pub struct CTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR3_W<'a> {
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
#[doc = "Field `CTR4` reader - Protocol Control Bit 4"]
pub struct CTR4_R(crate::FieldReader<bool, bool>);
impl CTR4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR4` writer - Protocol Control Bit 4"]
pub struct CTR4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR4_W<'a> {
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
#[doc = "Field `CTR5` reader - Protocol Control Bit 5"]
pub struct CTR5_R(crate::FieldReader<bool, bool>);
impl CTR5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR5` writer - Protocol Control Bit 5"]
pub struct CTR5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR5_W<'a> {
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
#[doc = "Field `CTR6` reader - Protocol Control Bit 6"]
pub struct CTR6_R(crate::FieldReader<bool, bool>);
impl CTR6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR6` writer - Protocol Control Bit 6"]
pub struct CTR6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR6_W<'a> {
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
#[doc = "Field `CTR7` reader - Protocol Control Bit 7"]
pub struct CTR7_R(crate::FieldReader<bool, bool>);
impl CTR7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR7` writer - Protocol Control Bit 7"]
pub struct CTR7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR7_W<'a> {
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
#[doc = "Field `CTR8` reader - Protocol Control Bit 8"]
pub struct CTR8_R(crate::FieldReader<bool, bool>);
impl CTR8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR8` writer - Protocol Control Bit 8"]
pub struct CTR8_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR8_W<'a> {
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
#[doc = "Field `CTR9` reader - Protocol Control Bit 9"]
pub struct CTR9_R(crate::FieldReader<bool, bool>);
impl CTR9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR9` writer - Protocol Control Bit 9"]
pub struct CTR9_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR9_W<'a> {
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
#[doc = "Field `CTR10` reader - Protocol Control Bit 10"]
pub struct CTR10_R(crate::FieldReader<bool, bool>);
impl CTR10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR10` writer - Protocol Control Bit 10"]
pub struct CTR10_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR10_W<'a> {
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
#[doc = "Field `CTR11` reader - Protocol Control Bit 11"]
pub struct CTR11_R(crate::FieldReader<bool, bool>);
impl CTR11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR11` writer - Protocol Control Bit 11"]
pub struct CTR11_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR11_W<'a> {
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
#[doc = "Field `CTR12` reader - Protocol Control Bit 12"]
pub struct CTR12_R(crate::FieldReader<bool, bool>);
impl CTR12_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR12` writer - Protocol Control Bit 12"]
pub struct CTR12_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR12_W<'a> {
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
#[doc = "Field `CTR13` reader - Protocol Control Bit 13"]
pub struct CTR13_R(crate::FieldReader<bool, bool>);
impl CTR13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR13` writer - Protocol Control Bit 13"]
pub struct CTR13_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR13_W<'a> {
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
#[doc = "Field `CTR14` reader - Protocol Control Bit 14"]
pub struct CTR14_R(crate::FieldReader<bool, bool>);
impl CTR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR14` writer - Protocol Control Bit 14"]
pub struct CTR14_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR14_W<'a> {
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
#[doc = "Field `CTR15` reader - Protocol Control Bit 15"]
pub struct CTR15_R(crate::FieldReader<bool, bool>);
impl CTR15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR15` writer - Protocol Control Bit 15"]
pub struct CTR15_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR15_W<'a> {
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
#[doc = "Field `CTR16` reader - Protocol Control Bit 16"]
pub struct CTR16_R(crate::FieldReader<bool, bool>);
impl CTR16_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR16` writer - Protocol Control Bit 16"]
pub struct CTR16_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CTR17` reader - Protocol Control Bit 17"]
pub struct CTR17_R(crate::FieldReader<bool, bool>);
impl CTR17_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR17` writer - Protocol Control Bit 17"]
pub struct CTR17_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CTR18` reader - Protocol Control Bit 18"]
pub struct CTR18_R(crate::FieldReader<bool, bool>);
impl CTR18_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR18` writer - Protocol Control Bit 18"]
pub struct CTR18_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CTR19` reader - Protocol Control Bit 19"]
pub struct CTR19_R(crate::FieldReader<bool, bool>);
impl CTR19_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR19` writer - Protocol Control Bit 19"]
pub struct CTR19_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CTR20` reader - Protocol Control Bit 20"]
pub struct CTR20_R(crate::FieldReader<bool, bool>);
impl CTR20_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR20` writer - Protocol Control Bit 20"]
pub struct CTR20_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CTR21` reader - Protocol Control Bit 21"]
pub struct CTR21_R(crate::FieldReader<bool, bool>);
impl CTR21_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR21` writer - Protocol Control Bit 21"]
pub struct CTR21_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CTR22` reader - Protocol Control Bit 22"]
pub struct CTR22_R(crate::FieldReader<bool, bool>);
impl CTR22_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR22` writer - Protocol Control Bit 22"]
pub struct CTR22_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CTR23` reader - Protocol Control Bit 23"]
pub struct CTR23_R(crate::FieldReader<bool, bool>);
impl CTR23_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR23` writer - Protocol Control Bit 23"]
pub struct CTR23_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `CTR24` reader - Protocol Control Bit 24"]
pub struct CTR24_R(crate::FieldReader<bool, bool>);
impl CTR24_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR24` writer - Protocol Control Bit 24"]
pub struct CTR24_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CTR25` reader - Protocol Control Bit 25"]
pub struct CTR25_R(crate::FieldReader<bool, bool>);
impl CTR25_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR25` writer - Protocol Control Bit 25"]
pub struct CTR25_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `CTR26` reader - Protocol Control Bit 26"]
pub struct CTR26_R(crate::FieldReader<bool, bool>);
impl CTR26_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR26` writer - Protocol Control Bit 26"]
pub struct CTR26_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CTR27` reader - Protocol Control Bit 27"]
pub struct CTR27_R(crate::FieldReader<bool, bool>);
impl CTR27_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR27` writer - Protocol Control Bit 27"]
pub struct CTR27_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `CTR28` reader - Protocol Control Bit 28"]
pub struct CTR28_R(crate::FieldReader<bool, bool>);
impl CTR28_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR28` writer - Protocol Control Bit 28"]
pub struct CTR28_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CTR29` reader - Protocol Control Bit 29"]
pub struct CTR29_R(crate::FieldReader<bool, bool>);
impl CTR29_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR29` writer - Protocol Control Bit 29"]
pub struct CTR29_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `CTR30` reader - Protocol Control Bit 30"]
pub struct CTR30_R(crate::FieldReader<bool, bool>);
impl CTR30_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR30` writer - Protocol Control Bit 30"]
pub struct CTR30_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `CTR31` reader - Protocol Control Bit 31"]
pub struct CTR31_R(crate::FieldReader<bool, bool>);
impl CTR31_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR31` writer - Protocol Control Bit 31"]
pub struct CTR31_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Protocol Control Bit 0"]
    #[inline(always)]
    pub fn ctr0(&self) -> CTR0_R {
        CTR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Protocol Control Bit 1"]
    #[inline(always)]
    pub fn ctr1(&self) -> CTR1_R {
        CTR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protocol Control Bit 2"]
    #[inline(always)]
    pub fn ctr2(&self) -> CTR2_R {
        CTR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Protocol Control Bit 3"]
    #[inline(always)]
    pub fn ctr3(&self) -> CTR3_R {
        CTR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Protocol Control Bit 4"]
    #[inline(always)]
    pub fn ctr4(&self) -> CTR4_R {
        CTR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Protocol Control Bit 5"]
    #[inline(always)]
    pub fn ctr5(&self) -> CTR5_R {
        CTR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Protocol Control Bit 6"]
    #[inline(always)]
    pub fn ctr6(&self) -> CTR6_R {
        CTR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Protocol Control Bit 7"]
    #[inline(always)]
    pub fn ctr7(&self) -> CTR7_R {
        CTR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Protocol Control Bit 8"]
    #[inline(always)]
    pub fn ctr8(&self) -> CTR8_R {
        CTR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Protocol Control Bit 9"]
    #[inline(always)]
    pub fn ctr9(&self) -> CTR9_R {
        CTR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Protocol Control Bit 10"]
    #[inline(always)]
    pub fn ctr10(&self) -> CTR10_R {
        CTR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Protocol Control Bit 11"]
    #[inline(always)]
    pub fn ctr11(&self) -> CTR11_R {
        CTR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Protocol Control Bit 12"]
    #[inline(always)]
    pub fn ctr12(&self) -> CTR12_R {
        CTR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Protocol Control Bit 13"]
    #[inline(always)]
    pub fn ctr13(&self) -> CTR13_R {
        CTR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Protocol Control Bit 14"]
    #[inline(always)]
    pub fn ctr14(&self) -> CTR14_R {
        CTR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Protocol Control Bit 15"]
    #[inline(always)]
    pub fn ctr15(&self) -> CTR15_R {
        CTR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Protocol Control Bit 16"]
    #[inline(always)]
    pub fn ctr16(&self) -> CTR16_R {
        CTR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Protocol Control Bit 17"]
    #[inline(always)]
    pub fn ctr17(&self) -> CTR17_R {
        CTR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Protocol Control Bit 18"]
    #[inline(always)]
    pub fn ctr18(&self) -> CTR18_R {
        CTR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Protocol Control Bit 19"]
    #[inline(always)]
    pub fn ctr19(&self) -> CTR19_R {
        CTR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Protocol Control Bit 20"]
    #[inline(always)]
    pub fn ctr20(&self) -> CTR20_R {
        CTR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Protocol Control Bit 21"]
    #[inline(always)]
    pub fn ctr21(&self) -> CTR21_R {
        CTR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Protocol Control Bit 22"]
    #[inline(always)]
    pub fn ctr22(&self) -> CTR22_R {
        CTR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Protocol Control Bit 23"]
    #[inline(always)]
    pub fn ctr23(&self) -> CTR23_R {
        CTR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Protocol Control Bit 24"]
    #[inline(always)]
    pub fn ctr24(&self) -> CTR24_R {
        CTR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Protocol Control Bit 25"]
    #[inline(always)]
    pub fn ctr25(&self) -> CTR25_R {
        CTR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Protocol Control Bit 26"]
    #[inline(always)]
    pub fn ctr26(&self) -> CTR26_R {
        CTR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol Control Bit 27"]
    #[inline(always)]
    pub fn ctr27(&self) -> CTR27_R {
        CTR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol Control Bit 28"]
    #[inline(always)]
    pub fn ctr28(&self) -> CTR28_R {
        CTR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Protocol Control Bit 29"]
    #[inline(always)]
    pub fn ctr29(&self) -> CTR29_R {
        CTR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Protocol Control Bit 30"]
    #[inline(always)]
    pub fn ctr30(&self) -> CTR30_R {
        CTR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Protocol Control Bit 31"]
    #[inline(always)]
    pub fn ctr31(&self) -> CTR31_R {
        CTR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protocol Control Bit 0"]
    #[inline(always)]
    pub fn ctr0(&mut self) -> CTR0_W {
        CTR0_W { w: self }
    }
    #[doc = "Bit 1 - Protocol Control Bit 1"]
    #[inline(always)]
    pub fn ctr1(&mut self) -> CTR1_W {
        CTR1_W { w: self }
    }
    #[doc = "Bit 2 - Protocol Control Bit 2"]
    #[inline(always)]
    pub fn ctr2(&mut self) -> CTR2_W {
        CTR2_W { w: self }
    }
    #[doc = "Bit 3 - Protocol Control Bit 3"]
    #[inline(always)]
    pub fn ctr3(&mut self) -> CTR3_W {
        CTR3_W { w: self }
    }
    #[doc = "Bit 4 - Protocol Control Bit 4"]
    #[inline(always)]
    pub fn ctr4(&mut self) -> CTR4_W {
        CTR4_W { w: self }
    }
    #[doc = "Bit 5 - Protocol Control Bit 5"]
    #[inline(always)]
    pub fn ctr5(&mut self) -> CTR5_W {
        CTR5_W { w: self }
    }
    #[doc = "Bit 6 - Protocol Control Bit 6"]
    #[inline(always)]
    pub fn ctr6(&mut self) -> CTR6_W {
        CTR6_W { w: self }
    }
    #[doc = "Bit 7 - Protocol Control Bit 7"]
    #[inline(always)]
    pub fn ctr7(&mut self) -> CTR7_W {
        CTR7_W { w: self }
    }
    #[doc = "Bit 8 - Protocol Control Bit 8"]
    #[inline(always)]
    pub fn ctr8(&mut self) -> CTR8_W {
        CTR8_W { w: self }
    }
    #[doc = "Bit 9 - Protocol Control Bit 9"]
    #[inline(always)]
    pub fn ctr9(&mut self) -> CTR9_W {
        CTR9_W { w: self }
    }
    #[doc = "Bit 10 - Protocol Control Bit 10"]
    #[inline(always)]
    pub fn ctr10(&mut self) -> CTR10_W {
        CTR10_W { w: self }
    }
    #[doc = "Bit 11 - Protocol Control Bit 11"]
    #[inline(always)]
    pub fn ctr11(&mut self) -> CTR11_W {
        CTR11_W { w: self }
    }
    #[doc = "Bit 12 - Protocol Control Bit 12"]
    #[inline(always)]
    pub fn ctr12(&mut self) -> CTR12_W {
        CTR12_W { w: self }
    }
    #[doc = "Bit 13 - Protocol Control Bit 13"]
    #[inline(always)]
    pub fn ctr13(&mut self) -> CTR13_W {
        CTR13_W { w: self }
    }
    #[doc = "Bit 14 - Protocol Control Bit 14"]
    #[inline(always)]
    pub fn ctr14(&mut self) -> CTR14_W {
        CTR14_W { w: self }
    }
    #[doc = "Bit 15 - Protocol Control Bit 15"]
    #[inline(always)]
    pub fn ctr15(&mut self) -> CTR15_W {
        CTR15_W { w: self }
    }
    #[doc = "Bit 16 - Protocol Control Bit 16"]
    #[inline(always)]
    pub fn ctr16(&mut self) -> CTR16_W {
        CTR16_W { w: self }
    }
    #[doc = "Bit 17 - Protocol Control Bit 17"]
    #[inline(always)]
    pub fn ctr17(&mut self) -> CTR17_W {
        CTR17_W { w: self }
    }
    #[doc = "Bit 18 - Protocol Control Bit 18"]
    #[inline(always)]
    pub fn ctr18(&mut self) -> CTR18_W {
        CTR18_W { w: self }
    }
    #[doc = "Bit 19 - Protocol Control Bit 19"]
    #[inline(always)]
    pub fn ctr19(&mut self) -> CTR19_W {
        CTR19_W { w: self }
    }
    #[doc = "Bit 20 - Protocol Control Bit 20"]
    #[inline(always)]
    pub fn ctr20(&mut self) -> CTR20_W {
        CTR20_W { w: self }
    }
    #[doc = "Bit 21 - Protocol Control Bit 21"]
    #[inline(always)]
    pub fn ctr21(&mut self) -> CTR21_W {
        CTR21_W { w: self }
    }
    #[doc = "Bit 22 - Protocol Control Bit 22"]
    #[inline(always)]
    pub fn ctr22(&mut self) -> CTR22_W {
        CTR22_W { w: self }
    }
    #[doc = "Bit 23 - Protocol Control Bit 23"]
    #[inline(always)]
    pub fn ctr23(&mut self) -> CTR23_W {
        CTR23_W { w: self }
    }
    #[doc = "Bit 24 - Protocol Control Bit 24"]
    #[inline(always)]
    pub fn ctr24(&mut self) -> CTR24_W {
        CTR24_W { w: self }
    }
    #[doc = "Bit 25 - Protocol Control Bit 25"]
    #[inline(always)]
    pub fn ctr25(&mut self) -> CTR25_W {
        CTR25_W { w: self }
    }
    #[doc = "Bit 26 - Protocol Control Bit 26"]
    #[inline(always)]
    pub fn ctr26(&mut self) -> CTR26_W {
        CTR26_W { w: self }
    }
    #[doc = "Bit 27 - Protocol Control Bit 27"]
    #[inline(always)]
    pub fn ctr27(&mut self) -> CTR27_W {
        CTR27_W { w: self }
    }
    #[doc = "Bit 28 - Protocol Control Bit 28"]
    #[inline(always)]
    pub fn ctr28(&mut self) -> CTR28_W {
        CTR28_W { w: self }
    }
    #[doc = "Bit 29 - Protocol Control Bit 29"]
    #[inline(always)]
    pub fn ctr29(&mut self) -> CTR29_W {
        CTR29_W { w: self }
    }
    #[doc = "Bit 30 - Protocol Control Bit 30"]
    #[inline(always)]
    pub fn ctr30(&mut self) -> CTR30_W {
        CTR30_W { w: self }
    }
    #[doc = "Bit 31 - Protocol Control Bit 31"]
    #[inline(always)]
    pub fn ctr31(&mut self) -> CTR31_W {
        CTR31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protocol Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](index.html) module"]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr::R](R) reader structure"]
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr::W](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCR to value 0"]
impl crate::Resettable for PCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
