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
#[doc = "Field `MSLS` reader - MSLS Status"]
pub type MSLS_R = crate::BitReader<MSLS_A>;
#[doc = "MSLS Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSLS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MSLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSLS_A::VALUE2
    }
}
#[doc = "Field `MSLS` writer - MSLS Status"]
pub type MSLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, MSLS_A, O>;
impl<'a, const O: u8> MSLS_W<'a, O> {
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
}
#[doc = "Field `DX2S` reader - DX2S Status"]
pub type DX2S_R = crate::BitReader<DX2S_A>;
#[doc = "DX2S Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DX2S_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DX2S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DX2S_A::VALUE2
    }
}
#[doc = "Field `DX2S` writer - DX2S Status"]
pub type DX2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, DX2S_A, O>;
impl<'a, const O: u8> DX2S_W<'a, O> {
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
}
#[doc = "Field `MSLSEV` reader - MSLS Event Detected"]
pub type MSLSEV_R = crate::BitReader<MSLSEV_A>;
#[doc = "MSLS Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSLSEV_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MSLSEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSLSEV_A::VALUE2
    }
}
#[doc = "Field `MSLSEV` writer - MSLS Event Detected"]
pub type MSLSEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, MSLSEV_A, O>;
impl<'a, const O: u8> MSLSEV_W<'a, O> {
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
}
#[doc = "Field `DX2TEV` reader - DX2T Event Detected"]
pub type DX2TEV_R = crate::BitReader<DX2TEV_A>;
#[doc = "DX2T Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DX2TEV_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DX2TEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DX2TEV_A::VALUE2
    }
}
#[doc = "Field `DX2TEV` writer - DX2T Event Detected"]
pub type DX2TEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, DX2TEV_A, O>;
impl<'a, const O: u8> DX2TEV_W<'a, O> {
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
}
#[doc = "Field `PARERR` reader - Parity Error Event Detected"]
pub type PARERR_R = crate::BitReader<PARERR_A>;
#[doc = "Parity Error Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PARERR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PARERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PARERR_A::VALUE2
    }
}
#[doc = "Field `PARERR` writer - Parity Error Event Detected"]
pub type PARERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, PARERR_A, O>;
impl<'a, const O: u8> PARERR_W<'a, O> {
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
}
#[doc = "Field `RSIF` reader - Receiver Start Indication Flag"]
pub type RSIF_R = crate::BitReader<RSIF_A>;
#[doc = "Receiver Start Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RSIF_R {
    #[doc = "Get enumerated values variant"]
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
        *self == RSIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSIF_A::VALUE2
    }
}
#[doc = "Field `RSIF` writer - Receiver Start Indication Flag"]
pub type RSIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, RSIF_A, O>;
impl<'a, const O: u8> RSIF_W<'a, O> {
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
}
#[doc = "Field `DLIF` reader - Data Lost Indication Flag"]
pub type DLIF_R = crate::BitReader<DLIF_A>;
#[doc = "Data Lost Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DLIF_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DLIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DLIF_A::VALUE2
    }
}
#[doc = "Field `DLIF` writer - Data Lost Indication Flag"]
pub type DLIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, DLIF_A, O>;
impl<'a, const O: u8> DLIF_W<'a, O> {
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
}
#[doc = "Field `TSIF` reader - Transmit Shift Indication Flag"]
pub type TSIF_R = crate::BitReader<TSIF_A>;
#[doc = "Transmit Shift Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TSIF_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TSIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSIF_A::VALUE2
    }
}
#[doc = "Field `TSIF` writer - Transmit Shift Indication Flag"]
pub type TSIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, TSIF_A, O>;
impl<'a, const O: u8> TSIF_W<'a, O> {
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
}
#[doc = "Field `TBIF` reader - Transmit Buffer Indication Flag"]
pub type TBIF_R = crate::BitReader<TBIF_A>;
#[doc = "Transmit Buffer Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TBIF_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TBIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBIF_A::VALUE2
    }
}
#[doc = "Field `TBIF` writer - Transmit Buffer Indication Flag"]
pub type TBIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, TBIF_A, O>;
impl<'a, const O: u8> TBIF_W<'a, O> {
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
}
#[doc = "Field `RIF` reader - Receive Indication Flag"]
pub type RIF_R = crate::BitReader<RIF_A>;
#[doc = "Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RIF_R {
    #[doc = "Get enumerated values variant"]
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
        *self == RIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RIF_A::VALUE2
    }
}
#[doc = "Field `RIF` writer - Receive Indication Flag"]
pub type RIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, RIF_A, O>;
impl<'a, const O: u8> RIF_W<'a, O> {
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
}
#[doc = "Field `AIF` reader - Alternative Receive Indication Flag"]
pub type AIF_R = crate::BitReader<AIF_A>;
#[doc = "Alternative Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl AIF_R {
    #[doc = "Get enumerated values variant"]
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
        *self == AIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AIF_A::VALUE2
    }
}
#[doc = "Field `AIF` writer - Alternative Receive Indication Flag"]
pub type AIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, AIF_A, O>;
impl<'a, const O: u8> AIF_W<'a, O> {
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
}
#[doc = "Field `BRGIF` reader - Baud Rate Generator Indication Flag"]
pub type BRGIF_R = crate::BitReader<BRGIF_A>;
#[doc = "Baud Rate Generator Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl BRGIF_R {
    #[doc = "Get enumerated values variant"]
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
        *self == BRGIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BRGIF_A::VALUE2
    }
}
#[doc = "Field `BRGIF` writer - Baud Rate Generator Indication Flag"]
pub type BRGIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SSCMODE_SPEC, BRGIF_A, O>;
impl<'a, const O: u8> BRGIF_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - MSLS Status"]
    #[inline(always)]
    pub fn msls(&self) -> MSLS_R {
        MSLS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DX2S Status"]
    #[inline(always)]
    pub fn dx2s(&self) -> DX2S_R {
        DX2S_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSLS Event Detected"]
    #[inline(always)]
    pub fn mslsev(&self) -> MSLSEV_R {
        MSLSEV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DX2T Event Detected"]
    #[inline(always)]
    pub fn dx2tev(&self) -> DX2TEV_R {
        DX2TEV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity Error Event Detected"]
    #[inline(always)]
    pub fn parerr(&self) -> PARERR_R {
        PARERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    pub fn rsif(&self) -> RSIF_R {
        RSIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    pub fn dlif(&self) -> DLIF_R {
        DLIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    pub fn tsif(&self) -> TSIF_R {
        TSIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    pub fn tbif(&self) -> TBIF_R {
        TBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    pub fn rif(&self) -> RIF_R {
        RIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    pub fn aif(&self) -> AIF_R {
        AIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    pub fn brgif(&self) -> BRGIF_R {
        BRGIF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSLS Status"]
    #[inline(always)]
    #[must_use]
    pub fn msls(&mut self) -> MSLS_W<0> {
        MSLS_W::new(self)
    }
    #[doc = "Bit 1 - DX2S Status"]
    #[inline(always)]
    #[must_use]
    pub fn dx2s(&mut self) -> DX2S_W<1> {
        DX2S_W::new(self)
    }
    #[doc = "Bit 2 - MSLS Event Detected"]
    #[inline(always)]
    #[must_use]
    pub fn mslsev(&mut self) -> MSLSEV_W<2> {
        MSLSEV_W::new(self)
    }
    #[doc = "Bit 3 - DX2T Event Detected"]
    #[inline(always)]
    #[must_use]
    pub fn dx2tev(&mut self) -> DX2TEV_W<3> {
        DX2TEV_W::new(self)
    }
    #[doc = "Bit 4 - Parity Error Event Detected"]
    #[inline(always)]
    #[must_use]
    pub fn parerr(&mut self) -> PARERR_W<4> {
        PARERR_W::new(self)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsif(&mut self) -> RSIF_W<10> {
        RSIF_W::new(self)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlif(&mut self) -> DLIF_W<11> {
        DLIF_W::new(self)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsif(&mut self) -> TSIF_W<12> {
        TSIF_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tbif(&mut self) -> TBIF_W<13> {
        TBIF_W::new(self)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rif(&mut self) -> RIF_W<14> {
        RIF_W::new(self)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aif(&mut self) -> AIF_W<15> {
        AIF_W::new(self)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn brgif(&mut self) -> BRGIF_W<16> {
        BRGIF_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSR_SSCMode to value 0"]
impl crate::Resettable for PSR_SSCMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
