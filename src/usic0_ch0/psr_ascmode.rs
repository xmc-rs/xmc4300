#[doc = "Register `PSR_ASCMode` reader"]
pub struct R(crate::R<PSR_ASCMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_ASCMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_ASCMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_ASCMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSR_ASCMode` writer"]
pub struct W(crate::W<PSR_ASCMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_ASCMODE_SPEC>;
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
impl From<crate::W<PSR_ASCMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_ASCMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmission Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIDLE_A {
    #[doc = "0: The transmitter line has not yet been idle."]
    VALUE1 = 0,
    #[doc = "1: The transmitter line has been idle and frame transmission is possible."]
    VALUE2 = 1,
}
impl From<TXIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIDLE` reader - Transmission Idle"]
pub struct TXIDLE_R(crate::FieldReader<bool, TXIDLE_A>);
impl TXIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXIDLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIDLE_A {
        match self.bits {
            false => TXIDLE_A::VALUE1,
            true => TXIDLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TXIDLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TXIDLE_A::VALUE2
    }
}
impl core::ops::Deref for TXIDLE_R {
    type Target = crate::FieldReader<bool, TXIDLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIDLE` writer - Transmission Idle"]
pub struct TXIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIDLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The transmitter line has not yet been idle."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXIDLE_A::VALUE1)
    }
    #[doc = "The transmitter line has been idle and frame transmission is possible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXIDLE_A::VALUE2)
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
#[doc = "Reception Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIDLE_A {
    #[doc = "0: The receiver line has not yet been idle."]
    VALUE1 = 0,
    #[doc = "1: The receiver line has been idle and frame reception is possible."]
    VALUE2 = 1,
}
impl From<RXIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIDLE` reader - Reception Idle"]
pub struct RXIDLE_R(crate::FieldReader<bool, RXIDLE_A>);
impl RXIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIDLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIDLE_A {
        match self.bits {
            false => RXIDLE_A::VALUE1,
            true => RXIDLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RXIDLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RXIDLE_A::VALUE2
    }
}
impl core::ops::Deref for RXIDLE_R {
    type Target = crate::FieldReader<bool, RXIDLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIDLE` writer - Reception Idle"]
pub struct RXIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIDLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The receiver line has not yet been idle."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXIDLE_A::VALUE1)
    }
    #[doc = "The receiver line has been idle and frame reception is possible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXIDLE_A::VALUE2)
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
#[doc = "Synchronization Break Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBD_A {
    #[doc = "0: A synchronization break has not yet been detected."]
    VALUE1 = 0,
    #[doc = "1: A synchronization break has been detected."]
    VALUE2 = 1,
}
impl From<SBD_A> for bool {
    #[inline(always)]
    fn from(variant: SBD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBD` reader - Synchronization Break Detected"]
pub struct SBD_R(crate::FieldReader<bool, SBD_A>);
impl SBD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBD_A {
        match self.bits {
            false => SBD_A::VALUE1,
            true => SBD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SBD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SBD_A::VALUE2
    }
}
impl core::ops::Deref for SBD_R {
    type Target = crate::FieldReader<bool, SBD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBD` writer - Synchronization Break Detected"]
pub struct SBD_W<'a> {
    w: &'a mut W,
}
impl<'a> SBD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A synchronization break has not yet been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SBD_A::VALUE1)
    }
    #[doc = "A synchronization break has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SBD_A::VALUE2)
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
#[doc = "Collision Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COL_A {
    #[doc = "0: A collision has not yet been detected and frame transmission is possible."]
    VALUE1 = 0,
    #[doc = "1: A collision has been detected and frame transmission is not possible."]
    VALUE2 = 1,
}
impl From<COL_A> for bool {
    #[inline(always)]
    fn from(variant: COL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COL` reader - Collision Detected"]
pub struct COL_R(crate::FieldReader<bool, COL_A>);
impl COL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COL_A {
        match self.bits {
            false => COL_A::VALUE1,
            true => COL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == COL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == COL_A::VALUE2
    }
}
impl core::ops::Deref for COL_R {
    type Target = crate::FieldReader<bool, COL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COL` writer - Collision Detected"]
pub struct COL_W<'a> {
    w: &'a mut W,
}
impl<'a> COL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A collision has not yet been detected and frame transmission is possible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(COL_A::VALUE1)
    }
    #[doc = "A collision has been detected and frame transmission is not possible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(COL_A::VALUE2)
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
#[doc = "Receiver Noise Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNS_A {
    #[doc = "0: Receiver noise has not been detected."]
    VALUE1 = 0,
    #[doc = "1: Receiver noise has been detected."]
    VALUE2 = 1,
}
impl From<RNS_A> for bool {
    #[inline(always)]
    fn from(variant: RNS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNS` reader - Receiver Noise Detected"]
pub struct RNS_R(crate::FieldReader<bool, RNS_A>);
impl RNS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNS_A {
        match self.bits {
            false => RNS_A::VALUE1,
            true => RNS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RNS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RNS_A::VALUE2
    }
}
impl core::ops::Deref for RNS_R {
    type Target = crate::FieldReader<bool, RNS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNS` writer - Receiver Noise Detected"]
pub struct RNS_W<'a> {
    w: &'a mut W,
}
impl<'a> RNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receiver noise has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RNS_A::VALUE1)
    }
    #[doc = "Receiver noise has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RNS_A::VALUE2)
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
#[doc = "Format Error in Stop Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER0_A {
    #[doc = "0: A format error 0 has not been detected."]
    VALUE1 = 0,
    #[doc = "1: A format error 0 has been detected."]
    VALUE2 = 1,
}
impl From<FER0_A> for bool {
    #[inline(always)]
    fn from(variant: FER0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FER0` reader - Format Error in Stop Bit 0"]
pub struct FER0_R(crate::FieldReader<bool, FER0_A>);
impl FER0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FER0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FER0_A {
        match self.bits {
            false => FER0_A::VALUE1,
            true => FER0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FER0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FER0_A::VALUE2
    }
}
impl core::ops::Deref for FER0_R {
    type Target = crate::FieldReader<bool, FER0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FER0` writer - Format Error in Stop Bit 0"]
pub struct FER0_W<'a> {
    w: &'a mut W,
}
impl<'a> FER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FER0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A format error 0 has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FER0_A::VALUE1)
    }
    #[doc = "A format error 0 has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FER0_A::VALUE2)
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
#[doc = "Format Error in Stop Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER1_A {
    #[doc = "0: A format error 1 has not been detected."]
    VALUE1 = 0,
    #[doc = "1: A format error 1 has been detected."]
    VALUE2 = 1,
}
impl From<FER1_A> for bool {
    #[inline(always)]
    fn from(variant: FER1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FER1` reader - Format Error in Stop Bit 1"]
pub struct FER1_R(crate::FieldReader<bool, FER1_A>);
impl FER1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FER1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FER1_A {
        match self.bits {
            false => FER1_A::VALUE1,
            true => FER1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FER1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FER1_A::VALUE2
    }
}
impl core::ops::Deref for FER1_R {
    type Target = crate::FieldReader<bool, FER1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FER1` writer - Format Error in Stop Bit 1"]
pub struct FER1_W<'a> {
    w: &'a mut W,
}
impl<'a> FER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FER1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A format error 1 has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FER1_A::VALUE1)
    }
    #[doc = "A format error 1 has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FER1_A::VALUE2)
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
#[doc = "Receive Frame Finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFF_A {
    #[doc = "0: The received frame is not yet finished."]
    VALUE1 = 0,
    #[doc = "1: The received frame is finished."]
    VALUE2 = 1,
}
impl From<RFF_A> for bool {
    #[inline(always)]
    fn from(variant: RFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF` reader - Receive Frame Finished"]
pub struct RFF_R(crate::FieldReader<bool, RFF_A>);
impl RFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFF_A {
        match self.bits {
            false => RFF_A::VALUE1,
            true => RFF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RFF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RFF_A::VALUE2
    }
}
impl core::ops::Deref for RFF_R {
    type Target = crate::FieldReader<bool, RFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFF` writer - Receive Frame Finished"]
pub struct RFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The received frame is not yet finished."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RFF_A::VALUE1)
    }
    #[doc = "The received frame is finished."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RFF_A::VALUE2)
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
#[doc = "Transmitter Frame Finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFF_A {
    #[doc = "0: The transmitter frame is not yet finished."]
    VALUE1 = 0,
    #[doc = "1: The transmitter frame is finished."]
    VALUE2 = 1,
}
impl From<TFF_A> for bool {
    #[inline(always)]
    fn from(variant: TFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFF` reader - Transmitter Frame Finished"]
pub struct TFF_R(crate::FieldReader<bool, TFF_A>);
impl TFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFF_A {
        match self.bits {
            false => TFF_A::VALUE1,
            true => TFF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TFF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TFF_A::VALUE2
    }
}
impl core::ops::Deref for TFF_R {
    type Target = crate::FieldReader<bool, TFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFF` writer - Transmitter Frame Finished"]
pub struct TFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The transmitter frame is not yet finished."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TFF_A::VALUE1)
    }
    #[doc = "The transmitter frame is finished."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TFF_A::VALUE2)
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
#[doc = "Transfer Status BUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: A data transfer does not take place."]
    VALUE1 = 0,
    #[doc = "1: A data transfer currently takes place."]
    VALUE2 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Transfer Status BUSY"]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUSY_A::VALUE2
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[doc = "Bit 0 - Transmission Idle"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reception Idle"]
    #[inline(always)]
    pub fn rxidle(&self) -> RXIDLE_R {
        RXIDLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Synchronization Break Detected"]
    #[inline(always)]
    pub fn sbd(&self) -> SBD_R {
        SBD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Collision Detected"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receiver Noise Detected"]
    #[inline(always)]
    pub fn rns(&self) -> RNS_R {
        RNS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Format Error in Stop Bit 0"]
    #[inline(always)]
    pub fn fer0(&self) -> FER0_R {
        FER0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Format Error in Stop Bit 1"]
    #[inline(always)]
    pub fn fer1(&self) -> FER1_R {
        FER1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Frame Finished"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmitter Frame Finished"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transfer Status BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 9) & 0x01) != 0)
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
    #[doc = "Bit 0 - Transmission Idle"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TXIDLE_W {
        TXIDLE_W { w: self }
    }
    #[doc = "Bit 1 - Reception Idle"]
    #[inline(always)]
    pub fn rxidle(&mut self) -> RXIDLE_W {
        RXIDLE_W { w: self }
    }
    #[doc = "Bit 2 - Synchronization Break Detected"]
    #[inline(always)]
    pub fn sbd(&mut self) -> SBD_W {
        SBD_W { w: self }
    }
    #[doc = "Bit 3 - Collision Detected"]
    #[inline(always)]
    pub fn col(&mut self) -> COL_W {
        COL_W { w: self }
    }
    #[doc = "Bit 4 - Receiver Noise Detected"]
    #[inline(always)]
    pub fn rns(&mut self) -> RNS_W {
        RNS_W { w: self }
    }
    #[doc = "Bit 5 - Format Error in Stop Bit 0"]
    #[inline(always)]
    pub fn fer0(&mut self) -> FER0_W {
        FER0_W { w: self }
    }
    #[doc = "Bit 6 - Format Error in Stop Bit 1"]
    #[inline(always)]
    pub fn fer1(&mut self) -> FER1_W {
        FER1_W { w: self }
    }
    #[doc = "Bit 7 - Receive Frame Finished"]
    #[inline(always)]
    pub fn rff(&mut self) -> RFF_W {
        RFF_W { w: self }
    }
    #[doc = "Bit 8 - Transmitter Frame Finished"]
    #[inline(always)]
    pub fn tff(&mut self) -> TFF_W {
        TFF_W { w: self }
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
#[doc = "Protocol Status Register \\[ASC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr_ascmode](index.html) module"]
pub struct PSR_ASCMODE_SPEC;
impl crate::RegisterSpec for PSR_ASCMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr_ascmode::R](R) reader structure"]
impl crate::Readable for PSR_ASCMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psr_ascmode::W](W) writer structure"]
impl crate::Writable for PSR_ASCMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSR_ASCMode to value 0"]
impl crate::Resettable for PSR_ASCMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
