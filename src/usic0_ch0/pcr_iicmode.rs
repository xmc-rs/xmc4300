#[doc = "Register `PCR_IICMode` reader"]
pub type R = crate::R<PCR_IICMODE_SPEC>;
#[doc = "Register `PCR_IICMode` writer"]
pub type W = crate::W<PCR_IICMODE_SPEC>;
#[doc = "Field `SLAD` reader - Slave Address"]
pub type SLAD_R = crate::FieldReader<u16>;
#[doc = "Field `SLAD` writer - Slave Address"]
pub type SLAD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ACK00` reader - Acknowledge 00H"]
pub type ACK00_R = crate::BitReader<ACK00_A>;
#[doc = "Acknowledge 00H\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ACK00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACK00_A {
        match self.bits {
            false => ACK00_A::VALUE1,
            true => ACK00_A::VALUE2,
        }
    }
    #[doc = "The slave device is not sensitive to this address."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACK00_A::VALUE1
    }
    #[doc = "The slave device is sensitive to this address."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACK00_A::VALUE2
    }
}
#[doc = "Field `ACK00` writer - Acknowledge 00H"]
pub type ACK00_W<'a, REG> = crate::BitWriter<'a, REG, ACK00_A>;
impl<'a, REG> ACK00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The slave device is not sensitive to this address."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ACK00_A::VALUE1)
    }
    #[doc = "The slave device is sensitive to this address."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ACK00_A::VALUE2)
    }
}
#[doc = "Field `STIM` reader - Symbol Timing"]
pub type STIM_R = crate::BitReader<STIM_A>;
#[doc = "Symbol Timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl STIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STIM_A {
        match self.bits {
            false => STIM_A::VALUE1,
            true => STIM_A::VALUE2,
        }
    }
    #[doc = "A symbol contains 10 time quanta. The timing is adapted for standard mode (100 kBaud)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STIM_A::VALUE1
    }
    #[doc = "A symbol contains 25 time quanta. The timing is adapted for fast mode (400 kBaud)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STIM_A::VALUE2
    }
}
#[doc = "Field `STIM` writer - Symbol Timing"]
pub type STIM_W<'a, REG> = crate::BitWriter<'a, REG, STIM_A>;
impl<'a, REG> STIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A symbol contains 10 time quanta. The timing is adapted for standard mode (100 kBaud)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STIM_A::VALUE1)
    }
    #[doc = "A symbol contains 25 time quanta. The timing is adapted for fast mode (400 kBaud)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STIM_A::VALUE2)
    }
}
#[doc = "Field `SCRIEN` reader - Start Condition Received Interrupt Enable"]
pub type SCRIEN_R = crate::BitReader<SCRIEN_A>;
#[doc = "Start Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SCRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCRIEN_A {
        match self.bits {
            false => SCRIEN_A::VALUE1,
            true => SCRIEN_A::VALUE2,
        }
    }
    #[doc = "The start condition interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCRIEN_A::VALUE1
    }
    #[doc = "The start condition interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCRIEN_A::VALUE2
    }
}
#[doc = "Field `SCRIEN` writer - Start Condition Received Interrupt Enable"]
pub type SCRIEN_W<'a, REG> = crate::BitWriter<'a, REG, SCRIEN_A>;
impl<'a, REG> SCRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The start condition interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SCRIEN_A::VALUE1)
    }
    #[doc = "The start condition interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SCRIEN_A::VALUE2)
    }
}
#[doc = "Field `RSCRIEN` reader - Repeated Start Condition Received Interrupt Enable"]
pub type RSCRIEN_R = crate::BitReader<RSCRIEN_A>;
#[doc = "Repeated Start Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RSCRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSCRIEN_A {
        match self.bits {
            false => RSCRIEN_A::VALUE1,
            true => RSCRIEN_A::VALUE2,
        }
    }
    #[doc = "The repeated start condition interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSCRIEN_A::VALUE1
    }
    #[doc = "The repeated start condition interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSCRIEN_A::VALUE2
    }
}
#[doc = "Field `RSCRIEN` writer - Repeated Start Condition Received Interrupt Enable"]
pub type RSCRIEN_W<'a, REG> = crate::BitWriter<'a, REG, RSCRIEN_A>;
impl<'a, REG> RSCRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The repeated start condition interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RSCRIEN_A::VALUE1)
    }
    #[doc = "The repeated start condition interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RSCRIEN_A::VALUE2)
    }
}
#[doc = "Field `PCRIEN` reader - Stop Condition Received Interrupt Enable"]
pub type PCRIEN_R = crate::BitReader<PCRIEN_A>;
#[doc = "Stop Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PCRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCRIEN_A {
        match self.bits {
            false => PCRIEN_A::VALUE1,
            true => PCRIEN_A::VALUE2,
        }
    }
    #[doc = "The stop condition interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PCRIEN_A::VALUE1
    }
    #[doc = "The stop condition interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCRIEN_A::VALUE2
    }
}
#[doc = "Field `PCRIEN` writer - Stop Condition Received Interrupt Enable"]
pub type PCRIEN_W<'a, REG> = crate::BitWriter<'a, REG, PCRIEN_A>;
impl<'a, REG> PCRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The stop condition interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PCRIEN_A::VALUE1)
    }
    #[doc = "The stop condition interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PCRIEN_A::VALUE2)
    }
}
#[doc = "Field `NACKIEN` reader - Non-Acknowledge Interrupt Enable"]
pub type NACKIEN_R = crate::BitReader<NACKIEN_A>;
#[doc = "Non-Acknowledge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl NACKIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NACKIEN_A {
        match self.bits {
            false => NACKIEN_A::VALUE1,
            true => NACKIEN_A::VALUE2,
        }
    }
    #[doc = "The non-acknowledge interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NACKIEN_A::VALUE1
    }
    #[doc = "The non-acknowledge interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NACKIEN_A::VALUE2
    }
}
#[doc = "Field `NACKIEN` writer - Non-Acknowledge Interrupt Enable"]
pub type NACKIEN_W<'a, REG> = crate::BitWriter<'a, REG, NACKIEN_A>;
impl<'a, REG> NACKIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The non-acknowledge interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NACKIEN_A::VALUE1)
    }
    #[doc = "The non-acknowledge interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NACKIEN_A::VALUE2)
    }
}
#[doc = "Field `ARLIEN` reader - Arbitration Lost Interrupt Enable"]
pub type ARLIEN_R = crate::BitReader<ARLIEN_A>;
#[doc = "Arbitration Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ARLIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARLIEN_A {
        match self.bits {
            false => ARLIEN_A::VALUE1,
            true => ARLIEN_A::VALUE2,
        }
    }
    #[doc = "The arbitration lost interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARLIEN_A::VALUE1
    }
    #[doc = "The arbitration lost interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARLIEN_A::VALUE2
    }
}
#[doc = "Field `ARLIEN` writer - Arbitration Lost Interrupt Enable"]
pub type ARLIEN_W<'a, REG> = crate::BitWriter<'a, REG, ARLIEN_A>;
impl<'a, REG> ARLIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The arbitration lost interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ARLIEN_A::VALUE1)
    }
    #[doc = "The arbitration lost interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ARLIEN_A::VALUE2)
    }
}
#[doc = "Field `SRRIEN` reader - Slave Read Request Interrupt Enable"]
pub type SRRIEN_R = crate::BitReader<SRRIEN_A>;
#[doc = "Slave Read Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SRRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRRIEN_A {
        match self.bits {
            false => SRRIEN_A::VALUE1,
            true => SRRIEN_A::VALUE2,
        }
    }
    #[doc = "The slave read request interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRRIEN_A::VALUE1
    }
    #[doc = "The slave read request interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRRIEN_A::VALUE2
    }
}
#[doc = "Field `SRRIEN` writer - Slave Read Request Interrupt Enable"]
pub type SRRIEN_W<'a, REG> = crate::BitWriter<'a, REG, SRRIEN_A>;
impl<'a, REG> SRRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The slave read request interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SRRIEN_A::VALUE1)
    }
    #[doc = "The slave read request interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SRRIEN_A::VALUE2)
    }
}
#[doc = "Field `ERRIEN` reader - Error Interrupt Enable"]
pub type ERRIEN_R = crate::BitReader<ERRIEN_A>;
#[doc = "Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIEN_A {
        match self.bits {
            false => ERRIEN_A::VALUE1,
            true => ERRIEN_A::VALUE2,
        }
    }
    #[doc = "The error interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERRIEN_A::VALUE1
    }
    #[doc = "The error interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERRIEN_A::VALUE2
    }
}
#[doc = "Field `ERRIEN` writer - Error Interrupt Enable"]
pub type ERRIEN_W<'a, REG> = crate::BitWriter<'a, REG, ERRIEN_A>;
impl<'a, REG> ERRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The error interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIEN_A::VALUE1)
    }
    #[doc = "The error interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIEN_A::VALUE2)
    }
}
#[doc = "Field `SACKDIS` reader - Slave Acknowledge Disable"]
pub type SACKDIS_R = crate::BitReader<SACKDIS_A>;
#[doc = "Slave Acknowledge Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SACKDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SACKDIS_A {
        match self.bits {
            false => SACKDIS_A::VALUE1,
            true => SACKDIS_A::VALUE2,
        }
    }
    #[doc = "The generation of an active slave acknowledge is enabled (slave acknowledge with 0 level = more bytes can be received)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SACKDIS_A::VALUE1
    }
    #[doc = "The generation of an active slave acknowledge is disabled (slave acknowledge with 1 level = reception stopped)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SACKDIS_A::VALUE2
    }
}
#[doc = "Field `SACKDIS` writer - Slave Acknowledge Disable"]
pub type SACKDIS_W<'a, REG> = crate::BitWriter<'a, REG, SACKDIS_A>;
impl<'a, REG> SACKDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The generation of an active slave acknowledge is enabled (slave acknowledge with 0 level = more bytes can be received)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SACKDIS_A::VALUE1)
    }
    #[doc = "The generation of an active slave acknowledge is disabled (slave acknowledge with 1 level = reception stopped)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SACKDIS_A::VALUE2)
    }
}
#[doc = "Field `HDEL` reader - Hardware Delay"]
pub type HDEL_R = crate::FieldReader;
#[doc = "Field `HDEL` writer - Hardware Delay"]
pub type HDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACKIEN` reader - Acknowledge Interrupt Enable"]
pub type ACKIEN_R = crate::BitReader<ACKIEN_A>;
#[doc = "Acknowledge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ACKIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACKIEN_A {
        match self.bits {
            false => ACKIEN_A::VALUE1,
            true => ACKIEN_A::VALUE2,
        }
    }
    #[doc = "The acknowledge interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACKIEN_A::VALUE1
    }
    #[doc = "The acknowledge interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACKIEN_A::VALUE2
    }
}
#[doc = "Field `ACKIEN` writer - Acknowledge Interrupt Enable"]
pub type ACKIEN_W<'a, REG> = crate::BitWriter<'a, REG, ACKIEN_A>;
impl<'a, REG> ACKIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The acknowledge interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ACKIEN_A::VALUE1)
    }
    #[doc = "The acknowledge interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ACKIEN_A::VALUE2)
    }
}
#[doc = "Field `MCLK` reader - Master Clock Enable"]
pub type MCLK_R = crate::BitReader<MCLK_A>;
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCLK_A {
        match self.bits {
            false => MCLK_A::VALUE1,
            true => MCLK_A::VALUE2,
        }
    }
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCLK_A::VALUE1
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCLK_A::VALUE2
    }
}
#[doc = "Field `MCLK` writer - Master Clock Enable"]
pub type MCLK_W<'a, REG> = crate::BitWriter<'a, REG, MCLK_A>;
impl<'a, REG> MCLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCLK_A::VALUE1)
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCLK_A::VALUE2)
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
        ACK00_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Symbol Timing"]
    #[inline(always)]
    pub fn stim(&self) -> STIM_R {
        STIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Start Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn scrien(&self) -> SCRIEN_R {
        SCRIEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Repeated Start Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn rscrien(&self) -> RSCRIEN_R {
        RSCRIEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stop Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn pcrien(&self) -> PCRIEN_R {
        PCRIEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Non-Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn nackien(&self) -> NACKIEN_R {
        NACKIEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arlien(&self) -> ARLIEN_R {
        ARLIEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave Read Request Interrupt Enable"]
    #[inline(always)]
    pub fn srrien(&self) -> SRRIEN_R {
        SRRIEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn errien(&self) -> ERRIEN_R {
        ERRIEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Slave Acknowledge Disable"]
    #[inline(always)]
    pub fn sackdis(&self) -> SACKDIS_R {
        SACKDIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - Hardware Delay"]
    #[inline(always)]
    pub fn hdel(&self) -> HDEL_R {
        HDEL_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn ackien(&self) -> ACKIEN_R {
        ACKIEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MCLK_R {
        MCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn slad(&mut self) -> SLAD_W<PCR_IICMODE_SPEC> {
        SLAD_W::new(self, 0)
    }
    #[doc = "Bit 16 - Acknowledge 00H"]
    #[inline(always)]
    #[must_use]
    pub fn ack00(&mut self) -> ACK00_W<PCR_IICMODE_SPEC> {
        ACK00_W::new(self, 16)
    }
    #[doc = "Bit 17 - Symbol Timing"]
    #[inline(always)]
    #[must_use]
    pub fn stim(&mut self) -> STIM_W<PCR_IICMODE_SPEC> {
        STIM_W::new(self, 17)
    }
    #[doc = "Bit 18 - Start Condition Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scrien(&mut self) -> SCRIEN_W<PCR_IICMODE_SPEC> {
        SCRIEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Repeated Start Condition Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rscrien(&mut self) -> RSCRIEN_W<PCR_IICMODE_SPEC> {
        RSCRIEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Stop Condition Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcrien(&mut self) -> PCRIEN_W<PCR_IICMODE_SPEC> {
        PCRIEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Non-Acknowledge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nackien(&mut self) -> NACKIEN_W<PCR_IICMODE_SPEC> {
        NACKIEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arlien(&mut self) -> ARLIEN_W<PCR_IICMODE_SPEC> {
        ARLIEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Slave Read Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srrien(&mut self) -> SRRIEN_W<PCR_IICMODE_SPEC> {
        SRRIEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn errien(&mut self) -> ERRIEN_W<PCR_IICMODE_SPEC> {
        ERRIEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Slave Acknowledge Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sackdis(&mut self) -> SACKDIS_W<PCR_IICMODE_SPEC> {
        SACKDIS_W::new(self, 25)
    }
    #[doc = "Bits 26:29 - Hardware Delay"]
    #[inline(always)]
    #[must_use]
    pub fn hdel(&mut self) -> HDEL_W<PCR_IICMODE_SPEC> {
        HDEL_W::new(self, 26)
    }
    #[doc = "Bit 30 - Acknowledge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackien(&mut self) -> ACKIEN_W<PCR_IICMODE_SPEC> {
        ACKIEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mclk(&mut self) -> MCLK_W<PCR_IICMODE_SPEC> {
        MCLK_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Protocol Control Register \\[IIC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_iicmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_iicmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCR_IICMODE_SPEC;
impl crate::RegisterSpec for PCR_IICMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr_iicmode::R`](R) reader structure"]
impl crate::Readable for PCR_IICMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcr_iicmode::W`](W) writer structure"]
impl crate::Writable for PCR_IICMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCR_IICMode to value 0"]
impl crate::Resettable for PCR_IICMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
