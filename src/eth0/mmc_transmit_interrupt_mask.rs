#[doc = "Register `MMC_TRANSMIT_INTERRUPT_MASK` reader"]
pub struct R(crate::R<MMC_TRANSMIT_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_TRANSMIT_INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_TRANSMIT_INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_TRANSMIT_INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_TRANSMIT_INTERRUPT_MASK` writer"]
pub struct W(crate::W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_TRANSMIT_INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXGBOCTIM` reader - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
pub struct TXGBOCTIM_R(crate::FieldReader<bool, bool>);
impl TXGBOCTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXGBOCTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXGBOCTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXGBOCTIM` writer - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
pub struct TXGBOCTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXGBOCTIM_W<'a> {
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
#[doc = "Field `TXGBFRMIM` reader - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
pub struct TXGBFRMIM_R(crate::FieldReader<bool, bool>);
impl TXGBFRMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXGBFRMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXGBFRMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXGBFRMIM` writer - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
pub struct TXGBFRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXGBFRMIM_W<'a> {
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
#[doc = "Field `TXBCGFIM` reader - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
pub struct TXBCGFIM_R(crate::FieldReader<bool, bool>);
impl TXBCGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBCGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBCGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBCGFIM` writer - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
pub struct TXBCGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBCGFIM_W<'a> {
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
#[doc = "Field `TXMCGFIM` reader - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
pub struct TXMCGFIM_R(crate::FieldReader<bool, bool>);
impl TXMCGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMCGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMCGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMCGFIM` writer - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
pub struct TXMCGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMCGFIM_W<'a> {
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
#[doc = "Field `TX64OCTGBFIM` reader - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX64OCTGBFIM_R(crate::FieldReader<bool, bool>);
impl TX64OCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX64OCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX64OCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX64OCTGBFIM` writer - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX64OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX64OCTGBFIM_W<'a> {
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
#[doc = "Field `TX65T127OCTGBFIM` reader - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX65T127OCTGBFIM_R(crate::FieldReader<bool, bool>);
impl TX65T127OCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX65T127OCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX65T127OCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX65T127OCTGBFIM` writer - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX65T127OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX65T127OCTGBFIM_W<'a> {
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
#[doc = "Field `TX128T255OCTGBFIM` reader - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX128T255OCTGBFIM_R(crate::FieldReader<bool, bool>);
impl TX128T255OCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX128T255OCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX128T255OCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX128T255OCTGBFIM` writer - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX128T255OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX128T255OCTGBFIM_W<'a> {
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
#[doc = "Field `TX256T511OCTGBFIM` reader - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX256T511OCTGBFIM_R(crate::FieldReader<bool, bool>);
impl TX256T511OCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX256T511OCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX256T511OCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX256T511OCTGBFIM` writer - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX256T511OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX256T511OCTGBFIM_W<'a> {
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
#[doc = "Field `TX512T1023OCTGBFIM` reader - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX512T1023OCTGBFIM_R(crate::FieldReader<bool, bool>);
impl TX512T1023OCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX512T1023OCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX512T1023OCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX512T1023OCTGBFIM` writer - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX512T1023OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX512T1023OCTGBFIM_W<'a> {
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
#[doc = "Field `TX1024TMAXOCTGBFIM` reader - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX1024TMAXOCTGBFIM_R(crate::FieldReader<bool, bool>);
impl TX1024TMAXOCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX1024TMAXOCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX1024TMAXOCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX1024TMAXOCTGBFIM` writer - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
pub struct TX1024TMAXOCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX1024TMAXOCTGBFIM_W<'a> {
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
#[doc = "Field `TXUCGBFIM` reader - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
pub struct TXUCGBFIM_R(crate::FieldReader<bool, bool>);
impl TXUCGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUCGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUCGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUCGBFIM` writer - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
pub struct TXUCGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUCGBFIM_W<'a> {
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
#[doc = "Field `TXMCGBFIM` reader - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
pub struct TXMCGBFIM_R(crate::FieldReader<bool, bool>);
impl TXMCGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMCGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMCGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMCGBFIM` writer - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
pub struct TXMCGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMCGBFIM_W<'a> {
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
#[doc = "Field `TXBCGBFIM` reader - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
pub struct TXBCGBFIM_R(crate::FieldReader<bool, bool>);
impl TXBCGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBCGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBCGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBCGBFIM` writer - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
pub struct TXBCGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBCGBFIM_W<'a> {
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
#[doc = "Field `TXUFLOWERFIM` reader - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
pub struct TXUFLOWERFIM_R(crate::FieldReader<bool, bool>);
impl TXUFLOWERFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUFLOWERFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUFLOWERFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUFLOWERFIM` writer - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
pub struct TXUFLOWERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUFLOWERFIM_W<'a> {
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
#[doc = "Field `TXSCOLGFIM` reader - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
pub struct TXSCOLGFIM_R(crate::FieldReader<bool, bool>);
impl TXSCOLGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSCOLGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSCOLGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSCOLGFIM` writer - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
pub struct TXSCOLGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSCOLGFIM_W<'a> {
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
#[doc = "Field `TXMCOLGFIM` reader - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
pub struct TXMCOLGFIM_R(crate::FieldReader<bool, bool>);
impl TXMCOLGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMCOLGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMCOLGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMCOLGFIM` writer - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
pub struct TXMCOLGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMCOLGFIM_W<'a> {
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
#[doc = "Field `TXDEFFIM` reader - MMC Transmit Deferred Frame Counter Interrupt Mask"]
pub struct TXDEFFIM_R(crate::FieldReader<bool, bool>);
impl TXDEFFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDEFFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDEFFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDEFFIM` writer - MMC Transmit Deferred Frame Counter Interrupt Mask"]
pub struct TXDEFFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDEFFIM_W<'a> {
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
#[doc = "Field `TXLATCOLFIM` reader - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
pub struct TXLATCOLFIM_R(crate::FieldReader<bool, bool>);
impl TXLATCOLFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXLATCOLFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLATCOLFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLATCOLFIM` writer - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
pub struct TXLATCOLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLATCOLFIM_W<'a> {
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
#[doc = "Field `TXEXCOLFIM` reader - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
pub struct TXEXCOLFIM_R(crate::FieldReader<bool, bool>);
impl TXEXCOLFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEXCOLFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEXCOLFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEXCOLFIM` writer - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
pub struct TXEXCOLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEXCOLFIM_W<'a> {
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
#[doc = "Field `TXCARERFIM` reader - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
pub struct TXCARERFIM_R(crate::FieldReader<bool, bool>);
impl TXCARERFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXCARERFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCARERFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCARERFIM` writer - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
pub struct TXCARERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCARERFIM_W<'a> {
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
#[doc = "Field `TXGOCTIM` reader - MMC Transmit Good Octet Counter Interrupt Mask"]
pub struct TXGOCTIM_R(crate::FieldReader<bool, bool>);
impl TXGOCTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXGOCTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXGOCTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXGOCTIM` writer - MMC Transmit Good Octet Counter Interrupt Mask"]
pub struct TXGOCTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXGOCTIM_W<'a> {
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
#[doc = "Field `TXGFRMIM` reader - MMC Transmit Good Frame Counter Interrupt Mask"]
pub struct TXGFRMIM_R(crate::FieldReader<bool, bool>);
impl TXGFRMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXGFRMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXGFRMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXGFRMIM` writer - MMC Transmit Good Frame Counter Interrupt Mask"]
pub struct TXGFRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXGFRMIM_W<'a> {
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
#[doc = "Field `TXEXDEFFIM` reader - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
pub struct TXEXDEFFIM_R(crate::FieldReader<bool, bool>);
impl TXEXDEFFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEXDEFFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEXDEFFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEXDEFFIM` writer - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
pub struct TXEXDEFFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEXDEFFIM_W<'a> {
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
#[doc = "Field `TXPAUSFIM` reader - MMC Transmit Pause Frame Counter Interrupt Mask"]
pub struct TXPAUSFIM_R(crate::FieldReader<bool, bool>);
impl TXPAUSFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXPAUSFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPAUSFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPAUSFIM` writer - MMC Transmit Pause Frame Counter Interrupt Mask"]
pub struct TXPAUSFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPAUSFIM_W<'a> {
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
#[doc = "Field `TXVLANGFIM` reader - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
pub struct TXVLANGFIM_R(crate::FieldReader<bool, bool>);
impl TXVLANGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXVLANGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXVLANGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXVLANGFIM` writer - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
pub struct TXVLANGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXVLANGFIM_W<'a> {
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
#[doc = "Field `TXOSIZEGFIM` reader - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
pub struct TXOSIZEGFIM_R(crate::FieldReader<bool, bool>);
impl TXOSIZEGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOSIZEGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOSIZEGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOSIZEGFIM` writer - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
pub struct TXOSIZEGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOSIZEGFIM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgboctim(&self) -> TXGBOCTIM_R {
        TXGBOCTIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgbfrmim(&self) -> TXGBFRMIM_R {
        TXGBFRMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgfim(&self) -> TXBCGFIM_R {
        TXBCGFIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgfim(&self) -> TXMCGFIM_R {
        TXMCGFIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx64octgbfim(&self) -> TX64OCTGBFIM_R {
        TX64OCTGBFIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx65t127octgbfim(&self) -> TX65T127OCTGBFIM_R {
        TX65T127OCTGBFIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx128t255octgbfim(&self) -> TX128T255OCTGBFIM_R {
        TX128T255OCTGBFIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx256t511octgbfim(&self) -> TX256T511OCTGBFIM_R {
        TX256T511OCTGBFIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx512t1023octgbfim(&self) -> TX512T1023OCTGBFIM_R {
        TX512T1023OCTGBFIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfim(&self) -> TX1024TMAXOCTGBFIM_R {
        TX1024TMAXOCTGBFIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txucgbfim(&self) -> TXUCGBFIM_R {
        TXUCGBFIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgbfim(&self) -> TXMCGBFIM_R {
        TXMCGBFIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgbfim(&self) -> TXBCGBFIM_R {
        TXBCGBFIM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txuflowerfim(&self) -> TXUFLOWERFIM_R {
        TXUFLOWERFIM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgfim(&self) -> TXSCOLGFIM_R {
        TXSCOLGFIM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgfim(&self) -> TXMCOLGFIM_R {
        TXMCOLGFIM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txdeffim(&self) -> TXDEFFIM_R {
        TXDEFFIM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txlatcolfim(&self) -> TXLATCOLFIM_R {
        TXLATCOLFIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexcolfim(&self) -> TXEXCOLFIM_R {
        TXEXCOLFIM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txcarerfim(&self) -> TXCARERFIM_R {
        TXCARERFIM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgoctim(&self) -> TXGOCTIM_R {
        TXGOCTIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgfrmim(&self) -> TXGFRMIM_R {
        TXGFRMIM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexdeffim(&self) -> TXEXDEFFIM_R {
        TXEXDEFFIM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txpausfim(&self) -> TXPAUSFIM_R {
        TXPAUSFIM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txvlangfim(&self) -> TXVLANGFIM_R {
        TXVLANGFIM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txosizegfim(&self) -> TXOSIZEGFIM_R {
        TXOSIZEGFIM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgboctim(&mut self) -> TXGBOCTIM_W {
        TXGBOCTIM_W { w: self }
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgbfrmim(&mut self) -> TXGBFRMIM_W {
        TXGBFRMIM_W { w: self }
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgfim(&mut self) -> TXBCGFIM_W {
        TXBCGFIM_W { w: self }
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgfim(&mut self) -> TXMCGFIM_W {
        TXMCGFIM_W { w: self }
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx64octgbfim(&mut self) -> TX64OCTGBFIM_W {
        TX64OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx65t127octgbfim(&mut self) -> TX65T127OCTGBFIM_W {
        TX65T127OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx128t255octgbfim(&mut self) -> TX128T255OCTGBFIM_W {
        TX128T255OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx256t511octgbfim(&mut self) -> TX256T511OCTGBFIM_W {
        TX256T511OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx512t1023octgbfim(&mut self) -> TX512T1023OCTGBFIM_W {
        TX512T1023OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfim(&mut self) -> TX1024TMAXOCTGBFIM_W {
        TX1024TMAXOCTGBFIM_W { w: self }
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txucgbfim(&mut self) -> TXUCGBFIM_W {
        TXUCGBFIM_W { w: self }
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgbfim(&mut self) -> TXMCGBFIM_W {
        TXMCGBFIM_W { w: self }
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgbfim(&mut self) -> TXBCGBFIM_W {
        TXBCGBFIM_W { w: self }
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txuflowerfim(&mut self) -> TXUFLOWERFIM_W {
        TXUFLOWERFIM_W { w: self }
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgfim(&mut self) -> TXSCOLGFIM_W {
        TXSCOLGFIM_W { w: self }
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgfim(&mut self) -> TXMCOLGFIM_W {
        TXMCOLGFIM_W { w: self }
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txdeffim(&mut self) -> TXDEFFIM_W {
        TXDEFFIM_W { w: self }
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txlatcolfim(&mut self) -> TXLATCOLFIM_W {
        TXLATCOLFIM_W { w: self }
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexcolfim(&mut self) -> TXEXCOLFIM_W {
        TXEXCOLFIM_W { w: self }
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txcarerfim(&mut self) -> TXCARERFIM_W {
        TXCARERFIM_W { w: self }
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgoctim(&mut self) -> TXGOCTIM_W {
        TXGOCTIM_W { w: self }
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgfrmim(&mut self) -> TXGFRMIM_W {
        TXGFRMIM_W { w: self }
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexdeffim(&mut self) -> TXEXDEFFIM_W {
        TXEXDEFFIM_W { w: self }
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txpausfim(&mut self) -> TXPAUSFIM_W {
        TXPAUSFIM_W { w: self }
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txvlangfim(&mut self) -> TXVLANGFIM_W {
        TXVLANGFIM_W { w: self }
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txosizegfim(&mut self) -> TXOSIZEGFIM_W {
        TXOSIZEGFIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Transmit Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_transmit_interrupt_mask](index.html) module"]
pub struct MMC_TRANSMIT_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_TRANSMIT_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_transmit_interrupt_mask::R](R) reader structure"]
impl crate::Readable for MMC_TRANSMIT_INTERRUPT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_transmit_interrupt_mask::W](W) writer structure"]
impl crate::Writable for MMC_TRANSMIT_INTERRUPT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_TRANSMIT_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_TRANSMIT_INTERRUPT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
