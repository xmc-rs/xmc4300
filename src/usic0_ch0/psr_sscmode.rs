#[doc = "Register `PSR_SSCMode` reader"]
pub struct R(crate::R<PSR_SSCMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SSCMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SSCMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SSCMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSR_SSCMode` writer"]
pub struct W(crate::W<PSR_SSCMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_SSCMODE_SPEC>;
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
impl From<crate::W<PSR_SSCMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_SSCMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MSLS Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSLS_A {
    #[doc = "0: The internal signal MSLS is inactive (0)."]
    VALUE1 = 0,
    #[doc = "1: The internal signal MSLS is active (1)."]
    VALUE2 = 1,
}
impl From<MSLS_A> for bool {
    #[inline(always)]
    fn from(variant: MSLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSLS` reader - MSLS Status"]
pub struct MSLS_R(crate::FieldReader<bool, MSLS_A>);
impl MSLS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSLS_A {
        match self.bits {
            false => MSLS_A::VALUE1,
            true => MSLS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSLS_A::VALUE2
    }
}
impl core::ops::Deref for MSLS_R {
    type Target = crate::FieldReader<bool, MSLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSLS` writer - MSLS Status"]
pub struct MSLS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSLS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The internal signal MSLS is inactive (0)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSLS_A::VALUE1)
    }
    #[doc = "The internal signal MSLS is active (1)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSLS_A::VALUE2)
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
#[doc = "DX2S Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DX2S_A {
    #[doc = "0: DX2S is 0."]
    VALUE1 = 0,
    #[doc = "1: DX2S is 1."]
    VALUE2 = 1,
}
impl From<DX2S_A> for bool {
    #[inline(always)]
    fn from(variant: DX2S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DX2S` reader - DX2S Status"]
pub struct DX2S_R(crate::FieldReader<bool, DX2S_A>);
impl DX2S_R {
    pub(crate) fn new(bits: bool) -> Self {
        DX2S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DX2S_A {
        match self.bits {
            false => DX2S_A::VALUE1,
            true => DX2S_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DX2S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DX2S_A::VALUE2
    }
}
impl core::ops::Deref for DX2S_R {
    type Target = crate::FieldReader<bool, DX2S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DX2S` writer - DX2S Status"]
pub struct DX2S_W<'a> {
    w: &'a mut W,
}
impl<'a> DX2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DX2S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DX2S is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DX2S_A::VALUE1)
    }
    #[doc = "DX2S is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DX2S_A::VALUE2)
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
#[doc = "MSLS Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSLSEV_A {
    #[doc = "0: The MSLS signal has not changed its state."]
    VALUE1 = 0,
    #[doc = "1: The MSLS signal has changed its state."]
    VALUE2 = 1,
}
impl From<MSLSEV_A> for bool {
    #[inline(always)]
    fn from(variant: MSLSEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSLSEV` reader - MSLS Event Detected"]
pub struct MSLSEV_R(crate::FieldReader<bool, MSLSEV_A>);
impl MSLSEV_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSLSEV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSLSEV_A {
        match self.bits {
            false => MSLSEV_A::VALUE1,
            true => MSLSEV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSLSEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSLSEV_A::VALUE2
    }
}
impl core::ops::Deref for MSLSEV_R {
    type Target = crate::FieldReader<bool, MSLSEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSLSEV` writer - MSLS Event Detected"]
pub struct MSLSEV_W<'a> {
    w: &'a mut W,
}
impl<'a> MSLSEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSLSEV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The MSLS signal has not changed its state."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSLSEV_A::VALUE1)
    }
    #[doc = "The MSLS signal has changed its state."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSLSEV_A::VALUE2)
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
#[doc = "DX2T Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DX2TEV_A {
    #[doc = "0: The DX2T signal has not been activated."]
    VALUE1 = 0,
    #[doc = "1: The DX2T signal has been activated."]
    VALUE2 = 1,
}
impl From<DX2TEV_A> for bool {
    #[inline(always)]
    fn from(variant: DX2TEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DX2TEV` reader - DX2T Event Detected"]
pub struct DX2TEV_R(crate::FieldReader<bool, DX2TEV_A>);
impl DX2TEV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DX2TEV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DX2TEV_A {
        match self.bits {
            false => DX2TEV_A::VALUE1,
            true => DX2TEV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DX2TEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DX2TEV_A::VALUE2
    }
}
impl core::ops::Deref for DX2TEV_R {
    type Target = crate::FieldReader<bool, DX2TEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DX2TEV` writer - DX2T Event Detected"]
pub struct DX2TEV_W<'a> {
    w: &'a mut W,
}
impl<'a> DX2TEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DX2TEV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DX2T signal has not been activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DX2TEV_A::VALUE1)
    }
    #[doc = "The DX2T signal has been activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DX2TEV_A::VALUE2)
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
#[doc = "Parity Error Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARERR_A {
    #[doc = "0: A parity error event has not been activated."]
    VALUE1 = 0,
    #[doc = "1: A parity error event has been activated."]
    VALUE2 = 1,
}
impl From<PARERR_A> for bool {
    #[inline(always)]
    fn from(variant: PARERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARERR` reader - Parity Error Event Detected"]
pub struct PARERR_R(crate::FieldReader<bool, PARERR_A>);
impl PARERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARERR_A {
        match self.bits {
            false => PARERR_A::VALUE1,
            true => PARERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PARERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PARERR_A::VALUE2
    }
}
impl core::ops::Deref for PARERR_R {
    type Target = crate::FieldReader<bool, PARERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARERR` writer - Parity Error Event Detected"]
pub struct PARERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PARERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A parity error event has not been activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PARERR_A::VALUE1)
    }
    #[doc = "A parity error event has been activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PARERR_A::VALUE2)
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
#[doc = "Receiver Start Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIF_A {
    #[doc = "0: A receiver start event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A receiver start event has occurred."]
    VALUE2 = 1,
}
impl From<RSIF_A> for bool {
    #[inline(always)]
    fn from(variant: RSIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSIF` reader - Receiver Start Indication Flag"]
pub struct RSIF_R(crate::FieldReader<bool, RSIF_A>);
impl RSIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSIF_A {
        match self.bits {
            false => RSIF_A::VALUE1,
            true => RSIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RSIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RSIF_A::VALUE2
    }
}
impl core::ops::Deref for RSIF_R {
    type Target = crate::FieldReader<bool, RSIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSIF` writer - Receiver Start Indication Flag"]
pub struct RSIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A receiver start event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSIF_A::VALUE1)
    }
    #[doc = "A receiver start event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSIF_A::VALUE2)
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
#[doc = "Data Lost Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLIF_A {
    #[doc = "0: A data lost event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A data lost event has occurred."]
    VALUE2 = 1,
}
impl From<DLIF_A> for bool {
    #[inline(always)]
    fn from(variant: DLIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLIF` reader - Data Lost Indication Flag"]
pub struct DLIF_R(crate::FieldReader<bool, DLIF_A>);
impl DLIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLIF_A {
        match self.bits {
            false => DLIF_A::VALUE1,
            true => DLIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DLIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DLIF_A::VALUE2
    }
}
impl core::ops::Deref for DLIF_R {
    type Target = crate::FieldReader<bool, DLIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLIF` writer - Data Lost Indication Flag"]
pub struct DLIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DLIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A data lost event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLIF_A::VALUE1)
    }
    #[doc = "A data lost event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLIF_A::VALUE2)
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
#[doc = "Transmit Shift Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIF_A {
    #[doc = "0: A transmit shift event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A transmit shift event has occurred."]
    VALUE2 = 1,
}
impl From<TSIF_A> for bool {
    #[inline(always)]
    fn from(variant: TSIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIF` reader - Transmit Shift Indication Flag"]
pub struct TSIF_R(crate::FieldReader<bool, TSIF_A>);
impl TSIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIF_A {
        match self.bits {
            false => TSIF_A::VALUE1,
            true => TSIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TSIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TSIF_A::VALUE2
    }
}
impl core::ops::Deref for TSIF_R {
    type Target = crate::FieldReader<bool, TSIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIF` writer - Transmit Shift Indication Flag"]
pub struct TSIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A transmit shift event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSIF_A::VALUE1)
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSIF_A::VALUE2)
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
#[doc = "Transmit Buffer Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBIF_A {
    #[doc = "0: A transmit buffer event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A transmit buffer event has occurred."]
    VALUE2 = 1,
}
impl From<TBIF_A> for bool {
    #[inline(always)]
    fn from(variant: TBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBIF` reader - Transmit Buffer Indication Flag"]
pub struct TBIF_R(crate::FieldReader<bool, TBIF_A>);
impl TBIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBIF_A {
        match self.bits {
            false => TBIF_A::VALUE1,
            true => TBIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TBIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TBIF_A::VALUE2
    }
}
impl core::ops::Deref for TBIF_R {
    type Target = crate::FieldReader<bool, TBIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBIF` writer - Transmit Buffer Indication Flag"]
pub struct TBIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A transmit buffer event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBIF_A::VALUE1)
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBIF_A::VALUE2)
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
#[doc = "Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIF_A {
    #[doc = "0: A receive event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A receive event has occurred."]
    VALUE2 = 1,
}
impl From<RIF_A> for bool {
    #[inline(always)]
    fn from(variant: RIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIF` reader - Receive Indication Flag"]
pub struct RIF_R(crate::FieldReader<bool, RIF_A>);
impl RIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIF_A {
        match self.bits {
            false => RIF_A::VALUE1,
            true => RIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RIF_A::VALUE2
    }
}
impl core::ops::Deref for RIF_R {
    type Target = crate::FieldReader<bool, RIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIF` writer - Receive Indication Flag"]
pub struct RIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RIF_A::VALUE1)
    }
    #[doc = "A receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RIF_A::VALUE2)
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
#[doc = "Alternative Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIF_A {
    #[doc = "0: An alternative receive event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: An alternative receive event has occurred."]
    VALUE2 = 1,
}
impl From<AIF_A> for bool {
    #[inline(always)]
    fn from(variant: AIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIF` reader - Alternative Receive Indication Flag"]
pub struct AIF_R(crate::FieldReader<bool, AIF_A>);
impl AIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIF_A {
        match self.bits {
            false => AIF_A::VALUE1,
            true => AIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == AIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AIF_A::VALUE2
    }
}
impl core::ops::Deref for AIF_R {
    type Target = crate::FieldReader<bool, AIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIF` writer - Alternative Receive Indication Flag"]
pub struct AIF_W<'a> {
    w: &'a mut W,
}
impl<'a> AIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An alternative receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AIF_A::VALUE1)
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AIF_A::VALUE2)
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
#[doc = "Baud Rate Generator Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRGIF_A {
    #[doc = "0: A baud rate generator event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A baud rate generator event has occurred."]
    VALUE2 = 1,
}
impl From<BRGIF_A> for bool {
    #[inline(always)]
    fn from(variant: BRGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRGIF` reader - Baud Rate Generator Indication Flag"]
pub struct BRGIF_R(crate::FieldReader<bool, BRGIF_A>);
impl BRGIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRGIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRGIF_A {
        match self.bits {
            false => BRGIF_A::VALUE1,
            true => BRGIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BRGIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BRGIF_A::VALUE2
    }
}
impl core::ops::Deref for BRGIF_R {
    type Target = crate::FieldReader<bool, BRGIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRGIF` writer - Baud Rate Generator Indication Flag"]
pub struct BRGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> BRGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRGIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A baud rate generator event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BRGIF_A::VALUE1)
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BRGIF_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - MSLS Status"]
    #[inline(always)]
    pub fn msls(&self) -> MSLS_R {
        MSLS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DX2S Status"]
    #[inline(always)]
    pub fn dx2s(&self) -> DX2S_R {
        DX2S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MSLS Event Detected"]
    #[inline(always)]
    pub fn mslsev(&self) -> MSLSEV_R {
        MSLSEV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DX2T Event Detected"]
    #[inline(always)]
    pub fn dx2tev(&self) -> DX2TEV_R {
        DX2TEV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity Error Event Detected"]
    #[inline(always)]
    pub fn parerr(&self) -> PARERR_R {
        PARERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    pub fn rsif(&self) -> RSIF_R {
        RSIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    pub fn dlif(&self) -> DLIF_R {
        DLIF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    pub fn tsif(&self) -> TSIF_R {
        TSIF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    pub fn tbif(&self) -> TBIF_R {
        TBIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    pub fn rif(&self) -> RIF_R {
        RIF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    pub fn aif(&self) -> AIF_R {
        AIF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    pub fn brgif(&self) -> BRGIF_R {
        BRGIF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSLS Status"]
    #[inline(always)]
    pub fn msls(&mut self) -> MSLS_W {
        MSLS_W { w: self }
    }
    #[doc = "Bit 1 - DX2S Status"]
    #[inline(always)]
    pub fn dx2s(&mut self) -> DX2S_W {
        DX2S_W { w: self }
    }
    #[doc = "Bit 2 - MSLS Event Detected"]
    #[inline(always)]
    pub fn mslsev(&mut self) -> MSLSEV_W {
        MSLSEV_W { w: self }
    }
    #[doc = "Bit 3 - DX2T Event Detected"]
    #[inline(always)]
    pub fn dx2tev(&mut self) -> DX2TEV_W {
        DX2TEV_W { w: self }
    }
    #[doc = "Bit 4 - Parity Error Event Detected"]
    #[inline(always)]
    pub fn parerr(&mut self) -> PARERR_W {
        PARERR_W { w: self }
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    pub fn rsif(&mut self) -> RSIF_W {
        RSIF_W { w: self }
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    pub fn dlif(&mut self) -> DLIF_W {
        DLIF_W { w: self }
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    pub fn tsif(&mut self) -> TSIF_W {
        TSIF_W { w: self }
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    pub fn tbif(&mut self) -> TBIF_W {
        TBIF_W { w: self }
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    pub fn rif(&mut self) -> RIF_W {
        RIF_W { w: self }
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    pub fn aif(&mut self) -> AIF_W {
        AIF_W { w: self }
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    pub fn brgif(&mut self) -> BRGIF_W {
        BRGIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protocol Status Register \\[SSC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr_sscmode](index.html) module"]
pub struct PSR_SSCMODE_SPEC;
impl crate::RegisterSpec for PSR_SSCMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr_sscmode::R](R) reader structure"]
impl crate::Readable for PSR_SSCMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psr_sscmode::W](W) writer structure"]
impl crate::Writable for PSR_SSCMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSR_SSCMode to value 0"]
impl crate::Resettable for PSR_SSCMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
