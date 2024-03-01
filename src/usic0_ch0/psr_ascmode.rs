#[doc = "Register `PSR_ASCMode` reader"]
pub type R = crate::R<PsrAscmodeSpec>;
#[doc = "Register `PSR_ASCMode` writer"]
pub type W = crate::W<PsrAscmodeSpec>;
#[doc = "Transmission Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txidle {
    #[doc = "0: The transmitter line has not yet been idle."]
    Value1 = 0,
    #[doc = "1: The transmitter line has been idle and frame transmission is possible."]
    Value2 = 1,
}
impl From<Txidle> for bool {
    #[inline(always)]
    fn from(variant: Txidle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIDLE` reader - Transmission Idle"]
pub type TxidleR = crate::BitReader<Txidle>;
impl TxidleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txidle {
        match self.bits {
            false => Txidle::Value1,
            true => Txidle::Value2,
        }
    }
    #[doc = "The transmitter line has not yet been idle."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Txidle::Value1
    }
    #[doc = "The transmitter line has been idle and frame transmission is possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Txidle::Value2
    }
}
#[doc = "Field `TXIDLE` writer - Transmission Idle"]
pub type TxidleW<'a, REG> = crate::BitWriter<'a, REG, Txidle>;
impl<'a, REG> TxidleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmitter line has not yet been idle."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Txidle::Value1)
    }
    #[doc = "The transmitter line has been idle and frame transmission is possible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Txidle::Value2)
    }
}
#[doc = "Reception Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxidle {
    #[doc = "0: The receiver line has not yet been idle."]
    Value1 = 0,
    #[doc = "1: The receiver line has been idle and frame reception is possible."]
    Value2 = 1,
}
impl From<Rxidle> for bool {
    #[inline(always)]
    fn from(variant: Rxidle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIDLE` reader - Reception Idle"]
pub type RxidleR = crate::BitReader<Rxidle>;
impl RxidleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxidle {
        match self.bits {
            false => Rxidle::Value1,
            true => Rxidle::Value2,
        }
    }
    #[doc = "The receiver line has not yet been idle."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxidle::Value1
    }
    #[doc = "The receiver line has been idle and frame reception is possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxidle::Value2
    }
}
#[doc = "Field `RXIDLE` writer - Reception Idle"]
pub type RxidleW<'a, REG> = crate::BitWriter<'a, REG, Rxidle>;
impl<'a, REG> RxidleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receiver line has not yet been idle."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxidle::Value1)
    }
    #[doc = "The receiver line has been idle and frame reception is possible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxidle::Value2)
    }
}
#[doc = "Synchronization Break Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbd {
    #[doc = "0: A synchronization break has not yet been detected."]
    Value1 = 0,
    #[doc = "1: A synchronization break has been detected."]
    Value2 = 1,
}
impl From<Sbd> for bool {
    #[inline(always)]
    fn from(variant: Sbd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBD` reader - Synchronization Break Detected"]
pub type SbdR = crate::BitReader<Sbd>;
impl SbdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbd {
        match self.bits {
            false => Sbd::Value1,
            true => Sbd::Value2,
        }
    }
    #[doc = "A synchronization break has not yet been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sbd::Value1
    }
    #[doc = "A synchronization break has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sbd::Value2
    }
}
#[doc = "Field `SBD` writer - Synchronization Break Detected"]
pub type SbdW<'a, REG> = crate::BitWriter<'a, REG, Sbd>;
impl<'a, REG> SbdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A synchronization break has not yet been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbd::Value1)
    }
    #[doc = "A synchronization break has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sbd::Value2)
    }
}
#[doc = "Collision Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Col {
    #[doc = "0: A collision has not yet been detected and frame transmission is possible."]
    Value1 = 0,
    #[doc = "1: A collision has been detected and frame transmission is not possible."]
    Value2 = 1,
}
impl From<Col> for bool {
    #[inline(always)]
    fn from(variant: Col) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COL` reader - Collision Detected"]
pub type ColR = crate::BitReader<Col>;
impl ColR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Col {
        match self.bits {
            false => Col::Value1,
            true => Col::Value2,
        }
    }
    #[doc = "A collision has not yet been detected and frame transmission is possible."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Col::Value1
    }
    #[doc = "A collision has been detected and frame transmission is not possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Col::Value2
    }
}
#[doc = "Field `COL` writer - Collision Detected"]
pub type ColW<'a, REG> = crate::BitWriter<'a, REG, Col>;
impl<'a, REG> ColW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A collision has not yet been detected and frame transmission is possible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Col::Value1)
    }
    #[doc = "A collision has been detected and frame transmission is not possible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Col::Value2)
    }
}
#[doc = "Receiver Noise Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rns {
    #[doc = "0: Receiver noise has not been detected."]
    Value1 = 0,
    #[doc = "1: Receiver noise has been detected."]
    Value2 = 1,
}
impl From<Rns> for bool {
    #[inline(always)]
    fn from(variant: Rns) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNS` reader - Receiver Noise Detected"]
pub type RnsR = crate::BitReader<Rns>;
impl RnsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rns {
        match self.bits {
            false => Rns::Value1,
            true => Rns::Value2,
        }
    }
    #[doc = "Receiver noise has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rns::Value1
    }
    #[doc = "Receiver noise has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rns::Value2
    }
}
#[doc = "Field `RNS` writer - Receiver Noise Detected"]
pub type RnsW<'a, REG> = crate::BitWriter<'a, REG, Rns>;
impl<'a, REG> RnsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver noise has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rns::Value1)
    }
    #[doc = "Receiver noise has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rns::Value2)
    }
}
#[doc = "Format Error in Stop Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fer0 {
    #[doc = "0: A format error 0 has not been detected."]
    Value1 = 0,
    #[doc = "1: A format error 0 has been detected."]
    Value2 = 1,
}
impl From<Fer0> for bool {
    #[inline(always)]
    fn from(variant: Fer0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FER0` reader - Format Error in Stop Bit 0"]
pub type Fer0R = crate::BitReader<Fer0>;
impl Fer0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fer0 {
        match self.bits {
            false => Fer0::Value1,
            true => Fer0::Value2,
        }
    }
    #[doc = "A format error 0 has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fer0::Value1
    }
    #[doc = "A format error 0 has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fer0::Value2
    }
}
#[doc = "Field `FER0` writer - Format Error in Stop Bit 0"]
pub type Fer0W<'a, REG> = crate::BitWriter<'a, REG, Fer0>;
impl<'a, REG> Fer0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A format error 0 has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fer0::Value1)
    }
    #[doc = "A format error 0 has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fer0::Value2)
    }
}
#[doc = "Format Error in Stop Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fer1 {
    #[doc = "0: A format error 1 has not been detected."]
    Value1 = 0,
    #[doc = "1: A format error 1 has been detected."]
    Value2 = 1,
}
impl From<Fer1> for bool {
    #[inline(always)]
    fn from(variant: Fer1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FER1` reader - Format Error in Stop Bit 1"]
pub type Fer1R = crate::BitReader<Fer1>;
impl Fer1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fer1 {
        match self.bits {
            false => Fer1::Value1,
            true => Fer1::Value2,
        }
    }
    #[doc = "A format error 1 has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fer1::Value1
    }
    #[doc = "A format error 1 has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fer1::Value2
    }
}
#[doc = "Field `FER1` writer - Format Error in Stop Bit 1"]
pub type Fer1W<'a, REG> = crate::BitWriter<'a, REG, Fer1>;
impl<'a, REG> Fer1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A format error 1 has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fer1::Value1)
    }
    #[doc = "A format error 1 has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fer1::Value2)
    }
}
#[doc = "Receive Frame Finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rff {
    #[doc = "0: The received frame is not yet finished."]
    Value1 = 0,
    #[doc = "1: The received frame is finished."]
    Value2 = 1,
}
impl From<Rff> for bool {
    #[inline(always)]
    fn from(variant: Rff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF` reader - Receive Frame Finished"]
pub type RffR = crate::BitReader<Rff>;
impl RffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rff {
        match self.bits {
            false => Rff::Value1,
            true => Rff::Value2,
        }
    }
    #[doc = "The received frame is not yet finished."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rff::Value1
    }
    #[doc = "The received frame is finished."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rff::Value2
    }
}
#[doc = "Field `RFF` writer - Receive Frame Finished"]
pub type RffW<'a, REG> = crate::BitWriter<'a, REG, Rff>;
impl<'a, REG> RffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The received frame is not yet finished."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rff::Value1)
    }
    #[doc = "The received frame is finished."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rff::Value2)
    }
}
#[doc = "Transmitter Frame Finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tff {
    #[doc = "0: The transmitter frame is not yet finished."]
    Value1 = 0,
    #[doc = "1: The transmitter frame is finished."]
    Value2 = 1,
}
impl From<Tff> for bool {
    #[inline(always)]
    fn from(variant: Tff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFF` reader - Transmitter Frame Finished"]
pub type TffR = crate::BitReader<Tff>;
impl TffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tff {
        match self.bits {
            false => Tff::Value1,
            true => Tff::Value2,
        }
    }
    #[doc = "The transmitter frame is not yet finished."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tff::Value1
    }
    #[doc = "The transmitter frame is finished."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tff::Value2
    }
}
#[doc = "Field `TFF` writer - Transmitter Frame Finished"]
pub type TffW<'a, REG> = crate::BitWriter<'a, REG, Tff>;
impl<'a, REG> TffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmitter frame is not yet finished."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tff::Value1)
    }
    #[doc = "The transmitter frame is finished."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tff::Value2)
    }
}
#[doc = "Transfer Status BUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: A data transfer does not take place."]
    Value1 = 0,
    #[doc = "1: A data transfer currently takes place."]
    Value2 = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Transfer Status BUSY"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Value1,
            true => Busy::Value2,
        }
    }
    #[doc = "A data transfer does not take place."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Busy::Value1
    }
    #[doc = "A data transfer currently takes place."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Busy::Value2
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
    #[doc = "Bit 0 - Transmission Idle"]
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reception Idle"]
    #[inline(always)]
    pub fn rxidle(&self) -> RxidleR {
        RxidleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization Break Detected"]
    #[inline(always)]
    pub fn sbd(&self) -> SbdR {
        SbdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Collision Detected"]
    #[inline(always)]
    pub fn col(&self) -> ColR {
        ColR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver Noise Detected"]
    #[inline(always)]
    pub fn rns(&self) -> RnsR {
        RnsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Format Error in Stop Bit 0"]
    #[inline(always)]
    pub fn fer0(&self) -> Fer0R {
        Fer0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Format Error in Stop Bit 1"]
    #[inline(always)]
    pub fn fer1(&self) -> Fer1R {
        Fer1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Frame Finished"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitter Frame Finished"]
    #[inline(always)]
    pub fn tff(&self) -> TffR {
        TffR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer Status BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 0 - Transmission Idle"]
    #[inline(always)]
    #[must_use]
    pub fn txidle(&mut self) -> TxidleW<PsrAscmodeSpec> {
        TxidleW::new(self, 0)
    }
    #[doc = "Bit 1 - Reception Idle"]
    #[inline(always)]
    #[must_use]
    pub fn rxidle(&mut self) -> RxidleW<PsrAscmodeSpec> {
        RxidleW::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization Break Detected"]
    #[inline(always)]
    #[must_use]
    pub fn sbd(&mut self) -> SbdW<PsrAscmodeSpec> {
        SbdW::new(self, 2)
    }
    #[doc = "Bit 3 - Collision Detected"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> ColW<PsrAscmodeSpec> {
        ColW::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Noise Detected"]
    #[inline(always)]
    #[must_use]
    pub fn rns(&mut self) -> RnsW<PsrAscmodeSpec> {
        RnsW::new(self, 4)
    }
    #[doc = "Bit 5 - Format Error in Stop Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fer0(&mut self) -> Fer0W<PsrAscmodeSpec> {
        Fer0W::new(self, 5)
    }
    #[doc = "Bit 6 - Format Error in Stop Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn fer1(&mut self) -> Fer1W<PsrAscmodeSpec> {
        Fer1W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Frame Finished"]
    #[inline(always)]
    #[must_use]
    pub fn rff(&mut self) -> RffW<PsrAscmodeSpec> {
        RffW::new(self, 7)
    }
    #[doc = "Bit 8 - Transmitter Frame Finished"]
    #[inline(always)]
    #[must_use]
    pub fn tff(&mut self) -> TffW<PsrAscmodeSpec> {
        TffW::new(self, 8)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsif(&mut self) -> RsifW<PsrAscmodeSpec> {
        RsifW::new(self, 10)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlif(&mut self) -> DlifW<PsrAscmodeSpec> {
        DlifW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsif(&mut self) -> TsifW<PsrAscmodeSpec> {
        TsifW::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tbif(&mut self) -> TbifW<PsrAscmodeSpec> {
        TbifW::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rif(&mut self) -> RifW<PsrAscmodeSpec> {
        RifW::new(self, 14)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aif(&mut self) -> AifW<PsrAscmodeSpec> {
        AifW::new(self, 15)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn brgif(&mut self) -> BrgifW<PsrAscmodeSpec> {
        BrgifW::new(self, 16)
    }
}
#[doc = "Protocol Status Register \\[ASC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_ascmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_ascmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrAscmodeSpec;
impl crate::RegisterSpec for PsrAscmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr_ascmode::R`](R) reader structure"]
impl crate::Readable for PsrAscmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`psr_ascmode::W`](W) writer structure"]
impl crate::Writable for PsrAscmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR_ASCMode to value 0"]
impl crate::Resettable for PsrAscmodeSpec {
    const RESET_VALUE: u32 = 0;
}
