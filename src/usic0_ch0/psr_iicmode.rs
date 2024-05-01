#[doc = "Register `PSR_IICMode` reader"]
pub type R = crate::R<PsrIicmodeSpec>;
#[doc = "Register `PSR_IICMode` writer"]
pub type W = crate::W<PsrIicmodeSpec>;
#[doc = "Slave Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slsel {
    #[doc = "0: The device is not selected as slave."]
    Value1 = 0,
    #[doc = "1: The device is selected as slave."]
    Value2 = 1,
}
impl From<Slsel> for bool {
    #[inline(always)]
    fn from(variant: Slsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLSEL` reader - Slave Select"]
pub type SlselR = crate::BitReader<Slsel>;
impl SlselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slsel {
        match self.bits {
            false => Slsel::Value1,
            true => Slsel::Value2,
        }
    }
    #[doc = "The device is not selected as slave."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Slsel::Value1
    }
    #[doc = "The device is selected as slave."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Slsel::Value2
    }
}
#[doc = "Field `SLSEL` writer - Slave Select"]
pub type SlselW<'a, REG> = crate::BitWriter<'a, REG, Slsel>;
impl<'a, REG> SlselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The device is not selected as slave."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Slsel::Value1)
    }
    #[doc = "The device is selected as slave."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Slsel::Value2)
    }
}
#[doc = "Wrong TDF Code Found\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wtdf {
    #[doc = "0: A wrong TDF code has not been found."]
    Value1 = 0,
    #[doc = "1: A wrong TDF code has been found."]
    Value2 = 1,
}
impl From<Wtdf> for bool {
    #[inline(always)]
    fn from(variant: Wtdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTDF` reader - Wrong TDF Code Found"]
pub type WtdfR = crate::BitReader<Wtdf>;
impl WtdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wtdf {
        match self.bits {
            false => Wtdf::Value1,
            true => Wtdf::Value2,
        }
    }
    #[doc = "A wrong TDF code has not been found."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wtdf::Value1
    }
    #[doc = "A wrong TDF code has been found."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wtdf::Value2
    }
}
#[doc = "Field `WTDF` writer - Wrong TDF Code Found"]
pub type WtdfW<'a, REG> = crate::BitWriter<'a, REG, Wtdf>;
impl<'a, REG> WtdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A wrong TDF code has not been found."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wtdf::Value1)
    }
    #[doc = "A wrong TDF code has been found."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wtdf::Value2)
    }
}
#[doc = "Start Condition Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scr {
    #[doc = "0: A start condition has not yet been detected."]
    Value1 = 0,
    #[doc = "1: A start condition has been detected."]
    Value2 = 1,
}
impl From<Scr> for bool {
    #[inline(always)]
    fn from(variant: Scr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCR` reader - Start Condition Received"]
pub type ScrR = crate::BitReader<Scr>;
impl ScrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scr {
        match self.bits {
            false => Scr::Value1,
            true => Scr::Value2,
        }
    }
    #[doc = "A start condition has not yet been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Scr::Value1
    }
    #[doc = "A start condition has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Scr::Value2
    }
}
#[doc = "Field `SCR` writer - Start Condition Received"]
pub type ScrW<'a, REG> = crate::BitWriter<'a, REG, Scr>;
impl<'a, REG> ScrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A start condition has not yet been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Scr::Value1)
    }
    #[doc = "A start condition has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Scr::Value2)
    }
}
#[doc = "Repeated Start Condition Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rscr {
    #[doc = "0: A repeated start condition has not yet been detected."]
    Value1 = 0,
    #[doc = "1: A repeated start condition has been detected."]
    Value2 = 1,
}
impl From<Rscr> for bool {
    #[inline(always)]
    fn from(variant: Rscr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSCR` reader - Repeated Start Condition Received"]
pub type RscrR = crate::BitReader<Rscr>;
impl RscrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rscr {
        match self.bits {
            false => Rscr::Value1,
            true => Rscr::Value2,
        }
    }
    #[doc = "A repeated start condition has not yet been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rscr::Value1
    }
    #[doc = "A repeated start condition has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rscr::Value2
    }
}
#[doc = "Field `RSCR` writer - Repeated Start Condition Received"]
pub type RscrW<'a, REG> = crate::BitWriter<'a, REG, Rscr>;
impl<'a, REG> RscrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A repeated start condition has not yet been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rscr::Value1)
    }
    #[doc = "A repeated start condition has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rscr::Value2)
    }
}
#[doc = "Stop Condition Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcr {
    #[doc = "0: A stop condition has not yet been detected."]
    Value1 = 0,
    #[doc = "1: A stop condition has been detected."]
    Value2 = 1,
}
impl From<Pcr> for bool {
    #[inline(always)]
    fn from(variant: Pcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCR` reader - Stop Condition Received"]
pub type PcrR = crate::BitReader<Pcr>;
impl PcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcr {
        match self.bits {
            false => Pcr::Value1,
            true => Pcr::Value2,
        }
    }
    #[doc = "A stop condition has not yet been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pcr::Value1
    }
    #[doc = "A stop condition has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pcr::Value2
    }
}
#[doc = "Field `PCR` writer - Stop Condition Received"]
pub type PcrW<'a, REG> = crate::BitWriter<'a, REG, Pcr>;
impl<'a, REG> PcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A stop condition has not yet been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::Value1)
    }
    #[doc = "A stop condition has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::Value2)
    }
}
#[doc = "Non-Acknowledge Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nack {
    #[doc = "0: A non-acknowledge has not been received."]
    Value1 = 0,
    #[doc = "1: A non-acknowledge has been received."]
    Value2 = 1,
}
impl From<Nack> for bool {
    #[inline(always)]
    fn from(variant: Nack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - Non-Acknowledge Received"]
pub type NackR = crate::BitReader<Nack>;
impl NackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nack {
        match self.bits {
            false => Nack::Value1,
            true => Nack::Value2,
        }
    }
    #[doc = "A non-acknowledge has not been received."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Nack::Value1
    }
    #[doc = "A non-acknowledge has been received."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Nack::Value2
    }
}
#[doc = "Field `NACK` writer - Non-Acknowledge Received"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG, Nack>;
impl<'a, REG> NackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A non-acknowledge has not been received."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Nack::Value1)
    }
    #[doc = "A non-acknowledge has been received."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Nack::Value2)
    }
}
#[doc = "Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arl {
    #[doc = "0: An arbitration has not been lost."]
    Value1 = 0,
    #[doc = "1: An arbitration has been lost."]
    Value2 = 1,
}
impl From<Arl> for bool {
    #[inline(always)]
    fn from(variant: Arl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARL` reader - Arbitration Lost"]
pub type ArlR = crate::BitReader<Arl>;
impl ArlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arl {
        match self.bits {
            false => Arl::Value1,
            true => Arl::Value2,
        }
    }
    #[doc = "An arbitration has not been lost."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Arl::Value1
    }
    #[doc = "An arbitration has been lost."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Arl::Value2
    }
}
#[doc = "Field `ARL` writer - Arbitration Lost"]
pub type ArlW<'a, REG> = crate::BitWriter<'a, REG, Arl>;
impl<'a, REG> ArlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An arbitration has not been lost."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Arl::Value1)
    }
    #[doc = "An arbitration has been lost."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Arl::Value2)
    }
}
#[doc = "Slave Read Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srr {
    #[doc = "0: A slave read request has not been detected."]
    Value1 = 0,
    #[doc = "1: A slave read request has been detected."]
    Value2 = 1,
}
impl From<Srr> for bool {
    #[inline(always)]
    fn from(variant: Srr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRR` reader - Slave Read Request"]
pub type SrrR = crate::BitReader<Srr>;
impl SrrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srr {
        match self.bits {
            false => Srr::Value1,
            true => Srr::Value2,
        }
    }
    #[doc = "A slave read request has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srr::Value1
    }
    #[doc = "A slave read request has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srr::Value2
    }
}
#[doc = "Field `SRR` writer - Slave Read Request"]
pub type SrrW<'a, REG> = crate::BitWriter<'a, REG, Srr>;
impl<'a, REG> SrrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A slave read request has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srr::Value1)
    }
    #[doc = "A slave read request has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srr::Value2)
    }
}
#[doc = "Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Err {
    #[doc = "0: An IIC error has not been detected."]
    Value1 = 0,
    #[doc = "1: An IIC error has been detected."]
    Value2 = 1,
}
impl From<Err> for bool {
    #[inline(always)]
    fn from(variant: Err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` reader - Error"]
pub type ErrR = crate::BitReader<Err>;
impl ErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Err {
        match self.bits {
            false => Err::Value1,
            true => Err::Value2,
        }
    }
    #[doc = "An IIC error has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Err::Value1
    }
    #[doc = "An IIC error has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Err::Value2
    }
}
#[doc = "Field `ERR` writer - Error"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG, Err>;
impl<'a, REG> ErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An IIC error has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Err::Value1)
    }
    #[doc = "An IIC error has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Err::Value2)
    }
}
#[doc = "Acknowledge Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ack {
    #[doc = "0: An acknowledge has not been received."]
    Value1 = 0,
    #[doc = "1: An acknowledge has been received."]
    Value2 = 1,
}
impl From<Ack> for bool {
    #[inline(always)]
    fn from(variant: Ack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK` reader - Acknowledge Received"]
pub type AckR = crate::BitReader<Ack>;
impl AckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ack {
        match self.bits {
            false => Ack::Value1,
            true => Ack::Value2,
        }
    }
    #[doc = "An acknowledge has not been received."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ack::Value1
    }
    #[doc = "An acknowledge has been received."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ack::Value2
    }
}
#[doc = "Field `ACK` writer - Acknowledge Received"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG, Ack>;
impl<'a, REG> AckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An acknowledge has not been received."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::Value1)
    }
    #[doc = "An acknowledge has been received."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::Value2)
    }
}
#[doc = "Receiver Start Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsif {
    #[doc = "0: A receiver start event has not occurred."]
    Value1 = 0,
    #[doc = "1: A receiver start event has occurred."]
    Value2 = 1,
}
impl From<Rsif> for bool {
    #[inline(always)]
    fn from(variant: Rsif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSIF` reader - Receiver Start Indication Flag"]
pub type RsifR = crate::BitReader<Rsif>;
impl RsifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsif {
        match self.bits {
            false => Rsif::Value1,
            true => Rsif::Value2,
        }
    }
    #[doc = "A receiver start event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rsif::Value1
    }
    #[doc = "A receiver start event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rsif::Value2
    }
}
#[doc = "Field `RSIF` writer - Receiver Start Indication Flag"]
pub type RsifW<'a, REG> = crate::BitWriter<'a, REG, Rsif>;
impl<'a, REG> RsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A receiver start event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsif::Value1)
    }
    #[doc = "A receiver start event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rsif::Value2)
    }
}
#[doc = "Data Lost Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlif {
    #[doc = "0: A data lost event has not occurred."]
    Value1 = 0,
    #[doc = "1: A data lost event has occurred."]
    Value2 = 1,
}
impl From<Dlif> for bool {
    #[inline(always)]
    fn from(variant: Dlif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLIF` reader - Data Lost Indication Flag"]
pub type DlifR = crate::BitReader<Dlif>;
impl DlifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlif {
        match self.bits {
            false => Dlif::Value1,
            true => Dlif::Value2,
        }
    }
    #[doc = "A data lost event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dlif::Value1
    }
    #[doc = "A data lost event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dlif::Value2
    }
}
#[doc = "Field `DLIF` writer - Data Lost Indication Flag"]
pub type DlifW<'a, REG> = crate::BitWriter<'a, REG, Dlif>;
impl<'a, REG> DlifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A data lost event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlif::Value1)
    }
    #[doc = "A data lost event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dlif::Value2)
    }
}
#[doc = "Transmit Shift Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsif {
    #[doc = "0: A transmit shift event has not occurred."]
    Value1 = 0,
    #[doc = "1: A transmit shift event has occurred."]
    Value2 = 1,
}
impl From<Tsif> for bool {
    #[inline(always)]
    fn from(variant: Tsif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIF` reader - Transmit Shift Indication Flag"]
pub type TsifR = crate::BitReader<Tsif>;
impl TsifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsif {
        match self.bits {
            false => Tsif::Value1,
            true => Tsif::Value2,
        }
    }
    #[doc = "A transmit shift event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsif::Value1
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsif::Value2
    }
}
#[doc = "Field `TSIF` writer - Transmit Shift Indication Flag"]
pub type TsifW<'a, REG> = crate::BitWriter<'a, REG, Tsif>;
impl<'a, REG> TsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A transmit shift event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsif::Value1)
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsif::Value2)
    }
}
#[doc = "Transmit Buffer Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbif {
    #[doc = "0: A transmit buffer event has not occurred."]
    Value1 = 0,
    #[doc = "1: A transmit buffer event has occurred."]
    Value2 = 1,
}
impl From<Tbif> for bool {
    #[inline(always)]
    fn from(variant: Tbif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBIF` reader - Transmit Buffer Indication Flag"]
pub type TbifR = crate::BitReader<Tbif>;
impl TbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbif {
        match self.bits {
            false => Tbif::Value1,
            true => Tbif::Value2,
        }
    }
    #[doc = "A transmit buffer event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tbif::Value1
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tbif::Value2
    }
}
#[doc = "Field `TBIF` writer - Transmit Buffer Indication Flag"]
pub type TbifW<'a, REG> = crate::BitWriter<'a, REG, Tbif>;
impl<'a, REG> TbifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A transmit buffer event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tbif::Value1)
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tbif::Value2)
    }
}
#[doc = "Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rif {
    #[doc = "0: A receive event has not occurred."]
    Value1 = 0,
    #[doc = "1: A receive event has occurred."]
    Value2 = 1,
}
impl From<Rif> for bool {
    #[inline(always)]
    fn from(variant: Rif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIF` reader - Receive Indication Flag"]
pub type RifR = crate::BitReader<Rif>;
impl RifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rif {
        match self.bits {
            false => Rif::Value1,
            true => Rif::Value2,
        }
    }
    #[doc = "A receive event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rif::Value1
    }
    #[doc = "A receive event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rif::Value2
    }
}
#[doc = "Field `RIF` writer - Receive Indication Flag"]
pub type RifW<'a, REG> = crate::BitWriter<'a, REG, Rif>;
impl<'a, REG> RifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rif::Value1)
    }
    #[doc = "A receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rif::Value2)
    }
}
#[doc = "Alternative Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aif {
    #[doc = "0: An alternative receive event has not occurred."]
    Value1 = 0,
    #[doc = "1: An alternative receive event has occurred."]
    Value2 = 1,
}
impl From<Aif> for bool {
    #[inline(always)]
    fn from(variant: Aif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIF` reader - Alternative Receive Indication Flag"]
pub type AifR = crate::BitReader<Aif>;
impl AifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aif {
        match self.bits {
            false => Aif::Value1,
            true => Aif::Value2,
        }
    }
    #[doc = "An alternative receive event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Aif::Value1
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Aif::Value2
    }
}
#[doc = "Field `AIF` writer - Alternative Receive Indication Flag"]
pub type AifW<'a, REG> = crate::BitWriter<'a, REG, Aif>;
impl<'a, REG> AifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An alternative receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Aif::Value1)
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Aif::Value2)
    }
}
#[doc = "Baud Rate Generator Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brgif {
    #[doc = "0: A baud rate generator event has not occurred."]
    Value1 = 0,
    #[doc = "1: A baud rate generator event has occurred."]
    Value2 = 1,
}
impl From<Brgif> for bool {
    #[inline(always)]
    fn from(variant: Brgif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRGIF` reader - Baud Rate Generator Indication Flag"]
pub type BrgifR = crate::BitReader<Brgif>;
impl BrgifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brgif {
        match self.bits {
            false => Brgif::Value1,
            true => Brgif::Value2,
        }
    }
    #[doc = "A baud rate generator event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Brgif::Value1
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Brgif::Value2
    }
}
#[doc = "Field `BRGIF` writer - Baud Rate Generator Indication Flag"]
pub type BrgifW<'a, REG> = crate::BitWriter<'a, REG, Brgif>;
impl<'a, REG> BrgifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A baud rate generator event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Brgif::Value1)
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Brgif::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Select"]
    #[inline(always)]
    pub fn slsel(&self) -> SlselR {
        SlselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wrong TDF Code Found"]
    #[inline(always)]
    pub fn wtdf(&self) -> WtdfR {
        WtdfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Condition Received"]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Repeated Start Condition Received"]
    #[inline(always)]
    pub fn rscr(&self) -> RscrR {
        RscrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop Condition Received"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-Acknowledge Received"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Arbitration Lost"]
    #[inline(always)]
    pub fn arl(&self) -> ArlR {
        ArlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave Read Request"]
    #[inline(always)]
    pub fn srr(&self) -> SrrR {
        SrrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Acknowledge Received"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    pub fn rsif(&self) -> RsifR {
        RsifR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    pub fn dlif(&self) -> DlifR {
        DlifR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    pub fn tsif(&self) -> TsifR {
        TsifR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    pub fn tbif(&self) -> TbifR {
        TbifR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    pub fn rif(&self) -> RifR {
        RifR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    pub fn aif(&self) -> AifR {
        AifR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    pub fn brgif(&self) -> BrgifR {
        BrgifR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Select"]
    #[inline(always)]
    #[must_use]
    pub fn slsel(&mut self) -> SlselW<PsrIicmodeSpec> {
        SlselW::new(self, 0)
    }
    #[doc = "Bit 1 - Wrong TDF Code Found"]
    #[inline(always)]
    #[must_use]
    pub fn wtdf(&mut self) -> WtdfW<PsrIicmodeSpec> {
        WtdfW::new(self, 1)
    }
    #[doc = "Bit 2 - Start Condition Received"]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> ScrW<PsrIicmodeSpec> {
        ScrW::new(self, 2)
    }
    #[doc = "Bit 3 - Repeated Start Condition Received"]
    #[inline(always)]
    #[must_use]
    pub fn rscr(&mut self) -> RscrW<PsrIicmodeSpec> {
        RscrW::new(self, 3)
    }
    #[doc = "Bit 4 - Stop Condition Received"]
    #[inline(always)]
    #[must_use]
    pub fn pcr(&mut self) -> PcrW<PsrIicmodeSpec> {
        PcrW::new(self, 4)
    }
    #[doc = "Bit 5 - Non-Acknowledge Received"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<PsrIicmodeSpec> {
        NackW::new(self, 5)
    }
    #[doc = "Bit 6 - Arbitration Lost"]
    #[inline(always)]
    #[must_use]
    pub fn arl(&mut self) -> ArlW<PsrIicmodeSpec> {
        ArlW::new(self, 6)
    }
    #[doc = "Bit 7 - Slave Read Request"]
    #[inline(always)]
    #[must_use]
    pub fn srr(&mut self) -> SrrW<PsrIicmodeSpec> {
        SrrW::new(self, 7)
    }
    #[doc = "Bit 8 - Error"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<PsrIicmodeSpec> {
        ErrW::new(self, 8)
    }
    #[doc = "Bit 9 - Acknowledge Received"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<PsrIicmodeSpec> {
        AckW::new(self, 9)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsif(&mut self) -> RsifW<PsrIicmodeSpec> {
        RsifW::new(self, 10)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlif(&mut self) -> DlifW<PsrIicmodeSpec> {
        DlifW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsif(&mut self) -> TsifW<PsrIicmodeSpec> {
        TsifW::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tbif(&mut self) -> TbifW<PsrIicmodeSpec> {
        TbifW::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rif(&mut self) -> RifW<PsrIicmodeSpec> {
        RifW::new(self, 14)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aif(&mut self) -> AifW<PsrIicmodeSpec> {
        AifW::new(self, 15)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn brgif(&mut self) -> BrgifW<PsrIicmodeSpec> {
        BrgifW::new(self, 16)
    }
}
#[doc = "Protocol Status Register \\[IIC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_iicmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_iicmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrIicmodeSpec;
impl crate::RegisterSpec for PsrIicmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr_iicmode::R`](R) reader structure"]
impl crate::Readable for PsrIicmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`psr_iicmode::W`](W) writer structure"]
impl crate::Writable for PsrIicmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR_IICMode to value 0"]
impl crate::Resettable for PsrIicmodeSpec {
    const RESET_VALUE: u32 = 0;
}
