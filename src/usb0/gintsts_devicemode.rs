#[doc = "Register `GINTSTS_DEVICEMODE` reader"]
pub struct R(crate::R<GINTSTS_DEVICEMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTSTS_DEVICEMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTSTS_DEVICEMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTSTS_DEVICEMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTSTS_DEVICEMODE` writer"]
pub struct W(crate::W<GINTSTS_DEVICEMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTSTS_DEVICEMODE_SPEC>;
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
impl From<crate::W<GINTSTS_DEVICEMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTSTS_DEVICEMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Current Mode of Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURMOD_A {
    #[doc = "0: Device mode"]
    VALUE1 = 0,
    #[doc = "1: Host mode"]
    VALUE2 = 1,
}
impl From<CURMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CURMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CurMod` reader - Current Mode of Operation"]
pub struct CURMOD_R(crate::FieldReader<bool, CURMOD_A>);
impl CURMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CURMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURMOD_A {
        match self.bits {
            false => CURMOD_A::VALUE1,
            true => CURMOD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CURMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CURMOD_A::VALUE2
    }
}
impl core::ops::Deref for CURMOD_R {
    type Target = crate::FieldReader<bool, CURMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ModeMis` reader - Mode Mismatch Interrupt"]
pub struct MODEMIS_R(crate::FieldReader<bool, bool>);
impl MODEMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ModeMis` writer - Mode Mismatch Interrupt"]
pub struct MODEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEMIS_W<'a> {
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
#[doc = "Field `OTGInt` reader - OTG Interrupt"]
pub struct OTGINT_R(crate::FieldReader<bool, bool>);
impl OTGINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTGINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Sof` reader - Start of Frame"]
pub struct SOF_R(crate::FieldReader<bool, bool>);
impl SOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Sof` writer - Start of Frame"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
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
#[doc = "Field `RxFLvl` reader - RxFIFO Non-Empty"]
pub struct RXFLVL_R(crate::FieldReader<bool, bool>);
impl RXFLVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFLVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINNakEff` reader - Global IN Non-Periodic NAK Effective"]
pub struct GINNAKEFF_R(crate::FieldReader<bool, bool>);
impl GINNAKEFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GINNAKEFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINNAKEFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GOUTNakEff` reader - Global OUT NAK Effective"]
pub struct GOUTNAKEFF_R(crate::FieldReader<bool, bool>);
impl GOUTNAKEFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GOUTNAKEFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GOUTNAKEFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ErlySusp` reader - Early Suspend"]
pub struct ERLYSUSP_R(crate::FieldReader<bool, bool>);
impl ERLYSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERLYSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERLYSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ErlySusp` writer - Early Suspend"]
pub struct ERLYSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ERLYSUSP_W<'a> {
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
#[doc = "Field `USBSusp` reader - USB Suspend"]
pub struct USBSUSP_R(crate::FieldReader<bool, bool>);
impl USBSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSusp` writer - USB Suspend"]
pub struct USBSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSUSP_W<'a> {
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
#[doc = "Field `USBRst` reader - USB Reset"]
pub struct USBRST_R(crate::FieldReader<bool, bool>);
impl USBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBRst` writer - USB Reset"]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
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
#[doc = "Field `EnumDone` reader - Enumeration Done"]
pub struct ENUMDONE_R(crate::FieldReader<bool, bool>);
impl ENUMDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENUMDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENUMDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EnumDone` writer - Enumeration Done"]
pub struct ENUMDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMDONE_W<'a> {
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
#[doc = "Field `ISOOutDrop` reader - Isochronous OUT Packet Dropped Interrupt"]
pub struct ISOOUTDROP_R(crate::FieldReader<bool, bool>);
impl ISOOUTDROP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOOUTDROP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOOUTDROP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOOutDrop` writer - Isochronous OUT Packet Dropped Interrupt"]
pub struct ISOOUTDROP_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOOUTDROP_W<'a> {
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
#[doc = "Field `EOPF` reader - End of Periodic Frame Interrupt"]
pub struct EOPF_R(crate::FieldReader<bool, bool>);
impl EOPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOPF` writer - End of Periodic Frame Interrupt"]
pub struct EOPF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPF_W<'a> {
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
#[doc = "Field `IEPInt` reader - IN Endpoints Interrupt"]
pub struct IEPINT_R(crate::FieldReader<bool, bool>);
impl IEPINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IEPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEPINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEPInt` reader - OUT Endpoints Interrupt"]
pub struct OEPINT_R(crate::FieldReader<bool, bool>);
impl OEPINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEPINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `incompISOIN` reader - Incomplete Isochronous IN Transfer"]
pub struct INCOMPISOIN_R(crate::FieldReader<bool, bool>);
impl INCOMPISOIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCOMPISOIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCOMPISOIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `incompISOIN` writer - Incomplete Isochronous IN Transfer"]
pub struct INCOMPISOIN_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPISOIN_W<'a> {
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
#[doc = "Field `incomplSOOUT` reader - Incomplete Isochronous OUT Transfer"]
pub struct INCOMPLSOOUT_R(crate::FieldReader<bool, bool>);
impl INCOMPLSOOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCOMPLSOOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCOMPLSOOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `incomplSOOUT` writer - Incomplete Isochronous OUT Transfer"]
pub struct INCOMPLSOOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPLSOOUT_W<'a> {
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
#[doc = "Field `ConIDStsChng` reader - Connector ID Status Change"]
pub struct CONIDSTSCHNG_R(crate::FieldReader<bool, bool>);
impl CONIDSTSCHNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONIDSTSCHNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONIDSTSCHNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ConIDStsChng` writer - Connector ID Status Change"]
pub struct CONIDSTSCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONIDSTSCHNG_W<'a> {
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
#[doc = "Field `SessReqInt` reader - Session Request/New Session Detected Interrupt"]
pub struct SESSREQINT_R(crate::FieldReader<bool, bool>);
impl SESSREQINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESSREQINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESSREQINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SessReqInt` writer - Session Request/New Session Detected Interrupt"]
pub struct SESSREQINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSREQINT_W<'a> {
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
#[doc = "Field `WkUpInt` reader - Resume/Remote Wakeup Detected Interrupt"]
pub struct WKUPINT_R(crate::FieldReader<bool, bool>);
impl WKUPINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WkUpInt` writer - Resume/Remote Wakeup Detected Interrupt"]
pub struct WKUPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPINT_W<'a> {
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
    #[doc = "Bit 0 - Current Mode of Operation"]
    #[inline(always)]
    pub fn cur_mod(&self) -> CURMOD_R {
        CURMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn mode_mis(&self) -> MODEMIS_R {
        MODEMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Non-Empty"]
    #[inline(always)]
    pub fn rx_flvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Global IN Non-Periodic NAK Effective"]
    #[inline(always)]
    pub fn ginnak_eff(&self) -> GINNAKEFF_R {
        GINNAKEFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective"]
    #[inline(always)]
    pub fn goutnak_eff(&self) -> GOUTNAKEFF_R {
        GOUTNAKEFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    pub fn erly_susp(&self) -> ERLYSUSP_R {
        ERLYSUSP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    pub fn enum_done(&self) -> ENUMDONE_R {
        ENUMDONE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    pub fn isoout_drop(&self) -> ISOOUTDROP_R {
        ISOOUTDROP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    pub fn incomp_isoin(&self) -> INCOMPISOIN_R {
        INCOMPISOIN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer"]
    #[inline(always)]
    pub fn incompl_soout(&self) -> INCOMPLSOOUT_R {
        INCOMPLSOOUT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn con_idsts_chng(&self) -> CONIDSTSCHNG_R {
        CONIDSTSCHNG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    pub fn sess_req_int(&self) -> SESSREQINT_R {
        SESSREQINT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wk_up_int(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn mode_mis(&mut self) -> MODEMIS_W {
        MODEMIS_W { w: self }
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    pub fn erly_susp(&mut self) -> ERLYSUSP_W {
        ERLYSUSP_W { w: self }
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W {
        USBSUSP_W { w: self }
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    pub fn enum_done(&mut self) -> ENUMDONE_W {
        ENUMDONE_W { w: self }
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    pub fn isoout_drop(&mut self) -> ISOOUTDROP_W {
        ISOOUTDROP_W { w: self }
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W {
        EOPF_W { w: self }
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    pub fn incomp_isoin(&mut self) -> INCOMPISOIN_W {
        INCOMPISOIN_W { w: self }
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer"]
    #[inline(always)]
    pub fn incompl_soout(&mut self) -> INCOMPLSOOUT_W {
        INCOMPLSOOUT_W { w: self }
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn con_idsts_chng(&mut self) -> CONIDSTSCHNG_W {
        CONIDSTSCHNG_W { w: self }
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    pub fn sess_req_int(&mut self) -> SESSREQINT_W {
        SESSREQINT_W { w: self }
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wk_up_int(&mut self) -> WKUPINT_W {
        WKUPINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Register \\[DEVICEMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintsts_devicemode](index.html) module"]
pub struct GINTSTS_DEVICEMODE_SPEC;
impl crate::RegisterSpec for GINTSTS_DEVICEMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintsts_devicemode::R](R) reader structure"]
impl crate::Readable for GINTSTS_DEVICEMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintsts_devicemode::W](W) writer structure"]
impl crate::Writable for GINTSTS_DEVICEMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTSTS_DEVICEMODE to value 0x1400_0020"]
impl crate::Resettable for GINTSTS_DEVICEMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1400_0020
    }
}
