#[doc = "Register `MMC_RECEIVE_INTERRUPT_MASK` reader"]
pub struct R(crate::R<MMC_RECEIVE_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_RECEIVE_INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_RECEIVE_INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_RECEIVE_INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_RECEIVE_INTERRUPT_MASK` writer"]
pub struct W(crate::W<MMC_RECEIVE_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_RECEIVE_INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<MMC_RECEIVE_INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_RECEIVE_INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXGBFRMIM` reader - MMC Receive Good Bad Frame Counter Interrupt Mask"]
pub struct RXGBFRMIM_R(crate::FieldReader<bool, bool>);
impl RXGBFRMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXGBFRMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXGBFRMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXGBFRMIM` writer - MMC Receive Good Bad Frame Counter Interrupt Mask"]
pub struct RXGBFRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXGBFRMIM_W<'a> {
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
#[doc = "Field `RXGBOCTIM` reader - MMC Receive Good Bad Octet Counter Interrupt Mask"]
pub struct RXGBOCTIM_R(crate::FieldReader<bool, bool>);
impl RXGBOCTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXGBOCTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXGBOCTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXGBOCTIM` writer - MMC Receive Good Bad Octet Counter Interrupt Mask"]
pub struct RXGBOCTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXGBOCTIM_W<'a> {
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
#[doc = "Field `RXGOCTIM` reader - MMC Receive Good Octet Counter Interrupt Mask"]
pub struct RXGOCTIM_R(crate::FieldReader<bool, bool>);
impl RXGOCTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXGOCTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXGOCTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXGOCTIM` writer - MMC Receive Good Octet Counter Interrupt Mask"]
pub struct RXGOCTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXGOCTIM_W<'a> {
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
#[doc = "Field `RXBCGFIM` reader - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
pub struct RXBCGFIM_R(crate::FieldReader<bool, bool>);
impl RXBCGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBCGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBCGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBCGFIM` writer - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
pub struct RXBCGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBCGFIM_W<'a> {
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
#[doc = "Field `RXMCGFIM` reader - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
pub struct RXMCGFIM_R(crate::FieldReader<bool, bool>);
impl RXMCGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMCGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMCGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMCGFIM` writer - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
pub struct RXMCGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMCGFIM_W<'a> {
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
#[doc = "Field `RXCRCERFIM` reader - MMC Receive CRC Error Frame Counter Interrupt Mask"]
pub struct RXCRCERFIM_R(crate::FieldReader<bool, bool>);
impl RXCRCERFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXCRCERFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCRCERFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCRCERFIM` writer - MMC Receive CRC Error Frame Counter Interrupt Mask"]
pub struct RXCRCERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCRCERFIM_W<'a> {
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
#[doc = "Field `RXALGNERFIM` reader - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
pub struct RXALGNERFIM_R(crate::FieldReader<bool, bool>);
impl RXALGNERFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXALGNERFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXALGNERFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXALGNERFIM` writer - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
pub struct RXALGNERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXALGNERFIM_W<'a> {
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
#[doc = "Field `RXRUNTFIM` reader - MMC Receive Runt Frame Counter Interrupt Mask"]
pub struct RXRUNTFIM_R(crate::FieldReader<bool, bool>);
impl RXRUNTFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRUNTFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRUNTFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRUNTFIM` writer - MMC Receive Runt Frame Counter Interrupt Mask"]
pub struct RXRUNTFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRUNTFIM_W<'a> {
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
#[doc = "Field `RXJABERFIM` reader - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
pub struct RXJABERFIM_R(crate::FieldReader<bool, bool>);
impl RXJABERFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXJABERFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXJABERFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXJABERFIM` writer - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
pub struct RXJABERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXJABERFIM_W<'a> {
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
#[doc = "Field `RXUSIZEGFIM` reader - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
pub struct RXUSIZEGFIM_R(crate::FieldReader<bool, bool>);
impl RXUSIZEGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUSIZEGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUSIZEGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUSIZEGFIM` writer - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
pub struct RXUSIZEGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUSIZEGFIM_W<'a> {
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
#[doc = "Field `RXOSIZEGFIM` reader - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
pub struct RXOSIZEGFIM_R(crate::FieldReader<bool, bool>);
impl RXOSIZEGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOSIZEGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOSIZEGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOSIZEGFIM` writer - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
pub struct RXOSIZEGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOSIZEGFIM_W<'a> {
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
#[doc = "Field `RX64OCTGBFIM` reader - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX64OCTGBFIM_R(crate::FieldReader<bool, bool>);
impl RX64OCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX64OCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX64OCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX64OCTGBFIM` writer - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX64OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX64OCTGBFIM_W<'a> {
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
#[doc = "Field `RX65T127OCTGBFIM` reader - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX65T127OCTGBFIM_R(crate::FieldReader<bool, bool>);
impl RX65T127OCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX65T127OCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX65T127OCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX65T127OCTGBFIM` writer - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX65T127OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX65T127OCTGBFIM_W<'a> {
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
#[doc = "Field `RX128T255OCTGBFIM` reader - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX128T255OCTGBFIM_R(crate::FieldReader<bool, bool>);
impl RX128T255OCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX128T255OCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX128T255OCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX128T255OCTGBFIM` writer - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX128T255OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX128T255OCTGBFIM_W<'a> {
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
#[doc = "Field `RX256T511OCTGBFIM` reader - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX256T511OCTGBFIM_R(crate::FieldReader<bool, bool>);
impl RX256T511OCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX256T511OCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX256T511OCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX256T511OCTGBFIM` writer - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX256T511OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX256T511OCTGBFIM_W<'a> {
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
#[doc = "Field `RX512T1023OCTGBFIM` reader - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX512T1023OCTGBFIM_R(crate::FieldReader<bool, bool>);
impl RX512T1023OCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX512T1023OCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX512T1023OCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX512T1023OCTGBFIM` writer - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX512T1023OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX512T1023OCTGBFIM_W<'a> {
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
#[doc = "Field `RX1024TMAXOCTGBFIM` reader - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX1024TMAXOCTGBFIM_R(crate::FieldReader<bool, bool>);
impl RX1024TMAXOCTGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX1024TMAXOCTGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX1024TMAXOCTGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX1024TMAXOCTGBFIM` writer - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
pub struct RX1024TMAXOCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX1024TMAXOCTGBFIM_W<'a> {
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
#[doc = "Field `RXUCGFIM` reader - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
pub struct RXUCGFIM_R(crate::FieldReader<bool, bool>);
impl RXUCGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUCGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUCGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUCGFIM` writer - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
pub struct RXUCGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUCGFIM_W<'a> {
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
#[doc = "Field `RXLENERFIM` reader - MMC Receive Length Error Frame Counter Interrupt Mask"]
pub struct RXLENERFIM_R(crate::FieldReader<bool, bool>);
impl RXLENERFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXLENERFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXLENERFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLENERFIM` writer - MMC Receive Length Error Frame Counter Interrupt Mask"]
pub struct RXLENERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLENERFIM_W<'a> {
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
#[doc = "Field `RXORANGEFIM` reader - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
pub struct RXORANGEFIM_R(crate::FieldReader<bool, bool>);
impl RXORANGEFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXORANGEFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXORANGEFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXORANGEFIM` writer - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
pub struct RXORANGEFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORANGEFIM_W<'a> {
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
#[doc = "Field `RXPAUSFIM` reader - MMC Receive Pause Frame Counter Interrupt Mask"]
pub struct RXPAUSFIM_R(crate::FieldReader<bool, bool>);
impl RXPAUSFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXPAUSFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPAUSFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPAUSFIM` writer - MMC Receive Pause Frame Counter Interrupt Mask"]
pub struct RXPAUSFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPAUSFIM_W<'a> {
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
#[doc = "Field `RXFOVFIM` reader - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
pub struct RXFOVFIM_R(crate::FieldReader<bool, bool>);
impl RXFOVFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFOVFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFOVFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFOVFIM` writer - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
pub struct RXFOVFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFOVFIM_W<'a> {
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
#[doc = "Field `RXVLANGBFIM` reader - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
pub struct RXVLANGBFIM_R(crate::FieldReader<bool, bool>);
impl RXVLANGBFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXVLANGBFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXVLANGBFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXVLANGBFIM` writer - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
pub struct RXVLANGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXVLANGBFIM_W<'a> {
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
#[doc = "Field `RXWDOGFIM` reader - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
pub struct RXWDOGFIM_R(crate::FieldReader<bool, bool>);
impl RXWDOGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXWDOGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXWDOGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXWDOGFIM` writer - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
pub struct RXWDOGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXWDOGFIM_W<'a> {
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
#[doc = "Field `RXRCVERRFIM` reader - MMC Receive Error Frame Counter Interrupt Mask"]
pub struct RXRCVERRFIM_R(crate::FieldReader<bool, bool>);
impl RXRCVERRFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRCVERRFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRCVERRFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRCVERRFIM` writer - MMC Receive Error Frame Counter Interrupt Mask"]
pub struct RXRCVERRFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRCVERRFIM_W<'a> {
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
#[doc = "Field `RXCTRLFIM` reader - MMC Receive Control Frame Counter Interrupt Mask"]
pub struct RXCTRLFIM_R(crate::FieldReader<bool, bool>);
impl RXCTRLFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXCTRLFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCTRLFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCTRLFIM` writer - MMC Receive Control Frame Counter Interrupt Mask"]
pub struct RXCTRLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCTRLFIM_W<'a> {
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
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgbfrmim(&self) -> RXGBFRMIM_R {
        RXGBFRMIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgboctim(&self) -> RXGBOCTIM_R {
        RXGBOCTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgoctim(&self) -> RXGOCTIM_R {
        RXGOCTIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxbcgfim(&self) -> RXBCGFIM_R {
        RXBCGFIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxmcgfim(&self) -> RXMCGFIM_R {
        RXMCGFIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxcrcerfim(&self) -> RXCRCERFIM_R {
        RXCRCERFIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxalgnerfim(&self) -> RXALGNERFIM_R {
        RXALGNERFIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxruntfim(&self) -> RXRUNTFIM_R {
        RXRUNTFIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxjaberfim(&self) -> RXJABERFIM_R {
        RXJABERFIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxusizegfim(&self) -> RXUSIZEGFIM_R {
        RXUSIZEGFIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxosizegfim(&self) -> RXOSIZEGFIM_R {
        RXOSIZEGFIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx64octgbfim(&self) -> RX64OCTGBFIM_R {
        RX64OCTGBFIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx65t127octgbfim(&self) -> RX65T127OCTGBFIM_R {
        RX65T127OCTGBFIM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx128t255octgbfim(&self) -> RX128T255OCTGBFIM_R {
        RX128T255OCTGBFIM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx256t511octgbfim(&self) -> RX256T511OCTGBFIM_R {
        RX256T511OCTGBFIM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx512t1023octgbfim(&self) -> RX512T1023OCTGBFIM_R {
        RX512T1023OCTGBFIM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfim(&self) -> RX1024TMAXOCTGBFIM_R {
        RX1024TMAXOCTGBFIM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxucgfim(&self) -> RXUCGFIM_R {
        RXUCGFIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxlenerfim(&self) -> RXLENERFIM_R {
        RXLENERFIM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxorangefim(&self) -> RXORANGEFIM_R {
        RXORANGEFIM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxpausfim(&self) -> RXPAUSFIM_R {
        RXPAUSFIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxfovfim(&self) -> RXFOVFIM_R {
        RXFOVFIM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxvlangbfim(&self) -> RXVLANGBFIM_R {
        RXVLANGBFIM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxwdogfim(&self) -> RXWDOGFIM_R {
        RXWDOGFIM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxrcverrfim(&self) -> RXRCVERRFIM_R {
        RXRCVERRFIM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxctrlfim(&self) -> RXCTRLFIM_R {
        RXCTRLFIM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgbfrmim(&mut self) -> RXGBFRMIM_W {
        RXGBFRMIM_W { w: self }
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgboctim(&mut self) -> RXGBOCTIM_W {
        RXGBOCTIM_W { w: self }
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgoctim(&mut self) -> RXGOCTIM_W {
        RXGOCTIM_W { w: self }
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxbcgfim(&mut self) -> RXBCGFIM_W {
        RXBCGFIM_W { w: self }
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxmcgfim(&mut self) -> RXMCGFIM_W {
        RXMCGFIM_W { w: self }
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxcrcerfim(&mut self) -> RXCRCERFIM_W {
        RXCRCERFIM_W { w: self }
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxalgnerfim(&mut self) -> RXALGNERFIM_W {
        RXALGNERFIM_W { w: self }
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxruntfim(&mut self) -> RXRUNTFIM_W {
        RXRUNTFIM_W { w: self }
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxjaberfim(&mut self) -> RXJABERFIM_W {
        RXJABERFIM_W { w: self }
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxusizegfim(&mut self) -> RXUSIZEGFIM_W {
        RXUSIZEGFIM_W { w: self }
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxosizegfim(&mut self) -> RXOSIZEGFIM_W {
        RXOSIZEGFIM_W { w: self }
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx64octgbfim(&mut self) -> RX64OCTGBFIM_W {
        RX64OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx65t127octgbfim(&mut self) -> RX65T127OCTGBFIM_W {
        RX65T127OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx128t255octgbfim(&mut self) -> RX128T255OCTGBFIM_W {
        RX128T255OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx256t511octgbfim(&mut self) -> RX256T511OCTGBFIM_W {
        RX256T511OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx512t1023octgbfim(&mut self) -> RX512T1023OCTGBFIM_W {
        RX512T1023OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfim(&mut self) -> RX1024TMAXOCTGBFIM_W {
        RX1024TMAXOCTGBFIM_W { w: self }
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxucgfim(&mut self) -> RXUCGFIM_W {
        RXUCGFIM_W { w: self }
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxlenerfim(&mut self) -> RXLENERFIM_W {
        RXLENERFIM_W { w: self }
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxorangefim(&mut self) -> RXORANGEFIM_W {
        RXORANGEFIM_W { w: self }
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxpausfim(&mut self) -> RXPAUSFIM_W {
        RXPAUSFIM_W { w: self }
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxfovfim(&mut self) -> RXFOVFIM_W {
        RXFOVFIM_W { w: self }
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxvlangbfim(&mut self) -> RXVLANGBFIM_W {
        RXVLANGBFIM_W { w: self }
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxwdogfim(&mut self) -> RXWDOGFIM_W {
        RXWDOGFIM_W { w: self }
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxrcverrfim(&mut self) -> RXRCVERRFIM_W {
        RXRCVERRFIM_W { w: self }
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxctrlfim(&mut self) -> RXCTRLFIM_W {
        RXCTRLFIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Reveive Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_receive_interrupt_mask](index.html) module"]
pub struct MMC_RECEIVE_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_RECEIVE_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_receive_interrupt_mask::R](R) reader structure"]
impl crate::Readable for MMC_RECEIVE_INTERRUPT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_receive_interrupt_mask::W](W) writer structure"]
impl crate::Writable for MMC_RECEIVE_INTERRUPT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_RECEIVE_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_RECEIVE_INTERRUPT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
