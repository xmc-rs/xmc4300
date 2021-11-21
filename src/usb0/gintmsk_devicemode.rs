#[doc = "Register `GINTMSK_DEVICEMODE` reader"]
pub struct R(crate::R<GINTMSK_DEVICEMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTMSK_DEVICEMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTMSK_DEVICEMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTMSK_DEVICEMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTMSK_DEVICEMODE` writer"]
pub struct W(crate::W<GINTMSK_DEVICEMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTMSK_DEVICEMODE_SPEC>;
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
impl From<crate::W<GINTMSK_DEVICEMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTMSK_DEVICEMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ModeMisMsk` reader - Mode Mismatch Interrupt Mask"]
pub struct MODEMISMSK_R(crate::FieldReader<bool, bool>);
impl MODEMISMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODEMISMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODEMISMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ModeMisMsk` writer - Mode Mismatch Interrupt Mask"]
pub struct MODEMISMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEMISMSK_W<'a> {
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
#[doc = "Field `OTGIntMsk` reader - OTG Interrupt Mask"]
pub struct OTGINTMSK_R(crate::FieldReader<bool, bool>);
impl OTGINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTGINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTGIntMsk` writer - OTG Interrupt Mask"]
pub struct OTGINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGINTMSK_W<'a> {
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
#[doc = "Field `SofMsk` reader - Start of Frame Mask"]
pub struct SOFMSK_R(crate::FieldReader<bool, bool>);
impl SOFMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SofMsk` writer - Start of Frame Mask"]
pub struct SOFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFMSK_W<'a> {
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
#[doc = "Field `RxFLvlMsk` reader - Receive FIFO Non-Empty Mask"]
pub struct RXFLVLMSK_R(crate::FieldReader<bool, bool>);
impl RXFLVLMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFLVLMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFLVLMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RxFLvlMsk` writer - Receive FIFO Non-Empty Mask"]
pub struct RXFLVLMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLVLMSK_W<'a> {
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
#[doc = "Field `GINNakEffMsk` reader - Global Non-periodic IN NAK Effective Mask"]
pub struct GINNAKEFFMSK_R(crate::FieldReader<bool, bool>);
impl GINNAKEFFMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GINNAKEFFMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINNAKEFFMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINNakEffMsk` writer - Global Non-periodic IN NAK Effective Mask"]
pub struct GINNAKEFFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> GINNAKEFFMSK_W<'a> {
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
#[doc = "Field `GOUTNakEffMsk` reader - Global OUT NAK Effective Mask"]
pub struct GOUTNAKEFFMSK_R(crate::FieldReader<bool, bool>);
impl GOUTNAKEFFMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GOUTNAKEFFMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GOUTNAKEFFMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GOUTNakEffMsk` writer - Global OUT NAK Effective Mask"]
pub struct GOUTNAKEFFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> GOUTNAKEFFMSK_W<'a> {
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
#[doc = "Field `ErlySuspMsk` reader - Early Suspend Mask"]
pub struct ERLYSUSPMSK_R(crate::FieldReader<bool, bool>);
impl ERLYSUSPMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERLYSUSPMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERLYSUSPMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ErlySuspMsk` writer - Early Suspend Mask"]
pub struct ERLYSUSPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERLYSUSPMSK_W<'a> {
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
#[doc = "Field `USBSuspMsk` reader - USB Suspend Mask"]
pub struct USBSUSPMSK_R(crate::FieldReader<bool, bool>);
impl USBSUSPMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSUSPMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBSUSPMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSuspMsk` writer - USB Suspend Mask"]
pub struct USBSUSPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSUSPMSK_W<'a> {
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
#[doc = "Field `USBRstMsk` reader - USB Reset Mask"]
pub struct USBRSTMSK_R(crate::FieldReader<bool, bool>);
impl USBRSTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBRSTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBRSTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBRstMsk` writer - USB Reset Mask"]
pub struct USBRSTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRSTMSK_W<'a> {
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
#[doc = "Field `EnumDoneMsk` reader - Enumeration Done Mask"]
pub struct ENUMDONEMSK_R(crate::FieldReader<bool, bool>);
impl ENUMDONEMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENUMDONEMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENUMDONEMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EnumDoneMsk` writer - Enumeration Done Mask"]
pub struct ENUMDONEMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMDONEMSK_W<'a> {
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
#[doc = "Field `ISOOutDropMsk` reader - Isochronous OUT Packet Dropped Interrupt Mask"]
pub struct ISOOUTDROPMSK_R(crate::FieldReader<bool, bool>);
impl ISOOUTDROPMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOOUTDROPMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOOUTDROPMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOOutDropMsk` writer - Isochronous OUT Packet Dropped Interrupt Mask"]
pub struct ISOOUTDROPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOOUTDROPMSK_W<'a> {
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
#[doc = "Field `EOPFMsk` reader - End of Periodic Frame Interrupt Mask"]
pub struct EOPFMSK_R(crate::FieldReader<bool, bool>);
impl EOPFMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOPFMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOPFMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOPFMsk` writer - End of Periodic Frame Interrupt Mask"]
pub struct EOPFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPFMSK_W<'a> {
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
#[doc = "Field `IEPIntMsk` reader - IN Endpoints Interrupt Mask"]
pub struct IEPINTMSK_R(crate::FieldReader<bool, bool>);
impl IEPINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        IEPINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEPINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEPIntMsk` writer - IN Endpoints Interrupt Mask"]
pub struct IEPINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPINTMSK_W<'a> {
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
#[doc = "Field `OEPIntMsk` reader - OUT Endpoints Interrupt Mask"]
pub struct OEPINTMSK_R(crate::FieldReader<bool, bool>);
impl OEPINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEPINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEPINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEPIntMsk` writer - OUT Endpoints Interrupt Mask"]
pub struct OEPINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OEPINTMSK_W<'a> {
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
#[doc = "Field `incompISOINMsk` reader - Incomplete Isochronous IN Transfer Mask"]
pub struct INCOMPISOINMSK_R(crate::FieldReader<bool, bool>);
impl INCOMPISOINMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCOMPISOINMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCOMPISOINMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `incompISOINMsk` writer - Incomplete Isochronous IN Transfer Mask"]
pub struct INCOMPISOINMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPISOINMSK_W<'a> {
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
#[doc = "Field `incomplSOOUTMsk` reader - Incomplete Isochronous OUT Transfer Mask"]
pub struct INCOMPLSOOUTMSK_R(crate::FieldReader<bool, bool>);
impl INCOMPLSOOUTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCOMPLSOOUTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCOMPLSOOUTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `incomplSOOUTMsk` writer - Incomplete Isochronous OUT Transfer Mask"]
pub struct INCOMPLSOOUTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPLSOOUTMSK_W<'a> {
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
#[doc = "Field `ConIDStsChngMsk` reader - Connector ID Status Change Mask"]
pub struct CONIDSTSCHNGMSK_R(crate::FieldReader<bool, bool>);
impl CONIDSTSCHNGMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONIDSTSCHNGMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONIDSTSCHNGMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ConIDStsChngMsk` writer - Connector ID Status Change Mask"]
pub struct CONIDSTSCHNGMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CONIDSTSCHNGMSK_W<'a> {
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
#[doc = "Field `DisconnIntMsk` reader - Disconnect Detected Interrupt Mask"]
pub struct DISCONNINTMSK_R(crate::FieldReader<bool, bool>);
impl DISCONNINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISCONNINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCONNINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DisconnIntMsk` writer - Disconnect Detected Interrupt Mask"]
pub struct DISCONNINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONNINTMSK_W<'a> {
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
#[doc = "Field `SessReqIntMsk` reader - Session Request/New Session Detected Interrupt Mask"]
pub struct SESSREQINTMSK_R(crate::FieldReader<bool, bool>);
impl SESSREQINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESSREQINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESSREQINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SessReqIntMsk` writer - Session Request/New Session Detected Interrupt Mask"]
pub struct SESSREQINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSREQINTMSK_W<'a> {
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
#[doc = "Field `WkUpIntMsk` reader - Resume/Remote Wakeup Detected Interrupt Mask"]
pub struct WKUPINTMSK_R(crate::FieldReader<bool, bool>);
impl WKUPINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WkUpIntMsk` writer - Resume/Remote Wakeup Detected Interrupt Mask"]
pub struct WKUPINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPINTMSK_W<'a> {
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
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn mode_mis_msk(&self) -> MODEMISMSK_R {
        MODEMISMSK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    pub fn otgint_msk(&self) -> OTGINTMSK_R {
        OTGINTMSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sof_msk(&self) -> SOFMSK_R {
        SOFMSK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rx_flvl_msk(&self) -> RXFLVLMSK_R {
        RXFLVLMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    pub fn ginnak_eff_msk(&self) -> GINNAKEFFMSK_R {
        GINNAKEFFMSK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    pub fn goutnak_eff_msk(&self) -> GOUTNAKEFFMSK_R {
        GOUTNAKEFFMSK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    pub fn erly_susp_msk(&self) -> ERLYSUSPMSK_R {
        ERLYSUSPMSK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    pub fn usbsusp_msk(&self) -> USBSUSPMSK_R {
        USBSUSPMSK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    pub fn usbrst_msk(&self) -> USBRSTMSK_R {
        USBRSTMSK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    pub fn enum_done_msk(&self) -> ENUMDONEMSK_R {
        ENUMDONEMSK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    pub fn isoout_drop_msk(&self) -> ISOOUTDROPMSK_R {
        ISOOUTDROPMSK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EOPFMSK_R {
        EOPFMSK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn iepint_msk(&self) -> IEPINTMSK_R {
        IEPINTMSK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn oepint_msk(&self) -> OEPINTMSK_R {
        OEPINTMSK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    pub fn incomp_isoinmsk(&self) -> INCOMPISOINMSK_R {
        INCOMPISOINMSK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer Mask"]
    #[inline(always)]
    pub fn incompl_sooutmsk(&self) -> INCOMPLSOOUTMSK_R {
        INCOMPLSOOUTMSK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    pub fn con_idsts_chng_msk(&self) -> CONIDSTSCHNGMSK_R {
        CONIDSTSCHNGMSK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    pub fn disconn_int_msk(&self) -> DISCONNINTMSK_R {
        DISCONNINTMSK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    pub fn sess_req_int_msk(&self) -> SESSREQINTMSK_R {
        SESSREQINTMSK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wk_up_int_msk(&self) -> WKUPINTMSK_R {
        WKUPINTMSK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn mode_mis_msk(&mut self) -> MODEMISMSK_W {
        MODEMISMSK_W { w: self }
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    pub fn otgint_msk(&mut self) -> OTGINTMSK_W {
        OTGINTMSK_W { w: self }
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sof_msk(&mut self) -> SOFMSK_W {
        SOFMSK_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rx_flvl_msk(&mut self) -> RXFLVLMSK_W {
        RXFLVLMSK_W { w: self }
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    pub fn ginnak_eff_msk(&mut self) -> GINNAKEFFMSK_W {
        GINNAKEFFMSK_W { w: self }
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    pub fn goutnak_eff_msk(&mut self) -> GOUTNAKEFFMSK_W {
        GOUTNAKEFFMSK_W { w: self }
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    pub fn erly_susp_msk(&mut self) -> ERLYSUSPMSK_W {
        ERLYSUSPMSK_W { w: self }
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    pub fn usbsusp_msk(&mut self) -> USBSUSPMSK_W {
        USBSUSPMSK_W { w: self }
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    pub fn usbrst_msk(&mut self) -> USBRSTMSK_W {
        USBRSTMSK_W { w: self }
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    pub fn enum_done_msk(&mut self) -> ENUMDONEMSK_W {
        ENUMDONEMSK_W { w: self }
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    pub fn isoout_drop_msk(&mut self) -> ISOOUTDROPMSK_W {
        ISOOUTDROPMSK_W { w: self }
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    pub fn eopfmsk(&mut self) -> EOPFMSK_W {
        EOPFMSK_W { w: self }
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn iepint_msk(&mut self) -> IEPINTMSK_W {
        IEPINTMSK_W { w: self }
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn oepint_msk(&mut self) -> OEPINTMSK_W {
        OEPINTMSK_W { w: self }
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    pub fn incomp_isoinmsk(&mut self) -> INCOMPISOINMSK_W {
        INCOMPISOINMSK_W { w: self }
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer Mask"]
    #[inline(always)]
    pub fn incompl_sooutmsk(&mut self) -> INCOMPLSOOUTMSK_W {
        INCOMPLSOOUTMSK_W { w: self }
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    pub fn con_idsts_chng_msk(&mut self) -> CONIDSTSCHNGMSK_W {
        CONIDSTSCHNGMSK_W { w: self }
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    pub fn disconn_int_msk(&mut self) -> DISCONNINTMSK_W {
        DISCONNINTMSK_W { w: self }
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    pub fn sess_req_int_msk(&mut self) -> SESSREQINTMSK_W {
        SESSREQINTMSK_W { w: self }
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wk_up_int_msk(&mut self) -> WKUPINTMSK_W {
        WKUPINTMSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register \\[DEVICEMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintmsk_devicemode](index.html) module"]
pub struct GINTMSK_DEVICEMODE_SPEC;
impl crate::RegisterSpec for GINTMSK_DEVICEMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintmsk_devicemode::R](R) reader structure"]
impl crate::Readable for GINTMSK_DEVICEMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintmsk_devicemode::W](W) writer structure"]
impl crate::Writable for GINTMSK_DEVICEMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTMSK_DEVICEMODE to value 0"]
impl crate::Resettable for GINTMSK_DEVICEMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
