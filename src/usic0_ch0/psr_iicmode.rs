#[doc = "Register `PSR_IICMode` reader"]
pub struct R(crate::R<PSR_IICMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_IICMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_IICMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_IICMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSR_IICMode` writer"]
pub struct W(crate::W<PSR_IICMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_IICMODE_SPEC>;
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
impl From<crate::W<PSR_IICMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_IICMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLSEL` reader - Slave Select"]
pub type SLSEL_R = crate::BitReader<SLSEL_A>;
#[doc = "Slave Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SLSEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `SLSEL` writer - Slave Select"]
pub type SLSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, SLSEL_A, O>;
impl<'a, const O: u8> SLSEL_W<'a, O> {
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
}
#[doc = "Field `WTDF` reader - Wrong TDF Code Found"]
pub type WTDF_R = crate::BitReader<WTDF_A>;
#[doc = "Wrong TDF Code Found\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WTDF_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `WTDF` writer - Wrong TDF Code Found"]
pub type WTDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, WTDF_A, O>;
impl<'a, const O: u8> WTDF_W<'a, O> {
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
}
#[doc = "Field `SCR` reader - Start Condition Received"]
pub type SCR_R = crate::BitReader<SCR_A>;
#[doc = "Start Condition Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SCR_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `SCR` writer - Start Condition Received"]
pub type SCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, SCR_A, O>;
impl<'a, const O: u8> SCR_W<'a, O> {
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
}
#[doc = "Field `RSCR` reader - Repeated Start Condition Received"]
pub type RSCR_R = crate::BitReader<RSCR_A>;
#[doc = "Repeated Start Condition Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RSCR_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `RSCR` writer - Repeated Start Condition Received"]
pub type RSCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, RSCR_A, O>;
impl<'a, const O: u8> RSCR_W<'a, O> {
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
}
#[doc = "Field `PCR` reader - Stop Condition Received"]
pub type PCR_R = crate::BitReader<PCR_A>;
#[doc = "Stop Condition Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PCR_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `PCR` writer - Stop Condition Received"]
pub type PCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, PCR_A, O>;
impl<'a, const O: u8> PCR_W<'a, O> {
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
}
#[doc = "Field `NACK` reader - Non-Acknowledge Received"]
pub type NACK_R = crate::BitReader<NACK_A>;
#[doc = "Non-Acknowledge Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl NACK_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `NACK` writer - Non-Acknowledge Received"]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, NACK_A, O>;
impl<'a, const O: u8> NACK_W<'a, O> {
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
}
#[doc = "Field `ARL` reader - Arbitration Lost"]
pub type ARL_R = crate::BitReader<ARL_A>;
#[doc = "Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ARL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `ARL` writer - Arbitration Lost"]
pub type ARL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, ARL_A, O>;
impl<'a, const O: u8> ARL_W<'a, O> {
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
}
#[doc = "Field `SRR` reader - Slave Read Request"]
pub type SRR_R = crate::BitReader<SRR_A>;
#[doc = "Slave Read Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SRR_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `SRR` writer - Slave Read Request"]
pub type SRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, SRR_A, O>;
impl<'a, const O: u8> SRR_W<'a, O> {
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
}
#[doc = "Field `ERR` reader - Error"]
pub type ERR_R = crate::BitReader<ERR_A>;
#[doc = "Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERR_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `ERR` writer - Error"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, ERR_A, O>;
impl<'a, const O: u8> ERR_W<'a, O> {
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
}
#[doc = "Field `ACK` reader - Acknowledge Received"]
pub type ACK_R = crate::BitReader<ACK_A>;
#[doc = "Acknowledge Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ACK_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `ACK` writer - Acknowledge Received"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, ACK_A, O>;
impl<'a, const O: u8> ACK_W<'a, O> {
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
pub type RSIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, RSIF_A, O>;
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
pub type DLIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, DLIF_A, O>;
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
pub type TSIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, TSIF_A, O>;
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
pub type TBIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, TBIF_A, O>;
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
pub type RIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, RIF_A, O>;
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
pub type AIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, AIF_A, O>;
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
pub type BRGIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_IICMODE_SPEC, BRGIF_A, O>;
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
    #[doc = "Bit 0 - Slave Select"]
    #[inline(always)]
    pub fn slsel(&self) -> SLSEL_R {
        SLSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wrong TDF Code Found"]
    #[inline(always)]
    pub fn wtdf(&self) -> WTDF_R {
        WTDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Condition Received"]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Repeated Start Condition Received"]
    #[inline(always)]
    pub fn rscr(&self) -> RSCR_R {
        RSCR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop Condition Received"]
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-Acknowledge Received"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Arbitration Lost"]
    #[inline(always)]
    pub fn arl(&self) -> ARL_R {
        ARL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave Read Request"]
    #[inline(always)]
    pub fn srr(&self) -> SRR_R {
        SRR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Acknowledge Received"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 0 - Slave Select"]
    #[inline(always)]
    #[must_use]
    pub fn slsel(&mut self) -> SLSEL_W<0> {
        SLSEL_W::new(self)
    }
    #[doc = "Bit 1 - Wrong TDF Code Found"]
    #[inline(always)]
    #[must_use]
    pub fn wtdf(&mut self) -> WTDF_W<1> {
        WTDF_W::new(self)
    }
    #[doc = "Bit 2 - Start Condition Received"]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> SCR_W<2> {
        SCR_W::new(self)
    }
    #[doc = "Bit 3 - Repeated Start Condition Received"]
    #[inline(always)]
    #[must_use]
    pub fn rscr(&mut self) -> RSCR_W<3> {
        RSCR_W::new(self)
    }
    #[doc = "Bit 4 - Stop Condition Received"]
    #[inline(always)]
    #[must_use]
    pub fn pcr(&mut self) -> PCR_W<4> {
        PCR_W::new(self)
    }
    #[doc = "Bit 5 - Non-Acknowledge Received"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<5> {
        NACK_W::new(self)
    }
    #[doc = "Bit 6 - Arbitration Lost"]
    #[inline(always)]
    #[must_use]
    pub fn arl(&mut self) -> ARL_W<6> {
        ARL_W::new(self)
    }
    #[doc = "Bit 7 - Slave Read Request"]
    #[inline(always)]
    #[must_use]
    pub fn srr(&mut self) -> SRR_W<7> {
        SRR_W::new(self)
    }
    #[doc = "Bit 8 - Error"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<8> {
        ERR_W::new(self)
    }
    #[doc = "Bit 9 - Acknowledge Received"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<9> {
        ACK_W::new(self)
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
#[doc = "Protocol Status Register \\[IIC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr_iicmode](index.html) module"]
pub struct PSR_IICMODE_SPEC;
impl crate::RegisterSpec for PSR_IICMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr_iicmode::R](R) reader structure"]
impl crate::Readable for PSR_IICMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psr_iicmode::W](W) writer structure"]
impl crate::Writable for PSR_IICMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSR_IICMode to value 0"]
impl crate::Resettable for PSR_IICMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
