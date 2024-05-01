#[doc = "Register `PCR_IICMode` reader"]
pub type R = crate::R<PcrIicmodeSpec>;
#[doc = "Register `PCR_IICMode` writer"]
pub type W = crate::W<PcrIicmodeSpec>;
#[doc = "Field `SLAD` reader - Slave Address"]
pub type SladR = crate::FieldReader<u16>;
#[doc = "Field `SLAD` writer - Slave Address"]
pub type SladW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Acknowledge 00H\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ack00 {
    #[doc = "0: The slave device is not sensitive to this address."]
    Value1 = 0,
    #[doc = "1: The slave device is sensitive to this address."]
    Value2 = 1,
}
impl From<Ack00> for bool {
    #[inline(always)]
    fn from(variant: Ack00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK00` reader - Acknowledge 00H"]
pub type Ack00R = crate::BitReader<Ack00>;
impl Ack00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ack00 {
        match self.bits {
            false => Ack00::Value1,
            true => Ack00::Value2,
        }
    }
    #[doc = "The slave device is not sensitive to this address."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ack00::Value1
    }
    #[doc = "The slave device is sensitive to this address."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ack00::Value2
    }
}
#[doc = "Field `ACK00` writer - Acknowledge 00H"]
pub type Ack00W<'a, REG> = crate::BitWriter<'a, REG, Ack00>;
impl<'a, REG> Ack00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The slave device is not sensitive to this address."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ack00::Value1)
    }
    #[doc = "The slave device is sensitive to this address."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ack00::Value2)
    }
}
#[doc = "Symbol Timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stim {
    #[doc = "0: A symbol contains 10 time quanta. The timing is adapted for standard mode (100 kBaud)."]
    Value1 = 0,
    #[doc = "1: A symbol contains 25 time quanta. The timing is adapted for fast mode (400 kBaud)."]
    Value2 = 1,
}
impl From<Stim> for bool {
    #[inline(always)]
    fn from(variant: Stim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STIM` reader - Symbol Timing"]
pub type StimR = crate::BitReader<Stim>;
impl StimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stim {
        match self.bits {
            false => Stim::Value1,
            true => Stim::Value2,
        }
    }
    #[doc = "A symbol contains 10 time quanta. The timing is adapted for standard mode (100 kBaud)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stim::Value1
    }
    #[doc = "A symbol contains 25 time quanta. The timing is adapted for fast mode (400 kBaud)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stim::Value2
    }
}
#[doc = "Field `STIM` writer - Symbol Timing"]
pub type StimW<'a, REG> = crate::BitWriter<'a, REG, Stim>;
impl<'a, REG> StimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A symbol contains 10 time quanta. The timing is adapted for standard mode (100 kBaud)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stim::Value1)
    }
    #[doc = "A symbol contains 25 time quanta. The timing is adapted for fast mode (400 kBaud)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stim::Value2)
    }
}
#[doc = "Start Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scrien {
    #[doc = "0: The start condition interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The start condition interrupt is enabled."]
    Value2 = 1,
}
impl From<Scrien> for bool {
    #[inline(always)]
    fn from(variant: Scrien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCRIEN` reader - Start Condition Received Interrupt Enable"]
pub type ScrienR = crate::BitReader<Scrien>;
impl ScrienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scrien {
        match self.bits {
            false => Scrien::Value1,
            true => Scrien::Value2,
        }
    }
    #[doc = "The start condition interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Scrien::Value1
    }
    #[doc = "The start condition interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Scrien::Value2
    }
}
#[doc = "Field `SCRIEN` writer - Start Condition Received Interrupt Enable"]
pub type ScrienW<'a, REG> = crate::BitWriter<'a, REG, Scrien>;
impl<'a, REG> ScrienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The start condition interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Scrien::Value1)
    }
    #[doc = "The start condition interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Scrien::Value2)
    }
}
#[doc = "Repeated Start Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rscrien {
    #[doc = "0: The repeated start condition interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The repeated start condition interrupt is enabled."]
    Value2 = 1,
}
impl From<Rscrien> for bool {
    #[inline(always)]
    fn from(variant: Rscrien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSCRIEN` reader - Repeated Start Condition Received Interrupt Enable"]
pub type RscrienR = crate::BitReader<Rscrien>;
impl RscrienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rscrien {
        match self.bits {
            false => Rscrien::Value1,
            true => Rscrien::Value2,
        }
    }
    #[doc = "The repeated start condition interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rscrien::Value1
    }
    #[doc = "The repeated start condition interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rscrien::Value2
    }
}
#[doc = "Field `RSCRIEN` writer - Repeated Start Condition Received Interrupt Enable"]
pub type RscrienW<'a, REG> = crate::BitWriter<'a, REG, Rscrien>;
impl<'a, REG> RscrienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The repeated start condition interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rscrien::Value1)
    }
    #[doc = "The repeated start condition interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rscrien::Value2)
    }
}
#[doc = "Stop Condition Received Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcrien {
    #[doc = "0: The stop condition interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The stop condition interrupt is enabled."]
    Value2 = 1,
}
impl From<Pcrien> for bool {
    #[inline(always)]
    fn from(variant: Pcrien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCRIEN` reader - Stop Condition Received Interrupt Enable"]
pub type PcrienR = crate::BitReader<Pcrien>;
impl PcrienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcrien {
        match self.bits {
            false => Pcrien::Value1,
            true => Pcrien::Value2,
        }
    }
    #[doc = "The stop condition interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pcrien::Value1
    }
    #[doc = "The stop condition interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pcrien::Value2
    }
}
#[doc = "Field `PCRIEN` writer - Stop Condition Received Interrupt Enable"]
pub type PcrienW<'a, REG> = crate::BitWriter<'a, REG, Pcrien>;
impl<'a, REG> PcrienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The stop condition interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrien::Value1)
    }
    #[doc = "The stop condition interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrien::Value2)
    }
}
#[doc = "Non-Acknowledge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nackien {
    #[doc = "0: The non-acknowledge interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The non-acknowledge interrupt is enabled."]
    Value2 = 1,
}
impl From<Nackien> for bool {
    #[inline(always)]
    fn from(variant: Nackien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKIEN` reader - Non-Acknowledge Interrupt Enable"]
pub type NackienR = crate::BitReader<Nackien>;
impl NackienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nackien {
        match self.bits {
            false => Nackien::Value1,
            true => Nackien::Value2,
        }
    }
    #[doc = "The non-acknowledge interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Nackien::Value1
    }
    #[doc = "The non-acknowledge interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Nackien::Value2
    }
}
#[doc = "Field `NACKIEN` writer - Non-Acknowledge Interrupt Enable"]
pub type NackienW<'a, REG> = crate::BitWriter<'a, REG, Nackien>;
impl<'a, REG> NackienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The non-acknowledge interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Nackien::Value1)
    }
    #[doc = "The non-acknowledge interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Nackien::Value2)
    }
}
#[doc = "Arbitration Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arlien {
    #[doc = "0: The arbitration lost interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The arbitration lost interrupt is enabled."]
    Value2 = 1,
}
impl From<Arlien> for bool {
    #[inline(always)]
    fn from(variant: Arlien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLIEN` reader - Arbitration Lost Interrupt Enable"]
pub type ArlienR = crate::BitReader<Arlien>;
impl ArlienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arlien {
        match self.bits {
            false => Arlien::Value1,
            true => Arlien::Value2,
        }
    }
    #[doc = "The arbitration lost interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Arlien::Value1
    }
    #[doc = "The arbitration lost interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Arlien::Value2
    }
}
#[doc = "Field `ARLIEN` writer - Arbitration Lost Interrupt Enable"]
pub type ArlienW<'a, REG> = crate::BitWriter<'a, REG, Arlien>;
impl<'a, REG> ArlienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The arbitration lost interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Arlien::Value1)
    }
    #[doc = "The arbitration lost interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Arlien::Value2)
    }
}
#[doc = "Slave Read Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srrien {
    #[doc = "0: The slave read request interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The slave read request interrupt is enabled."]
    Value2 = 1,
}
impl From<Srrien> for bool {
    #[inline(always)]
    fn from(variant: Srrien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRRIEN` reader - Slave Read Request Interrupt Enable"]
pub type SrrienR = crate::BitReader<Srrien>;
impl SrrienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srrien {
        match self.bits {
            false => Srrien::Value1,
            true => Srrien::Value2,
        }
    }
    #[doc = "The slave read request interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srrien::Value1
    }
    #[doc = "The slave read request interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srrien::Value2
    }
}
#[doc = "Field `SRRIEN` writer - Slave Read Request Interrupt Enable"]
pub type SrrienW<'a, REG> = crate::BitWriter<'a, REG, Srrien>;
impl<'a, REG> SrrienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The slave read request interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srrien::Value1)
    }
    #[doc = "The slave read request interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srrien::Value2)
    }
}
#[doc = "Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errien {
    #[doc = "0: The error interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The error interrupt is enabled."]
    Value2 = 1,
}
impl From<Errien> for bool {
    #[inline(always)]
    fn from(variant: Errien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIEN` reader - Error Interrupt Enable"]
pub type ErrienR = crate::BitReader<Errien>;
impl ErrienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errien {
        match self.bits {
            false => Errien::Value1,
            true => Errien::Value2,
        }
    }
    #[doc = "The error interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Errien::Value1
    }
    #[doc = "The error interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Errien::Value2
    }
}
#[doc = "Field `ERRIEN` writer - Error Interrupt Enable"]
pub type ErrienW<'a, REG> = crate::BitWriter<'a, REG, Errien>;
impl<'a, REG> ErrienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The error interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Errien::Value1)
    }
    #[doc = "The error interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Errien::Value2)
    }
}
#[doc = "Slave Acknowledge Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sackdis {
    #[doc = "0: The generation of an active slave acknowledge is enabled (slave acknowledge with 0 level = more bytes can be received)."]
    Value1 = 0,
    #[doc = "1: The generation of an active slave acknowledge is disabled (slave acknowledge with 1 level = reception stopped)."]
    Value2 = 1,
}
impl From<Sackdis> for bool {
    #[inline(always)]
    fn from(variant: Sackdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKDIS` reader - Slave Acknowledge Disable"]
pub type SackdisR = crate::BitReader<Sackdis>;
impl SackdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sackdis {
        match self.bits {
            false => Sackdis::Value1,
            true => Sackdis::Value2,
        }
    }
    #[doc = "The generation of an active slave acknowledge is enabled (slave acknowledge with 0 level = more bytes can be received)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sackdis::Value1
    }
    #[doc = "The generation of an active slave acknowledge is disabled (slave acknowledge with 1 level = reception stopped)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sackdis::Value2
    }
}
#[doc = "Field `SACKDIS` writer - Slave Acknowledge Disable"]
pub type SackdisW<'a, REG> = crate::BitWriter<'a, REG, Sackdis>;
impl<'a, REG> SackdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The generation of an active slave acknowledge is enabled (slave acknowledge with 0 level = more bytes can be received)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sackdis::Value1)
    }
    #[doc = "The generation of an active slave acknowledge is disabled (slave acknowledge with 1 level = reception stopped)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sackdis::Value2)
    }
}
#[doc = "Field `HDEL` reader - Hardware Delay"]
pub type HdelR = crate::FieldReader;
#[doc = "Field `HDEL` writer - Hardware Delay"]
pub type HdelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Acknowledge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackien {
    #[doc = "0: The acknowledge interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The acknowledge interrupt is enabled."]
    Value2 = 1,
}
impl From<Ackien> for bool {
    #[inline(always)]
    fn from(variant: Ackien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKIEN` reader - Acknowledge Interrupt Enable"]
pub type AckienR = crate::BitReader<Ackien>;
impl AckienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackien {
        match self.bits {
            false => Ackien::Value1,
            true => Ackien::Value2,
        }
    }
    #[doc = "The acknowledge interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ackien::Value1
    }
    #[doc = "The acknowledge interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ackien::Value2
    }
}
#[doc = "Field `ACKIEN` writer - Acknowledge Interrupt Enable"]
pub type AckienW<'a, REG> = crate::BitWriter<'a, REG, Ackien>;
impl<'a, REG> AckienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The acknowledge interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ackien::Value1)
    }
    #[doc = "The acknowledge interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ackien::Value2)
    }
}
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclk {
    #[doc = "0: The MCLK generation is disabled and MCLK is 0."]
    Value1 = 0,
    #[doc = "1: The MCLK generation is enabled."]
    Value2 = 1,
}
impl From<Mclk> for bool {
    #[inline(always)]
    fn from(variant: Mclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLK` reader - Master Clock Enable"]
pub type MclkR = crate::BitReader<Mclk>;
impl MclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mclk {
        match self.bits {
            false => Mclk::Value1,
            true => Mclk::Value2,
        }
    }
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mclk::Value1
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mclk::Value2
    }
}
#[doc = "Field `MCLK` writer - Master Clock Enable"]
pub type MclkW<'a, REG> = crate::BitWriter<'a, REG, Mclk>;
impl<'a, REG> MclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mclk::Value1)
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mclk::Value2)
    }
}
impl R {
    #[doc = "Bits 0:15 - Slave Address"]
    #[inline(always)]
    pub fn slad(&self) -> SladR {
        SladR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Acknowledge 00H"]
    #[inline(always)]
    pub fn ack00(&self) -> Ack00R {
        Ack00R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Symbol Timing"]
    #[inline(always)]
    pub fn stim(&self) -> StimR {
        StimR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Start Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn scrien(&self) -> ScrienR {
        ScrienR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Repeated Start Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn rscrien(&self) -> RscrienR {
        RscrienR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stop Condition Received Interrupt Enable"]
    #[inline(always)]
    pub fn pcrien(&self) -> PcrienR {
        PcrienR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Non-Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn nackien(&self) -> NackienR {
        NackienR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arlien(&self) -> ArlienR {
        ArlienR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave Read Request Interrupt Enable"]
    #[inline(always)]
    pub fn srrien(&self) -> SrrienR {
        SrrienR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn errien(&self) -> ErrienR {
        ErrienR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Slave Acknowledge Disable"]
    #[inline(always)]
    pub fn sackdis(&self) -> SackdisR {
        SackdisR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - Hardware Delay"]
    #[inline(always)]
    pub fn hdel(&self) -> HdelR {
        HdelR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn ackien(&self) -> AckienR {
        AckienR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MclkR {
        MclkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn slad(&mut self) -> SladW<PcrIicmodeSpec> {
        SladW::new(self, 0)
    }
    #[doc = "Bit 16 - Acknowledge 00H"]
    #[inline(always)]
    #[must_use]
    pub fn ack00(&mut self) -> Ack00W<PcrIicmodeSpec> {
        Ack00W::new(self, 16)
    }
    #[doc = "Bit 17 - Symbol Timing"]
    #[inline(always)]
    #[must_use]
    pub fn stim(&mut self) -> StimW<PcrIicmodeSpec> {
        StimW::new(self, 17)
    }
    #[doc = "Bit 18 - Start Condition Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scrien(&mut self) -> ScrienW<PcrIicmodeSpec> {
        ScrienW::new(self, 18)
    }
    #[doc = "Bit 19 - Repeated Start Condition Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rscrien(&mut self) -> RscrienW<PcrIicmodeSpec> {
        RscrienW::new(self, 19)
    }
    #[doc = "Bit 20 - Stop Condition Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcrien(&mut self) -> PcrienW<PcrIicmodeSpec> {
        PcrienW::new(self, 20)
    }
    #[doc = "Bit 21 - Non-Acknowledge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nackien(&mut self) -> NackienW<PcrIicmodeSpec> {
        NackienW::new(self, 21)
    }
    #[doc = "Bit 22 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arlien(&mut self) -> ArlienW<PcrIicmodeSpec> {
        ArlienW::new(self, 22)
    }
    #[doc = "Bit 23 - Slave Read Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srrien(&mut self) -> SrrienW<PcrIicmodeSpec> {
        SrrienW::new(self, 23)
    }
    #[doc = "Bit 24 - Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn errien(&mut self) -> ErrienW<PcrIicmodeSpec> {
        ErrienW::new(self, 24)
    }
    #[doc = "Bit 25 - Slave Acknowledge Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sackdis(&mut self) -> SackdisW<PcrIicmodeSpec> {
        SackdisW::new(self, 25)
    }
    #[doc = "Bits 26:29 - Hardware Delay"]
    #[inline(always)]
    #[must_use]
    pub fn hdel(&mut self) -> HdelW<PcrIicmodeSpec> {
        HdelW::new(self, 26)
    }
    #[doc = "Bit 30 - Acknowledge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackien(&mut self) -> AckienW<PcrIicmodeSpec> {
        AckienW::new(self, 30)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mclk(&mut self) -> MclkW<PcrIicmodeSpec> {
        MclkW::new(self, 31)
    }
}
#[doc = "Protocol Control Register \\[IIC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_iicmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_iicmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrIicmodeSpec;
impl crate::RegisterSpec for PcrIicmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr_iicmode::R`](R) reader structure"]
impl crate::Readable for PcrIicmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`pcr_iicmode::W`](W) writer structure"]
impl crate::Writable for PcrIicmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCR_IICMode to value 0"]
impl crate::Resettable for PcrIicmodeSpec {
    const RESET_VALUE: u32 = 0;
}
