#[doc = "Reader of register PSR_IICMode"]
pub type R = crate::R<u32, super::PSR_IICMODE>;
#[doc = "Writer for register PSR_IICMode"]
pub type W = crate::W<u32, super::PSR_IICMODE>;
#[doc = "Register PSR_IICMode `reset()`'s with value 0"]
impl crate::ResetValue for super::PSR_IICMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Slave Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLSEL_A {
    #[doc = "0: The device is not selected as slave."]
    VALUE1 = 0,
    #[doc = "1: The device is selected as slave."]
    VALUE2 = 1,
}
impl From<SLSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SLSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLSEL`"]
pub type SLSEL_R = crate::R<bool, SLSEL_A>;
impl SLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLSEL_A {
        match self.bits {
            false => SLSEL_A::VALUE1,
            true => SLSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SLSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `SLSEL`"]
pub struct SLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The device is not selected as slave."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SLSEL_A::VALUE1)
    }
    #[doc = "The device is selected as slave."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SLSEL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Wrong TDF Code Found\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTDF_A {
    #[doc = "0: A wrong TDF code has not been found."]
    VALUE1 = 0,
    #[doc = "1: A wrong TDF code has been found."]
    VALUE2 = 1,
}
impl From<WTDF_A> for bool {
    #[inline(always)]
    fn from(variant: WTDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WTDF`"]
pub type WTDF_R = crate::R<bool, WTDF_A>;
impl WTDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTDF_A {
        match self.bits {
            false => WTDF_A::VALUE1,
            true => WTDF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WTDF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WTDF_A::VALUE2
    }
}
#[doc = "Write proxy for field `WTDF`"]
pub struct WTDF_W<'a> {
    w: &'a mut W,
}
impl<'a> WTDF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTDF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A wrong TDF code has not been found."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WTDF_A::VALUE1)
    }
    #[doc = "A wrong TDF code has been found."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WTDF_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Start Condition Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCR_A {
    #[doc = "0: A start condition has not yet been detected."]
    VALUE1 = 0,
    #[doc = "1: A start condition has been detected."]
    VALUE2 = 1,
}
impl From<SCR_A> for bool {
    #[inline(always)]
    fn from(variant: SCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCR`"]
pub type SCR_R = crate::R<bool, SCR_A>;
impl SCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCR_A {
        match self.bits {
            false => SCR_A::VALUE1,
            true => SCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCR_A::VALUE2
    }
}
#[doc = "Write proxy for field `SCR`"]
pub struct SCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A start condition has not yet been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCR_A::VALUE1)
    }
    #[doc = "A start condition has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Repeated Start Condition Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSCR_A {
    #[doc = "0: A repeated start condition has not yet been detected."]
    VALUE1 = 0,
    #[doc = "1: A repeated start condition has been detected."]
    VALUE2 = 1,
}
impl From<RSCR_A> for bool {
    #[inline(always)]
    fn from(variant: RSCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSCR`"]
pub type RSCR_R = crate::R<bool, RSCR_A>;
impl RSCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSCR_A {
        match self.bits {
            false => RSCR_A::VALUE1,
            true => RSCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSCR_A::VALUE2
    }
}
#[doc = "Write proxy for field `RSCR`"]
pub struct RSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A repeated start condition has not yet been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSCR_A::VALUE1)
    }
    #[doc = "A repeated start condition has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSCR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Stop Condition Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCR_A {
    #[doc = "0: A stop condition has not yet been detected."]
    VALUE1 = 0,
    #[doc = "1: A stop condition has been detected."]
    VALUE2 = 1,
}
impl From<PCR_A> for bool {
    #[inline(always)]
    fn from(variant: PCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PCR`"]
pub type PCR_R = crate::R<bool, PCR_A>;
impl PCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCR_A {
        match self.bits {
            false => PCR_A::VALUE1,
            true => PCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCR_A::VALUE2
    }
}
#[doc = "Write proxy for field `PCR`"]
pub struct PCR_W<'a> {
    w: &'a mut W,
}
impl<'a> PCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A stop condition has not yet been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCR_A::VALUE1)
    }
    #[doc = "A stop condition has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Non-Acknowledge Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_A {
    #[doc = "0: A non-acknowledge has not been received."]
    VALUE1 = 0,
    #[doc = "1: A non-acknowledge has been received."]
    VALUE2 = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NACK`"]
pub type NACK_R = crate::R<bool, NACK_A>;
impl NACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::VALUE1,
            true => NACK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NACK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NACK_A::VALUE2
    }
}
#[doc = "Write proxy for field `NACK`"]
pub struct NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A non-acknowledge has not been received."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NACK_A::VALUE1)
    }
    #[doc = "A non-acknowledge has been received."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NACK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARL_A {
    #[doc = "0: An arbitration has not been lost."]
    VALUE1 = 0,
    #[doc = "1: An arbitration has been lost."]
    VALUE2 = 1,
}
impl From<ARL_A> for bool {
    #[inline(always)]
    fn from(variant: ARL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARL`"]
pub type ARL_R = crate::R<bool, ARL_A>;
impl ARL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARL_A {
        match self.bits {
            false => ARL_A::VALUE1,
            true => ARL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARL_A::VALUE2
    }
}
#[doc = "Write proxy for field `ARL`"]
pub struct ARL_W<'a> {
    w: &'a mut W,
}
impl<'a> ARL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An arbitration has not been lost."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARL_A::VALUE1)
    }
    #[doc = "An arbitration has been lost."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Slave Read Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRR_A {
    #[doc = "0: A slave read request has not been detected."]
    VALUE1 = 0,
    #[doc = "1: A slave read request has been detected."]
    VALUE2 = 1,
}
impl From<SRR_A> for bool {
    #[inline(always)]
    fn from(variant: SRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRR`"]
pub type SRR_R = crate::R<bool, SRR_A>;
impl SRR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRR_A {
        match self.bits {
            false => SRR_A::VALUE1,
            true => SRR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRR_A::VALUE2
    }
}
#[doc = "Write proxy for field `SRR`"]
pub struct SRR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A slave read request has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRR_A::VALUE1)
    }
    #[doc = "A slave read request has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_A {
    #[doc = "0: An IIC error has not been detected."]
    VALUE1 = 0,
    #[doc = "1: An IIC error has been detected."]
    VALUE2 = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, ERR_A>;
impl ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_A {
        match self.bits {
            false => ERR_A::VALUE1,
            true => ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An IIC error has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERR_A::VALUE1)
    }
    #[doc = "An IIC error has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Acknowledge Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK_A {
    #[doc = "0: An acknowledge has not been received."]
    VALUE1 = 0,
    #[doc = "1: An acknowledge has been received."]
    VALUE2 = 1,
}
impl From<ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<bool, ACK_A>;
impl ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK_A {
        match self.bits {
            false => ACK_A::VALUE1,
            true => ACK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACK_A::VALUE2
    }
}
#[doc = "Write proxy for field `ACK`"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An acknowledge has not been received."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACK_A::VALUE1)
    }
    #[doc = "An acknowledge has been received."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
#[doc = "Reader of field `RSIF`"]
pub type RSIF_R = crate::R<bool, RSIF_A>;
impl RSIF_R {
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
        *self == RSIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSIF_A::VALUE2
    }
}
#[doc = "Write proxy for field `RSIF`"]
pub struct RSIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
#[doc = "Reader of field `DLIF`"]
pub type DLIF_R = crate::R<bool, DLIF_A>;
impl DLIF_R {
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
        *self == DLIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DLIF_A::VALUE2
    }
}
#[doc = "Write proxy for field `DLIF`"]
pub struct DLIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DLIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
#[doc = "Reader of field `TSIF`"]
pub type TSIF_R = crate::R<bool, TSIF_A>;
impl TSIF_R {
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
        *self == TSIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSIF_A::VALUE2
    }
}
#[doc = "Write proxy for field `TSIF`"]
pub struct TSIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
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
#[doc = "Reader of field `TBIF`"]
pub type TBIF_R = crate::R<bool, TBIF_A>;
impl TBIF_R {
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
        *self == TBIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBIF_A::VALUE2
    }
}
#[doc = "Write proxy for field `TBIF`"]
pub struct TBIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
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
#[doc = "Reader of field `RIF`"]
pub type RIF_R = crate::R<bool, RIF_A>;
impl RIF_R {
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
        *self == RIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RIF_A::VALUE2
    }
}
#[doc = "Write proxy for field `RIF`"]
pub struct RIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
#[doc = "Reader of field `AIF`"]
pub type AIF_R = crate::R<bool, AIF_A>;
impl AIF_R {
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
        *self == AIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AIF_A::VALUE2
    }
}
#[doc = "Write proxy for field `AIF`"]
pub struct AIF_W<'a> {
    w: &'a mut W,
}
impl<'a> AIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
#[doc = "Reader of field `BRGIF`"]
pub type BRGIF_R = crate::R<bool, BRGIF_A>;
impl BRGIF_R {
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
        *self == BRGIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BRGIF_A::VALUE2
    }
}
#[doc = "Write proxy for field `BRGIF`"]
pub struct BRGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> BRGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRGIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slave Select"]
    #[inline(always)]
    pub fn slsel(&self) -> SLSEL_R {
        SLSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wrong TDF Code Found"]
    #[inline(always)]
    pub fn wtdf(&self) -> WTDF_R {
        WTDF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start Condition Received"]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Repeated Start Condition Received"]
    #[inline(always)]
    pub fn rscr(&self) -> RSCR_R {
        RSCR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stop Condition Received"]
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Non-Acknowledge Received"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Arbitration Lost"]
    #[inline(always)]
    pub fn arl(&self) -> ARL_R {
        ARL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Slave Read Request"]
    #[inline(always)]
    pub fn srr(&self) -> SRR_R {
        SRR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Acknowledge Received"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 9) & 0x01) != 0)
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
    #[doc = "Bit 0 - Slave Select"]
    #[inline(always)]
    pub fn slsel(&mut self) -> SLSEL_W {
        SLSEL_W { w: self }
    }
    #[doc = "Bit 1 - Wrong TDF Code Found"]
    #[inline(always)]
    pub fn wtdf(&mut self) -> WTDF_W {
        WTDF_W { w: self }
    }
    #[doc = "Bit 2 - Start Condition Received"]
    #[inline(always)]
    pub fn scr(&mut self) -> SCR_W {
        SCR_W { w: self }
    }
    #[doc = "Bit 3 - Repeated Start Condition Received"]
    #[inline(always)]
    pub fn rscr(&mut self) -> RSCR_W {
        RSCR_W { w: self }
    }
    #[doc = "Bit 4 - Stop Condition Received"]
    #[inline(always)]
    pub fn pcr(&mut self) -> PCR_W {
        PCR_W { w: self }
    }
    #[doc = "Bit 5 - Non-Acknowledge Received"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    #[doc = "Bit 6 - Arbitration Lost"]
    #[inline(always)]
    pub fn arl(&mut self) -> ARL_W {
        ARL_W { w: self }
    }
    #[doc = "Bit 7 - Slave Read Request"]
    #[inline(always)]
    pub fn srr(&mut self) -> SRR_W {
        SRR_W { w: self }
    }
    #[doc = "Bit 8 - Error"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 9 - Acknowledge Received"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
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
}
