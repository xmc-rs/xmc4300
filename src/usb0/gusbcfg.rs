#[doc = "Register `GUSBCFG` reader"]
pub struct R(crate::R<GUSBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GUSBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GUSBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GUSBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GUSBCFG` writer"]
pub struct W(crate::W<GUSBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GUSBCFG_SPEC>;
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
impl From<crate::W<GUSBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GUSBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOutCal` reader - FS Timeout Calibration"]
pub struct TOUTCAL_R(crate::FieldReader<u8, u8>);
impl TOUTCAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOUTCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUTCAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOutCal` writer - FS Timeout Calibration"]
pub struct TOUTCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "USB 1.1 Full-Speed Serial Transceiver Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYSEL_A {
    #[doc = "1: USB 1.1 full-speed serial transceiver"]
    VALUE2 = 1,
}
impl From<PHYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PHYSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYSel` reader - USB 1.1 Full-Speed Serial Transceiver Select"]
pub struct PHYSEL_R(crate::FieldReader<bool, PHYSEL_A>);
impl PHYSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PHYSEL_A> {
        match self.bits {
            true => Some(PHYSEL_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PHYSEL_A::VALUE2
    }
}
impl core::ops::Deref for PHYSEL_R {
    type Target = crate::FieldReader<bool, PHYSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SRP-Capable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRPCAP_A {
    #[doc = "0: SRP capability is not enabled."]
    VALUE1 = 0,
    #[doc = "1: SRP capability is enabled."]
    VALUE2 = 1,
}
impl From<SRPCAP_A> for bool {
    #[inline(always)]
    fn from(variant: SRPCAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRPCap` reader - SRP-Capable"]
pub struct SRPCAP_R(crate::FieldReader<bool, SRPCAP_A>);
impl SRPCAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRPCAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRPCAP_A {
        match self.bits {
            false => SRPCAP_A::VALUE1,
            true => SRPCAP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SRPCAP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SRPCAP_A::VALUE2
    }
}
impl core::ops::Deref for SRPCAP_R {
    type Target = crate::FieldReader<bool, SRPCAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRPCap` writer - SRP-Capable"]
pub struct SRPCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPCAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRPCAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRP capability is not enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRPCAP_A::VALUE1)
    }
    #[doc = "SRP capability is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRPCAP_A::VALUE2)
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
#[doc = "HNP-Capable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HNPCAP_A {
    #[doc = "0: HNP capability is not enabled."]
    VALUE1 = 0,
    #[doc = "1: HNP capability is enabled."]
    VALUE2 = 1,
}
impl From<HNPCAP_A> for bool {
    #[inline(always)]
    fn from(variant: HNPCAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HNPCap` reader - HNP-Capable"]
pub struct HNPCAP_R(crate::FieldReader<bool, HNPCAP_A>);
impl HNPCAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNPCAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HNPCAP_A {
        match self.bits {
            false => HNPCAP_A::VALUE1,
            true => HNPCAP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HNPCAP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HNPCAP_A::VALUE2
    }
}
impl core::ops::Deref for HNPCAP_R {
    type Target = crate::FieldReader<bool, HNPCAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPCap` writer - HNP-Capable"]
pub struct HNPCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPCAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HNPCAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HNP capability is not enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HNPCAP_A::VALUE1)
    }
    #[doc = "HNP capability is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HNPCAP_A::VALUE2)
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
#[doc = "Field `USBTrdTim` reader - USB Turnaround Time"]
pub struct USBTRDTIM_R(crate::FieldReader<u8, u8>);
impl USBTRDTIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBTRDTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBTRDTIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBTrdTim` writer - USB Turnaround Time"]
pub struct USBTRDTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBTRDTIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "UTMIFS Interface Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGI2CSEL_A {
    #[doc = "0: UTMI USB 1.1 Full-Speed interface for OTG signals"]
    VALUE1 = 0,
}
impl From<OTGI2CSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OTGI2CSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OtgI2CSel` reader - UTMIFS Interface Select"]
pub struct OTGI2CSEL_R(crate::FieldReader<bool, OTGI2CSEL_A>);
impl OTGI2CSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGI2CSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OTGI2CSEL_A> {
        match self.bits {
            false => Some(OTGI2CSEL_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == OTGI2CSEL_A::VALUE1
    }
}
impl core::ops::Deref for OTGI2CSEL_R {
    type Target = crate::FieldReader<bool, OTGI2CSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OtgI2CSel` writer - UTMIFS Interface Select"]
pub struct OTGI2CSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGI2CSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTGI2CSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UTMI USB 1.1 Full-Speed interface for OTG signals"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OTGI2CSEL_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Tx End Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENDDELAY_A {
    #[doc = "0: Normal mode"]
    VALUE1 = 0,
    #[doc = "1: Introduce Tx end delay timers"]
    VALUE2 = 1,
}
impl From<TXENDDELAY_A> for bool {
    #[inline(always)]
    fn from(variant: TXENDDELAY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TxEndDelay` reader - Tx End Delay"]
pub struct TXENDDELAY_R(crate::FieldReader<bool, TXENDDELAY_A>);
impl TXENDDELAY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXENDDELAY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENDDELAY_A {
        match self.bits {
            false => TXENDDELAY_A::VALUE1,
            true => TXENDDELAY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TXENDDELAY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TXENDDELAY_A::VALUE2
    }
}
impl core::ops::Deref for TXENDDELAY_R {
    type Target = crate::FieldReader<bool, TXENDDELAY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TxEndDelay` writer - Tx End Delay"]
pub struct TXENDDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDDELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXENDDELAY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXENDDELAY_A::VALUE1)
    }
    #[doc = "Introduce Tx end delay timers"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXENDDELAY_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Force Host Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEHSTMODE_A {
    #[doc = "0: Normal Mode"]
    VALUE1 = 0,
    #[doc = "1: Force Host Mode"]
    VALUE2 = 1,
}
impl From<FORCEHSTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEHSTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ForceHstMode` reader - Force Host Mode"]
pub struct FORCEHSTMODE_R(crate::FieldReader<bool, FORCEHSTMODE_A>);
impl FORCEHSTMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCEHSTMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEHSTMODE_A {
        match self.bits {
            false => FORCEHSTMODE_A::VALUE1,
            true => FORCEHSTMODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FORCEHSTMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FORCEHSTMODE_A::VALUE2
    }
}
impl core::ops::Deref for FORCEHSTMODE_R {
    type Target = crate::FieldReader<bool, FORCEHSTMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ForceHstMode` writer - Force Host Mode"]
pub struct FORCEHSTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEHSTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEHSTMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FORCEHSTMODE_A::VALUE1)
    }
    #[doc = "Force Host Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FORCEHSTMODE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Force Device Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEDEVMODE_A {
    #[doc = "0: Normal Mode"]
    VALUE1 = 0,
    #[doc = "1: Force Device Mode"]
    VALUE2 = 1,
}
impl From<FORCEDEVMODE_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEDEVMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ForceDevMode` reader - Force Device Mode"]
pub struct FORCEDEVMODE_R(crate::FieldReader<bool, FORCEDEVMODE_A>);
impl FORCEDEVMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCEDEVMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEDEVMODE_A {
        match self.bits {
            false => FORCEDEVMODE_A::VALUE1,
            true => FORCEDEVMODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FORCEDEVMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FORCEDEVMODE_A::VALUE2
    }
}
impl core::ops::Deref for FORCEDEVMODE_R {
    type Target = crate::FieldReader<bool, FORCEDEVMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ForceDevMode` writer - Force Device Mode"]
pub struct FORCEDEVMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEDEVMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEDEVMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FORCEDEVMODE_A::VALUE1)
    }
    #[doc = "Force Device Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FORCEDEVMODE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `CTP` reader - Corrupt Tx packet"]
pub struct CTP_R(crate::FieldReader<bool, bool>);
impl CTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTP` writer - Corrupt Tx packet"]
pub struct CTP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTP_W<'a> {
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
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    pub fn tout_cal(&self) -> TOUTCAL_R {
        TOUTCAL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 6 - USB 1.1 Full-Speed Serial Transceiver Select"]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SRP-Capable"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HNP-Capable"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrd_tim(&self) -> USBTRDTIM_R {
        USBTRDTIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - UTMIFS Interface Select"]
    #[inline(always)]
    pub fn otg_i2csel(&self) -> OTGI2CSEL_R {
        OTGI2CSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn tx_end_delay(&self) -> TXENDDELAY_R {
        TXENDDELAY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Force Host Mode"]
    #[inline(always)]
    pub fn force_hst_mode(&self) -> FORCEHSTMODE_R {
        FORCEHSTMODE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    pub fn force_dev_mode(&self) -> FORCEDEVMODE_R {
        FORCEDEVMODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    pub fn tout_cal(&mut self) -> TOUTCAL_W {
        TOUTCAL_W { w: self }
    }
    #[doc = "Bit 8 - SRP-Capable"]
    #[inline(always)]
    pub fn srpcap(&mut self) -> SRPCAP_W {
        SRPCAP_W { w: self }
    }
    #[doc = "Bit 9 - HNP-Capable"]
    #[inline(always)]
    pub fn hnpcap(&mut self) -> HNPCAP_W {
        HNPCAP_W { w: self }
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrd_tim(&mut self) -> USBTRDTIM_W {
        USBTRDTIM_W { w: self }
    }
    #[doc = "Bit 16 - UTMIFS Interface Select"]
    #[inline(always)]
    pub fn otg_i2csel(&mut self) -> OTGI2CSEL_W {
        OTGI2CSEL_W { w: self }
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn tx_end_delay(&mut self) -> TXENDDELAY_W {
        TXENDDELAY_W { w: self }
    }
    #[doc = "Bit 29 - Force Host Mode"]
    #[inline(always)]
    pub fn force_hst_mode(&mut self) -> FORCEHSTMODE_W {
        FORCEHSTMODE_W { w: self }
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    pub fn force_dev_mode(&mut self) -> FORCEDEVMODE_W {
        FORCEDEVMODE_W { w: self }
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctp(&mut self) -> CTP_W {
        CTP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcfg](index.html) module"]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gusbcfg::R](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1440"]
impl crate::Resettable for GUSBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1440
    }
}
