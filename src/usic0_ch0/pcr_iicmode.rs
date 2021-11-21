#[doc = "Register `PCR_IICMode` reader"]
pub struct R(crate::R<PCR_IICMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_IICMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_IICMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_IICMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR_IICMode` writer"]
pub struct W(crate::W<PCR_IICMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_IICMODE_SPEC>;
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
impl From<crate::W<PCR_IICMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_IICMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAD` reader - Slave Address"]
pub struct SLAD_R(crate::FieldReader<u16, u16>);
impl SLAD_R {
    pub(crate) fn new(bits: u16) -> Self {
        SLAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAD` writer - Slave Address"]
pub struct SLAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Acknowledge 00H\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK00_A {
    #[doc = "0: The slave device is not sensitive to this address."]
    VALUE1 = 0,
    #[doc = "1: The slave device is sensitive to this address."]
    VALUE2 = 1,
}
impl From<ACK00_A> for bool {
    #[inline(always)]
    fn from(variant: ACK00_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK00` reader - Acknowledge 00H"]
pub struct ACK00_R(crate::FieldReader<bool, ACK00_A>);
impl ACK00_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACK00_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK00_A {
        match self.bits {
            false => ACK00_A::VALUE1,
            true => ACK00_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ACK00_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACK00_A::VALUE2
    }
}
impl core::ops::Deref for ACK00_R {
    type Target = crate::FieldReader<bool, ACK00_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK00` writer - Acknowledge 00H"]
pub struct ACK00_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK00_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACK00_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The slave device is not sensitive to this address."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACK00_A::VALUE1)
    }
    #[doc = "The slave device is sensitive to this address."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACK00_A::VALUE2)
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
#[doc = "Symbol Timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STIM_A {
    #[doc = "0: A symbol contains 10 time quanta. The timing is adapted for standard mode (100 kBaud)."]
    VALUE1 = 0,
    #[doc = "1: A symbol contains 25 time quanta. The timing is adapted for fast mode (400 kBaud)."]
    VALUE2 = 1,
}
impl From<STIM_A> for bool {
    #[inline(always)]
    fn from(variant: STIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STIM` reader - Symbol Timing"]
pub struct STIM_R(crate::FieldReader<bool, STIM_A>);
impl STIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STIM_A {
        match self.bits {
            false => STIM_A::VALUE1,
            true => STIM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STIM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STIM_A::VALUE2
    }
}
impl core::ops::Deref for STIM_R {
    type Target = crate::FieldReader<bool, STIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIM` writer - Symbol Timing"]
pub struct STIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A symbol contains 10 time quanta. The timing is adapted for standard mode (100 kBaud)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STIM_A::VALUE1)
    }
    #[doc = "A symbol contains 25 time quanta. The timing is adapted for fast mode (400 kBaud)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STIM_A::VALUE2)
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
#[doc = "Start Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCRIEN_A {
    #[doc = "0: The start condition interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The start condition interrupt is enabled."]
    VALUE2 = 1,
}
impl From<SCRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCRIEN` reader - Start Condition Received Interrupt Enable"]
pub struct SCRIEN_R(crate::FieldReader<bool, SCRIEN_A>);
impl SCRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCRIEN_A {
        match self.bits {
            false => SCRIEN_A::VALUE1,
            true => SCRIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SCRIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SCRIEN_A::VALUE2
    }
}
impl core::ops::Deref for SCRIEN_R {
    type Target = crate::FieldReader<bool, SCRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRIEN` writer - Start Condition Received Interrupt Enable"]
pub struct SCRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The start condition interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCRIEN_A::VALUE1)
    }
    #[doc = "The start condition interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCRIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Repeated Start Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSCRIEN_A {
    #[doc = "0: The repeated start condition interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The repeated start condition interrupt is enabled."]
    VALUE2 = 1,
}
impl From<RSCRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RSCRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSCRIEN` reader - Repeated Start Condition Received Interrupt Enable"]
pub struct RSCRIEN_R(crate::FieldReader<bool, RSCRIEN_A>);
impl RSCRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSCRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSCRIEN_A {
        match self.bits {
            false => RSCRIEN_A::VALUE1,
            true => RSCRIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RSCRIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RSCRIEN_A::VALUE2
    }
}
impl core::ops::Deref for RSCRIEN_R {
    type Target = crate::FieldReader<bool, RSCRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSCRIEN` writer - Repeated Start Condition Received Interrupt Enable"]
pub struct RSCRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSCRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSCRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The repeated start condition interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSCRIEN_A::VALUE1)
    }
    #[doc = "The repeated start condition interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSCRIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Stop Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCRIEN_A {
    #[doc = "0: The stop condition interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The stop condition interrupt is enabled."]
    VALUE2 = 1,
}
impl From<PCRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: PCRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCRIEN` reader - Stop Condition Received Interrupt Enable"]
pub struct PCRIEN_R(crate::FieldReader<bool, PCRIEN_A>);
impl PCRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCRIEN_A {
        match self.bits {
            false => PCRIEN_A::VALUE1,
            true => PCRIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PCRIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PCRIEN_A::VALUE2
    }
}
impl core::ops::Deref for PCRIEN_R {
    type Target = crate::FieldReader<bool, PCRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCRIEN` writer - Stop Condition Received Interrupt Enable"]
pub struct PCRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The stop condition interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCRIEN_A::VALUE1)
    }
    #[doc = "The stop condition interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCRIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Non-Acknowledge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKIEN_A {
    #[doc = "0: The non-acknowledge interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The non-acknowledge interrupt is enabled."]
    VALUE2 = 1,
}
impl From<NACKIEN_A> for bool {
    #[inline(always)]
    fn from(variant: NACKIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKIEN` reader - Non-Acknowledge Interrupt Enable"]
pub struct NACKIEN_R(crate::FieldReader<bool, NACKIEN_A>);
impl NACKIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        NACKIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKIEN_A {
        match self.bits {
            false => NACKIEN_A::VALUE1,
            true => NACKIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NACKIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NACKIEN_A::VALUE2
    }
}
impl core::ops::Deref for NACKIEN_R {
    type Target = crate::FieldReader<bool, NACKIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACKIEN` writer - Non-Acknowledge Interrupt Enable"]
pub struct NACKIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACKIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The non-acknowledge interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NACKIEN_A::VALUE1)
    }
    #[doc = "The non-acknowledge interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NACKIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Arbitration Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLIEN_A {
    #[doc = "0: The arbitration lost interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The arbitration lost interrupt is enabled."]
    VALUE2 = 1,
}
impl From<ARLIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ARLIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLIEN` reader - Arbitration Lost Interrupt Enable"]
pub struct ARLIEN_R(crate::FieldReader<bool, ARLIEN_A>);
impl ARLIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARLIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARLIEN_A {
        match self.bits {
            false => ARLIEN_A::VALUE1,
            true => ARLIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ARLIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ARLIEN_A::VALUE2
    }
}
impl core::ops::Deref for ARLIEN_R {
    type Target = crate::FieldReader<bool, ARLIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARLIEN` writer - Arbitration Lost Interrupt Enable"]
pub struct ARLIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARLIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARLIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The arbitration lost interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARLIEN_A::VALUE1)
    }
    #[doc = "The arbitration lost interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARLIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Slave Read Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRRIEN_A {
    #[doc = "0: The slave read request interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The slave read request interrupt is enabled."]
    VALUE2 = 1,
}
impl From<SRRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRRIEN` reader - Slave Read Request Interrupt Enable"]
pub struct SRRIEN_R(crate::FieldReader<bool, SRRIEN_A>);
impl SRRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRRIEN_A {
        match self.bits {
            false => SRRIEN_A::VALUE1,
            true => SRRIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SRRIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SRRIEN_A::VALUE2
    }
}
impl core::ops::Deref for SRRIEN_R {
    type Target = crate::FieldReader<bool, SRRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRRIEN` writer - Slave Read Request Interrupt Enable"]
pub struct SRRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The slave read request interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRRIEN_A::VALUE1)
    }
    #[doc = "The slave read request interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRRIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIEN_A {
    #[doc = "0: The error interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The error interrupt is enabled."]
    VALUE2 = 1,
}
impl From<ERRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIEN` reader - Error Interrupt Enable"]
pub struct ERRIEN_R(crate::FieldReader<bool, ERRIEN_A>);
impl ERRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIEN_A {
        match self.bits {
            false => ERRIEN_A::VALUE1,
            true => ERRIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ERRIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ERRIEN_A::VALUE2
    }
}
impl core::ops::Deref for ERRIEN_R {
    type Target = crate::FieldReader<bool, ERRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRIEN` writer - Error Interrupt Enable"]
pub struct ERRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERRIEN_A::VALUE1)
    }
    #[doc = "The error interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERRIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Slave Acknowledge Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACKDIS_A {
    #[doc = "0: The generation of an active slave acknowledge is enabled (slave acknowledge with 0 level = more bytes can be received)."]
    VALUE1 = 0,
    #[doc = "1: The generation of an active slave acknowledge is disabled (slave acknowledge with 1 level = reception stopped)."]
    VALUE2 = 1,
}
impl From<SACKDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SACKDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKDIS` reader - Slave Acknowledge Disable"]
pub struct SACKDIS_R(crate::FieldReader<bool, SACKDIS_A>);
impl SACKDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SACKDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SACKDIS_A {
        match self.bits {
            false => SACKDIS_A::VALUE1,
            true => SACKDIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SACKDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SACKDIS_A::VALUE2
    }
}
impl core::ops::Deref for SACKDIS_R {
    type Target = crate::FieldReader<bool, SACKDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SACKDIS` writer - Slave Acknowledge Disable"]
pub struct SACKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SACKDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SACKDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The generation of an active slave acknowledge is enabled (slave acknowledge with 0 level = more bytes can be received)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SACKDIS_A::VALUE1)
    }
    #[doc = "The generation of an active slave acknowledge is disabled (slave acknowledge with 1 level = reception stopped)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SACKDIS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `HDEL` reader - Hardware Delay"]
pub struct HDEL_R(crate::FieldReader<u8, u8>);
impl HDEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HDEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDEL` writer - Hardware Delay"]
pub struct HDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HDEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | ((value as u32 & 0x0f) << 26);
        self.w
    }
}
#[doc = "Acknowledge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKIEN_A {
    #[doc = "0: The acknowledge interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The acknowledge interrupt is enabled."]
    VALUE2 = 1,
}
impl From<ACKIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACKIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKIEN` reader - Acknowledge Interrupt Enable"]
pub struct ACKIEN_R(crate::FieldReader<bool, ACKIEN_A>);
impl ACKIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKIEN_A {
        match self.bits {
            false => ACKIEN_A::VALUE1,
            true => ACKIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ACKIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACKIEN_A::VALUE2
    }
}
impl core::ops::Deref for ACKIEN_R {
    type Target = crate::FieldReader<bool, ACKIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKIEN` writer - Acknowledge Interrupt Enable"]
pub struct ACKIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACKIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The acknowledge interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACKIEN_A::VALUE1)
    }
    #[doc = "The acknowledge interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACKIEN_A::VALUE2)
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
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLK_A {
    #[doc = "0: The MCLK generation is disabled and MCLK is 0."]
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
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
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
    #[doc = "Bits 0:15 - Slave Address"]
    #[inline(always)]
    pub fn slad(&self) -> SLAD_R {
        SLAD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Acknowledge 00H"]
    #[inline(always)]
    pub fn ack00(&self) -> ACK00_R {
        ACK00_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Symbol Timing"]
    #[inline(always)]
    pub fn stim(&self) -> STIM_R {
        STIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Start Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn scrien(&self) -> SCRIEN_R {
        SCRIEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Repeated Start Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn rscrien(&self) -> RSCRIEN_R {
        RSCRIEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Stop Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn pcrien(&self) -> PCRIEN_R {
        PCRIEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Non-Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn nackien(&self) -> NACKIEN_R {
        NACKIEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arlien(&self) -> ARLIEN_R {
        ARLIEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Slave Read Request Interrupt Enable"]
    #[inline(always)]
    pub fn srrien(&self) -> SRRIEN_R {
        SRRIEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn errien(&self) -> ERRIEN_R {
        ERRIEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Slave Acknowledge Disable"]
    #[inline(always)]
    pub fn sackdis(&self) -> SACKDIS_R {
        SACKDIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:29 - Hardware Delay"]
    #[inline(always)]
    pub fn hdel(&self) -> HDEL_R {
        HDEL_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn ackien(&self) -> ACKIEN_R {
        ACKIEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MCLK_R {
        MCLK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Slave Address"]
    #[inline(always)]
    pub fn slad(&mut self) -> SLAD_W {
        SLAD_W { w: self }
    }
    #[doc = "Bit 16 - Acknowledge 00H"]
    #[inline(always)]
    pub fn ack00(&mut self) -> ACK00_W {
        ACK00_W { w: self }
    }
    #[doc = "Bit 17 - Symbol Timing"]
    #[inline(always)]
    pub fn stim(&mut self) -> STIM_W {
        STIM_W { w: self }
    }
    #[doc = "Bit 18 - Start Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn scrien(&mut self) -> SCRIEN_W {
        SCRIEN_W { w: self }
    }
    #[doc = "Bit 19 - Repeated Start Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn rscrien(&mut self) -> RSCRIEN_W {
        RSCRIEN_W { w: self }
    }
    #[doc = "Bit 20 - Stop Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn pcrien(&mut self) -> PCRIEN_W {
        PCRIEN_W { w: self }
    }
    #[doc = "Bit 21 - Non-Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn nackien(&mut self) -> NACKIEN_W {
        NACKIEN_W { w: self }
    }
    #[doc = "Bit 22 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arlien(&mut self) -> ARLIEN_W {
        ARLIEN_W { w: self }
    }
    #[doc = "Bit 23 - Slave Read Request Interrupt Enable"]
    #[inline(always)]
    pub fn srrien(&mut self) -> SRRIEN_W {
        SRRIEN_W { w: self }
    }
    #[doc = "Bit 24 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn errien(&mut self) -> ERRIEN_W {
        ERRIEN_W { w: self }
    }
    #[doc = "Bit 25 - Slave Acknowledge Disable"]
    #[inline(always)]
    pub fn sackdis(&mut self) -> SACKDIS_W {
        SACKDIS_W { w: self }
    }
    #[doc = "Bits 26:29 - Hardware Delay"]
    #[inline(always)]
    pub fn hdel(&mut self) -> HDEL_W {
        HDEL_W { w: self }
    }
    #[doc = "Bit 30 - Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn ackien(&mut self) -> ACKIEN_W {
        ACKIEN_W { w: self }
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
#[doc = "Protocol Control Register \\[IIC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr_iicmode](index.html) module"]
pub struct PCR_IICMODE_SPEC;
impl crate::RegisterSpec for PCR_IICMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr_iicmode::R](R) reader structure"]
impl crate::Readable for PCR_IICMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr_iicmode::W](W) writer structure"]
impl crate::Writable for PCR_IICMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCR_IICMode to value 0"]
impl crate::Resettable for PCR_IICMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
