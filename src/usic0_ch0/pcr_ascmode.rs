#[doc = "Register `PCR_ASCMode` reader"]
pub struct R(crate::R<PCR_ASCMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_ASCMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_ASCMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_ASCMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR_ASCMode` writer"]
pub struct W(crate::W<PCR_ASCMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_ASCMODE_SPEC>;
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
impl From<crate::W<PCR_ASCMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_ASCMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sample Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMD_A {
    #[doc = "0: Only one sample is taken per bit time. The current input value is sampled."]
    VALUE1 = 0,
    #[doc = "1: Three samples are taken per bit time and a majority decision is made."]
    VALUE2 = 1,
}
impl From<SMD_A> for bool {
    #[inline(always)]
    fn from(variant: SMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMD` reader - Sample Mode"]
pub struct SMD_R(crate::FieldReader<bool, SMD_A>);
impl SMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMD_A {
        match self.bits {
            false => SMD_A::VALUE1,
            true => SMD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SMD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMD_A::VALUE2
    }
}
impl core::ops::Deref for SMD_R {
    type Target = crate::FieldReader<bool, SMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMD` writer - Sample Mode"]
pub struct SMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only one sample is taken per bit time. The current input value is sampled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMD_A::VALUE1)
    }
    #[doc = "Three samples are taken per bit time and a majority decision is made."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMD_A::VALUE2)
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
#[doc = "Stop Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPB_A {
    #[doc = "0: The number of stop bits is 1."]
    VALUE1 = 0,
    #[doc = "1: The number of stop bits is 2."]
    VALUE2 = 1,
}
impl From<STPB_A> for bool {
    #[inline(always)]
    fn from(variant: STPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPB` reader - Stop Bits"]
pub struct STPB_R(crate::FieldReader<bool, STPB_A>);
impl STPB_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPB_A {
        match self.bits {
            false => STPB_A::VALUE1,
            true => STPB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STPB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STPB_A::VALUE2
    }
}
impl core::ops::Deref for STPB_R {
    type Target = crate::FieldReader<bool, STPB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPB` writer - Stop Bits"]
pub struct STPB_W<'a> {
    w: &'a mut W,
}
impl<'a> STPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The number of stop bits is 1."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STPB_A::VALUE1)
    }
    #[doc = "The number of stop bits is 2."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STPB_A::VALUE2)
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
#[doc = "Idle Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDM_A {
    #[doc = "0: The bus idle detection is switched off and bits PSR.TXIDLE and PSR.RXIDLE are set automatically to enable data transfers without checking the inputs before."]
    VALUE1 = 0,
    #[doc = "1: The bus is considered as idle after a number of consecutive passive bit times defined by SCTR.FLE plus 2 (in the case without parity bit) or plus 3 (in the case with parity bit)."]
    VALUE2 = 1,
}
impl From<IDM_A> for bool {
    #[inline(always)]
    fn from(variant: IDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDM` reader - Idle Detection Mode"]
pub struct IDM_R(crate::FieldReader<bool, IDM_A>);
impl IDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDM_A {
        match self.bits {
            false => IDM_A::VALUE1,
            true => IDM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == IDM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == IDM_A::VALUE2
    }
}
impl core::ops::Deref for IDM_R {
    type Target = crate::FieldReader<bool, IDM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDM` writer - Idle Detection Mode"]
pub struct IDM_W<'a> {
    w: &'a mut W,
}
impl<'a> IDM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The bus idle detection is switched off and bits PSR.TXIDLE and PSR.RXIDLE are set automatically to enable data transfers without checking the inputs before."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDM_A::VALUE1)
    }
    #[doc = "The bus is considered as idle after a number of consecutive passive bit times defined by SCTR.FLE plus 2 (in the case without parity bit) or plus 3 (in the case with parity bit)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDM_A::VALUE2)
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
#[doc = "Synchronization Break Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBIEN_A {
    #[doc = "0: The interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<SBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SBIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBIEN` reader - Synchronization Break Interrupt Enable"]
pub struct SBIEN_R(crate::FieldReader<bool, SBIEN_A>);
impl SBIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBIEN_A {
        match self.bits {
            false => SBIEN_A::VALUE1,
            true => SBIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SBIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SBIEN_A::VALUE2
    }
}
impl core::ops::Deref for SBIEN_R {
    type Target = crate::FieldReader<bool, SBIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBIEN` writer - Synchronization Break Interrupt Enable"]
pub struct SBIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SBIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SBIEN_A::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SBIEN_A::VALUE2)
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
#[doc = "Collision Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDEN_A {
    #[doc = "0: The collision detection is disabled."]
    VALUE1 = 0,
    #[doc = "1: If a collision is detected, the transmitter stops its data transmission, outputs a 1, sets bit PSR.COL and generates a protocol interrupt. In order to allow data transmission again, PSR.COL has to be cleared by software."]
    VALUE2 = 1,
}
impl From<CDEN_A> for bool {
    #[inline(always)]
    fn from(variant: CDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDEN` reader - Collision Detection Enable"]
pub struct CDEN_R(crate::FieldReader<bool, CDEN_A>);
impl CDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDEN_A {
        match self.bits {
            false => CDEN_A::VALUE1,
            true => CDEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CDEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CDEN_A::VALUE2
    }
}
impl core::ops::Deref for CDEN_R {
    type Target = crate::FieldReader<bool, CDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDEN` writer - Collision Detection Enable"]
pub struct CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The collision detection is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDEN_A::VALUE1)
    }
    #[doc = "If a collision is detected, the transmitter stops its data transmission, outputs a 1, sets bit PSR.COL and generates a protocol interrupt. In order to allow data transmission again, PSR.COL has to be cleared by software."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDEN_A::VALUE2)
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
#[doc = "Receiver Noise Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNIEN_A {
    #[doc = "0: The interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<RNIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNIEN` reader - Receiver Noise Detection Interrupt Enable"]
pub struct RNIEN_R(crate::FieldReader<bool, RNIEN_A>);
impl RNIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNIEN_A {
        match self.bits {
            false => RNIEN_A::VALUE1,
            true => RNIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RNIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RNIEN_A::VALUE2
    }
}
impl core::ops::Deref for RNIEN_R {
    type Target = crate::FieldReader<bool, RNIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNIEN` writer - Receiver Noise Detection Interrupt Enable"]
pub struct RNIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RNIEN_A::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RNIEN_A::VALUE2)
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
#[doc = "Format Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIEN_A {
    #[doc = "0: The interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<FEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: FEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIEN` reader - Format Error Interrupt Enable"]
pub struct FEIEN_R(crate::FieldReader<bool, FEIEN_A>);
impl FEIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIEN_A {
        match self.bits {
            false => FEIEN_A::VALUE1,
            true => FEIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FEIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FEIEN_A::VALUE2
    }
}
impl core::ops::Deref for FEIEN_R {
    type Target = crate::FieldReader<bool, FEIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIEN` writer - Format Error Interrupt Enable"]
pub struct FEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FEIEN_A::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FEIEN_A::VALUE2)
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
#[doc = "Frame Finished Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFIEN_A {
    #[doc = "0: The interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<FFIEN_A> for bool {
    #[inline(always)]
    fn from(variant: FFIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFIEN` reader - Frame Finished Interrupt Enable"]
pub struct FFIEN_R(crate::FieldReader<bool, FFIEN_A>);
impl FFIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFIEN_A {
        match self.bits {
            false => FFIEN_A::VALUE1,
            true => FFIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FFIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FFIEN_A::VALUE2
    }
}
impl core::ops::Deref for FFIEN_R {
    type Target = crate::FieldReader<bool, FFIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFIEN` writer - Frame Finished Interrupt Enable"]
pub struct FFIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FFIEN_A::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FFIEN_A::VALUE2)
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
#[doc = "Field `SP` reader - Sample Point"]
pub struct SP_R(crate::FieldReader<u8, u8>);
impl SP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP` writer - Sample Point"]
pub struct SP_W<'a> {
    w: &'a mut W,
}
impl<'a> SP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Pulse Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PL_A {
    #[doc = "0: The pulse length is equal to the bit length (no shortened 0)."]
    VALUE1 = 0,
    #[doc = "1: The pulse length of a 0 bit is 2 time quanta."]
    VALUE2 = 1,
    #[doc = "2: The pulse length of a 0 bit is 3 time quanta."]
    VALUE3 = 2,
    #[doc = "7: The pulse length of a 0 bit is 8 time quanta."]
    VALUE4 = 7,
}
impl From<PL_A> for u8 {
    #[inline(always)]
    fn from(variant: PL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PL` reader - Pulse Length"]
pub struct PL_R(crate::FieldReader<u8, PL_A>);
impl PL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PL_A> {
        match self.bits {
            0 => Some(PL_A::VALUE1),
            1 => Some(PL_A::VALUE2),
            2 => Some(PL_A::VALUE3),
            7 => Some(PL_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PL_A::VALUE4
    }
}
impl core::ops::Deref for PL_R {
    type Target = crate::FieldReader<u8, PL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PL` writer - Pulse Length"]
pub struct PL_W<'a> {
    w: &'a mut W,
}
impl<'a> PL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The pulse length is equal to the bit length (no shortened 0)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PL_A::VALUE1)
    }
    #[doc = "The pulse length of a 0 bit is 2 time quanta."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PL_A::VALUE2)
    }
    #[doc = "The pulse length of a 0 bit is 3 time quanta."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PL_A::VALUE3)
    }
    #[doc = "The pulse length of a 0 bit is 8 time quanta."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Receiver Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTEN_A {
    #[doc = "0: Flag PSR\\[9\\]
is not modified depending on the receiver status."]
    VALUE1 = 0,
    #[doc = "1: Flag PSR\\[9\\]
is set during the complete reception of a frame."]
    VALUE2 = 1,
}
impl From<RSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTEN` reader - Receiver Status Enable"]
pub struct RSTEN_R(crate::FieldReader<bool, RSTEN_A>);
impl RSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTEN_A {
        match self.bits {
            false => RSTEN_A::VALUE1,
            true => RSTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RSTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RSTEN_A::VALUE2
    }
}
impl core::ops::Deref for RSTEN_R {
    type Target = crate::FieldReader<bool, RSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTEN` writer - Receiver Status Enable"]
pub struct RSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flag PSR\\[9\\]
is not modified depending on the receiver status."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSTEN_A::VALUE1)
    }
    #[doc = "Flag PSR\\[9\\]
is set during the complete reception of a frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSTEN_A::VALUE2)
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
#[doc = "Transmitter Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTEN_A {
    #[doc = "0: Flag PSR\\[9\\]
is not modified depending on the transmitter status."]
    VALUE1 = 0,
    #[doc = "1: Flag PSR\\[9\\]
is set during the complete transmission of a frame."]
    VALUE2 = 1,
}
impl From<TSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTEN` reader - Transmitter Status Enable"]
pub struct TSTEN_R(crate::FieldReader<bool, TSTEN_A>);
impl TSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTEN_A {
        match self.bits {
            false => TSTEN_A::VALUE1,
            true => TSTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TSTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TSTEN_A::VALUE2
    }
}
impl core::ops::Deref for TSTEN_R {
    type Target = crate::FieldReader<bool, TSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTEN` writer - Transmitter Status Enable"]
pub struct TSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flag PSR\\[9\\]
is not modified depending on the transmitter status."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSTEN_A::VALUE1)
    }
    #[doc = "Flag PSR\\[9\\]
is set during the complete transmission of a frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSTEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLK_A {
    #[doc = "0: The MCLK generation is disabled and the MCLK signal is 0."]
    VALUE1 = 0,
    #[doc = "1: The MCLK generation is enabled."]
    VALUE2 = 1,
}
impl From<MCLK_A> for bool {
    #[inline(always)]
    fn from(variant: MCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLK` reader - Master Clock Enable"]
pub struct MCLK_R(crate::FieldReader<bool, MCLK_A>);
impl MCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLK_A {
        match self.bits {
            false => MCLK_A::VALUE1,
            true => MCLK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MCLK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MCLK_A::VALUE2
    }
}
impl core::ops::Deref for MCLK_R {
    type Target = crate::FieldReader<bool, MCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLK` writer - Master Clock Enable"]
pub struct MCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The MCLK generation is disabled and the MCLK signal is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCLK_A::VALUE1)
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCLK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sample Mode"]
    #[inline(always)]
    pub fn smd(&self) -> SMD_R {
        SMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Stop Bits"]
    #[inline(always)]
    pub fn stpb(&self) -> STPB_R {
        STPB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Idle Detection Mode"]
    #[inline(always)]
    pub fn idm(&self) -> IDM_R {
        IDM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Synchronization Break Interrupt Enable"]
    #[inline(always)]
    pub fn sbien(&self) -> SBIEN_R {
        SBIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Collision Detection Enable"]
    #[inline(always)]
    pub fn cden(&self) -> CDEN_R {
        CDEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receiver Noise Detection Interrupt Enable"]
    #[inline(always)]
    pub fn rnien(&self) -> RNIEN_R {
        RNIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Format Error Interrupt Enable"]
    #[inline(always)]
    pub fn feien(&self) -> FEIEN_R {
        FEIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Frame Finished Interrupt Enable"]
    #[inline(always)]
    pub fn ffien(&self) -> FFIEN_R {
        FFIEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Sample Point"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Pulse Length"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Receiver Status Enable"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmitter Status Enable"]
    #[inline(always)]
    pub fn tsten(&self) -> TSTEN_R {
        TSTEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MCLK_R {
        MCLK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample Mode"]
    #[inline(always)]
    pub fn smd(&mut self) -> SMD_W {
        SMD_W { w: self }
    }
    #[doc = "Bit 1 - Stop Bits"]
    #[inline(always)]
    pub fn stpb(&mut self) -> STPB_W {
        STPB_W { w: self }
    }
    #[doc = "Bit 2 - Idle Detection Mode"]
    #[inline(always)]
    pub fn idm(&mut self) -> IDM_W {
        IDM_W { w: self }
    }
    #[doc = "Bit 3 - Synchronization Break Interrupt Enable"]
    #[inline(always)]
    pub fn sbien(&mut self) -> SBIEN_W {
        SBIEN_W { w: self }
    }
    #[doc = "Bit 4 - Collision Detection Enable"]
    #[inline(always)]
    pub fn cden(&mut self) -> CDEN_W {
        CDEN_W { w: self }
    }
    #[doc = "Bit 5 - Receiver Noise Detection Interrupt Enable"]
    #[inline(always)]
    pub fn rnien(&mut self) -> RNIEN_W {
        RNIEN_W { w: self }
    }
    #[doc = "Bit 6 - Format Error Interrupt Enable"]
    #[inline(always)]
    pub fn feien(&mut self) -> FEIEN_W {
        FEIEN_W { w: self }
    }
    #[doc = "Bit 7 - Frame Finished Interrupt Enable"]
    #[inline(always)]
    pub fn ffien(&mut self) -> FFIEN_W {
        FFIEN_W { w: self }
    }
    #[doc = "Bits 8:12 - Sample Point"]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W {
        SP_W { w: self }
    }
    #[doc = "Bits 13:15 - Pulse Length"]
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W {
        PL_W { w: self }
    }
    #[doc = "Bit 16 - Receiver Status Enable"]
    #[inline(always)]
    pub fn rsten(&mut self) -> RSTEN_W {
        RSTEN_W { w: self }
    }
    #[doc = "Bit 17 - Transmitter Status Enable"]
    #[inline(always)]
    pub fn tsten(&mut self) -> TSTEN_W {
        TSTEN_W { w: self }
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&mut self) -> MCLK_W {
        MCLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protocol Control Register \\[ASC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr_ascmode](index.html) module"]
pub struct PCR_ASCMODE_SPEC;
impl crate::RegisterSpec for PCR_ASCMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr_ascmode::R](R) reader structure"]
impl crate::Readable for PCR_ASCMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr_ascmode::W](W) writer structure"]
impl crate::Writable for PCR_ASCMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCR_ASCMode to value 0"]
impl crate::Resettable for PCR_ASCMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
