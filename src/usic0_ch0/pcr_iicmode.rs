#[doc = "Reader of register PCR_IICMode"]
pub type R = crate::R<u32, super::PCR_IICMODE>;
#[doc = "Writer for register PCR_IICMode"]
pub type W = crate::W<u32, super::PCR_IICMODE>;
#[doc = "Register PCR_IICMode `reset()`'s with value 0"]
impl crate::ResetValue for super::PCR_IICMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLAD`"]
pub type SLAD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLAD`"]
pub struct SLAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Acknowledge 00H\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK00_A {
    #[doc = "0: The slave device is not sensitive to this address."]
    VALUE1,
    #[doc = "1: The slave device is sensitive to this address."]
    VALUE2,
}
impl From<ACK00_A> for bool {
    #[inline(always)]
    fn from(variant: ACK00_A) -> Self {
        match variant {
            ACK00_A::VALUE1 => false,
            ACK00_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ACK00`"]
pub type ACK00_R = crate::R<bool, ACK00_A>;
impl ACK00_R {
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
        *self == ACK00_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACK00_A::VALUE2
    }
}
#[doc = "Write proxy for field `ACK00`"]
pub struct ACK00_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK00_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACK00_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Symbol Timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STIM_A {
    #[doc = "0: A symbol contains 10 time quanta. The timing is adapted for standard mode (100 kBaud)."]
    VALUE1,
    #[doc = "1: A symbol contains 25 time quanta. The timing is adapted for fast mode (400 kBaud)."]
    VALUE2,
}
impl From<STIM_A> for bool {
    #[inline(always)]
    fn from(variant: STIM_A) -> Self {
        match variant {
            STIM_A::VALUE1 => false,
            STIM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `STIM`"]
pub type STIM_R = crate::R<bool, STIM_A>;
impl STIM_R {
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
        *self == STIM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STIM_A::VALUE2
    }
}
#[doc = "Write proxy for field `STIM`"]
pub struct STIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Start Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCRIEN_A {
    #[doc = "0: The start condition interrupt is disabled."]
    VALUE1,
    #[doc = "1: The start condition interrupt is enabled."]
    VALUE2,
}
impl From<SCRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCRIEN_A) -> Self {
        match variant {
            SCRIEN_A::VALUE1 => false,
            SCRIEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SCRIEN`"]
pub type SCRIEN_R = crate::R<bool, SCRIEN_A>;
impl SCRIEN_R {
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
        *self == SCRIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCRIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `SCRIEN`"]
pub struct SCRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCRIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Repeated Start Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSCRIEN_A {
    #[doc = "0: The repeated start condition interrupt is disabled."]
    VALUE1,
    #[doc = "1: The repeated start condition interrupt is enabled."]
    VALUE2,
}
impl From<RSCRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RSCRIEN_A) -> Self {
        match variant {
            RSCRIEN_A::VALUE1 => false,
            RSCRIEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RSCRIEN`"]
pub type RSCRIEN_R = crate::R<bool, RSCRIEN_A>;
impl RSCRIEN_R {
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
        *self == RSCRIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSCRIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `RSCRIEN`"]
pub struct RSCRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSCRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSCRIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Stop Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCRIEN_A {
    #[doc = "0: The stop condition interrupt is disabled."]
    VALUE1,
    #[doc = "1: The stop condition interrupt is enabled."]
    VALUE2,
}
impl From<PCRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: PCRIEN_A) -> Self {
        match variant {
            PCRIEN_A::VALUE1 => false,
            PCRIEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PCRIEN`"]
pub type PCRIEN_R = crate::R<bool, PCRIEN_A>;
impl PCRIEN_R {
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
        *self == PCRIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCRIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `PCRIEN`"]
pub struct PCRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCRIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Non-Acknowledge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKIEN_A {
    #[doc = "0: The non-acknowledge interrupt is disabled."]
    VALUE1,
    #[doc = "1: The non-acknowledge interrupt is enabled."]
    VALUE2,
}
impl From<NACKIEN_A> for bool {
    #[inline(always)]
    fn from(variant: NACKIEN_A) -> Self {
        match variant {
            NACKIEN_A::VALUE1 => false,
            NACKIEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `NACKIEN`"]
pub type NACKIEN_R = crate::R<bool, NACKIEN_A>;
impl NACKIEN_R {
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
        *self == NACKIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NACKIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `NACKIEN`"]
pub struct NACKIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACKIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Arbitration Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLIEN_A {
    #[doc = "0: The arbitration lost interrupt is disabled."]
    VALUE1,
    #[doc = "1: The arbitration lost interrupt is enabled."]
    VALUE2,
}
impl From<ARLIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ARLIEN_A) -> Self {
        match variant {
            ARLIEN_A::VALUE1 => false,
            ARLIEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ARLIEN`"]
pub type ARLIEN_R = crate::R<bool, ARLIEN_A>;
impl ARLIEN_R {
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
        *self == ARLIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARLIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `ARLIEN`"]
pub struct ARLIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARLIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARLIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Slave Read Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRRIEN_A {
    #[doc = "0: The slave read request interrupt is disabled."]
    VALUE1,
    #[doc = "1: The slave read request interrupt is enabled."]
    VALUE2,
}
impl From<SRRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRRIEN_A) -> Self {
        match variant {
            SRRIEN_A::VALUE1 => false,
            SRRIEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SRRIEN`"]
pub type SRRIEN_R = crate::R<bool, SRRIEN_A>;
impl SRRIEN_R {
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
        *self == SRRIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRRIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `SRRIEN`"]
pub struct SRRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRRIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIEN_A {
    #[doc = "0: The error interrupt is disabled."]
    VALUE1,
    #[doc = "1: The error interrupt is enabled."]
    VALUE2,
}
impl From<ERRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIEN_A) -> Self {
        match variant {
            ERRIEN_A::VALUE1 => false,
            ERRIEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ERRIEN`"]
pub type ERRIEN_R = crate::R<bool, ERRIEN_A>;
impl ERRIEN_R {
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
        *self == ERRIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERRIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `ERRIEN`"]
pub struct ERRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Slave Acknowledge Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACKDIS_A {
    #[doc = "0: The generation of an active slave acknowledge is enabled (slave acknowledge with 0 level = more bytes can be received)."]
    VALUE1,
    #[doc = "1: The generation of an active slave acknowledge is disabled (slave acknowledge with 1 level = reception stopped)."]
    VALUE2,
}
impl From<SACKDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SACKDIS_A) -> Self {
        match variant {
            SACKDIS_A::VALUE1 => false,
            SACKDIS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SACKDIS`"]
pub type SACKDIS_R = crate::R<bool, SACKDIS_A>;
impl SACKDIS_R {
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
        *self == SACKDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SACKDIS_A::VALUE2
    }
}
#[doc = "Write proxy for field `SACKDIS`"]
pub struct SACKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SACKDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SACKDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `HDEL`"]
pub type HDEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HDEL`"]
pub struct HDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HDEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | (((value as u32) & 0x0f) << 26);
        self.w
    }
}
#[doc = "Acknowledge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKIEN_A {
    #[doc = "0: The acknowledge interrupt is disabled."]
    VALUE1,
    #[doc = "1: The acknowledge interrupt is enabled."]
    VALUE2,
}
impl From<ACKIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACKIEN_A) -> Self {
        match variant {
            ACKIEN_A::VALUE1 => false,
            ACKIEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ACKIEN`"]
pub type ACKIEN_R = crate::R<bool, ACKIEN_A>;
impl ACKIEN_R {
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
        *self == ACKIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACKIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `ACKIEN`"]
pub struct ACKIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACKIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLK_A {
    #[doc = "0: The MCLK generation is disabled and MCLK is 0."]
    VALUE1,
    #[doc = "1: The MCLK generation is enabled."]
    VALUE2,
}
impl From<MCLK_A> for bool {
    #[inline(always)]
    fn from(variant: MCLK_A) -> Self {
        match variant {
            MCLK_A::VALUE1 => false,
            MCLK_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `MCLK`"]
pub type MCLK_R = crate::R<bool, MCLK_A>;
impl MCLK_R {
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
        *self == MCLK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCLK_A::VALUE2
    }
}
#[doc = "Write proxy for field `MCLK`"]
pub struct MCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
}
