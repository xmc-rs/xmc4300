#[doc = "Register `PSR_ASCMode` reader"]
pub type R = crate::R<PSR_ASCMODE_SPEC>;
#[doc = "Register `PSR_ASCMode` writer"]
pub type W = crate::W<PSR_ASCMODE_SPEC>;
#[doc = "Field `TXIDLE` reader - Transmission Idle"]
pub type TXIDLE_R = crate::BitReader<TXIDLE_A>;
#[doc = "Transmission Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TXIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXIDLE_A {
        match self.bits {
            false => TXIDLE_A::VALUE1,
            true => TXIDLE_A::VALUE2,
        }
    }
    #[doc = "The transmitter line has not yet been idle."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXIDLE_A::VALUE1
    }
    #[doc = "The transmitter line has been idle and frame transmission is possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXIDLE_A::VALUE2
    }
}
#[doc = "Field `TXIDLE` writer - Transmission Idle"]
pub type TXIDLE_W<'a, REG> = crate::BitWriter<'a, REG, TXIDLE_A>;
impl<'a, REG> TXIDLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmitter line has not yet been idle."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TXIDLE_A::VALUE1)
    }
    #[doc = "The transmitter line has been idle and frame transmission is possible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TXIDLE_A::VALUE2)
    }
}
#[doc = "Field `RXIDLE` reader - Reception Idle"]
pub type RXIDLE_R = crate::BitReader<RXIDLE_A>;
#[doc = "Reception Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RXIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXIDLE_A {
        match self.bits {
            false => RXIDLE_A::VALUE1,
            true => RXIDLE_A::VALUE2,
        }
    }
    #[doc = "The receiver line has not yet been idle."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXIDLE_A::VALUE1
    }
    #[doc = "The receiver line has been idle and frame reception is possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXIDLE_A::VALUE2
    }
}
#[doc = "Field `RXIDLE` writer - Reception Idle"]
pub type RXIDLE_W<'a, REG> = crate::BitWriter<'a, REG, RXIDLE_A>;
impl<'a, REG> RXIDLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receiver line has not yet been idle."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RXIDLE_A::VALUE1)
    }
    #[doc = "The receiver line has been idle and frame reception is possible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RXIDLE_A::VALUE2)
    }
}
#[doc = "Field `SBD` reader - Synchronization Break Detected"]
pub type SBD_R = crate::BitReader<SBD_A>;
#[doc = "Synchronization Break Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SBD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBD_A {
        match self.bits {
            false => SBD_A::VALUE1,
            true => SBD_A::VALUE2,
        }
    }
    #[doc = "A synchronization break has not yet been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SBD_A::VALUE1
    }
    #[doc = "A synchronization break has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SBD_A::VALUE2
    }
}
#[doc = "Field `SBD` writer - Synchronization Break Detected"]
pub type SBD_W<'a, REG> = crate::BitWriter<'a, REG, SBD_A>;
impl<'a, REG> SBD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A synchronization break has not yet been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SBD_A::VALUE1)
    }
    #[doc = "A synchronization break has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SBD_A::VALUE2)
    }
}
#[doc = "Field `COL` reader - Collision Detected"]
pub type COL_R = crate::BitReader<COL_A>;
#[doc = "Collision Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl COL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COL_A {
        match self.bits {
            false => COL_A::VALUE1,
            true => COL_A::VALUE2,
        }
    }
    #[doc = "A collision has not yet been detected and frame transmission is possible."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COL_A::VALUE1
    }
    #[doc = "A collision has been detected and frame transmission is not possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COL_A::VALUE2
    }
}
#[doc = "Field `COL` writer - Collision Detected"]
pub type COL_W<'a, REG> = crate::BitWriter<'a, REG, COL_A>;
impl<'a, REG> COL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A collision has not yet been detected and frame transmission is possible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(COL_A::VALUE1)
    }
    #[doc = "A collision has been detected and frame transmission is not possible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(COL_A::VALUE2)
    }
}
#[doc = "Field `RNS` reader - Receiver Noise Detected"]
pub type RNS_R = crate::BitReader<RNS_A>;
#[doc = "Receiver Noise Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RNS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNS_A {
        match self.bits {
            false => RNS_A::VALUE1,
            true => RNS_A::VALUE2,
        }
    }
    #[doc = "Receiver noise has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RNS_A::VALUE1
    }
    #[doc = "Receiver noise has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RNS_A::VALUE2
    }
}
#[doc = "Field `RNS` writer - Receiver Noise Detected"]
pub type RNS_W<'a, REG> = crate::BitWriter<'a, REG, RNS_A>;
impl<'a, REG> RNS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver noise has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RNS_A::VALUE1)
    }
    #[doc = "Receiver noise has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RNS_A::VALUE2)
    }
}
#[doc = "Field `FER0` reader - Format Error in Stop Bit 0"]
pub type FER0_R = crate::BitReader<FER0_A>;
#[doc = "Format Error in Stop Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl FER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FER0_A {
        match self.bits {
            false => FER0_A::VALUE1,
            true => FER0_A::VALUE2,
        }
    }
    #[doc = "A format error 0 has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FER0_A::VALUE1
    }
    #[doc = "A format error 0 has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FER0_A::VALUE2
    }
}
#[doc = "Field `FER0` writer - Format Error in Stop Bit 0"]
pub type FER0_W<'a, REG> = crate::BitWriter<'a, REG, FER0_A>;
impl<'a, REG> FER0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A format error 0 has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FER0_A::VALUE1)
    }
    #[doc = "A format error 0 has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FER0_A::VALUE2)
    }
}
#[doc = "Field `FER1` reader - Format Error in Stop Bit 1"]
pub type FER1_R = crate::BitReader<FER1_A>;
#[doc = "Format Error in Stop Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl FER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FER1_A {
        match self.bits {
            false => FER1_A::VALUE1,
            true => FER1_A::VALUE2,
        }
    }
    #[doc = "A format error 1 has not been detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FER1_A::VALUE1
    }
    #[doc = "A format error 1 has been detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FER1_A::VALUE2
    }
}
#[doc = "Field `FER1` writer - Format Error in Stop Bit 1"]
pub type FER1_W<'a, REG> = crate::BitWriter<'a, REG, FER1_A>;
impl<'a, REG> FER1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A format error 1 has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FER1_A::VALUE1)
    }
    #[doc = "A format error 1 has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FER1_A::VALUE2)
    }
}
#[doc = "Field `RFF` reader - Receive Frame Finished"]
pub type RFF_R = crate::BitReader<RFF_A>;
#[doc = "Receive Frame Finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFF_A {
        match self.bits {
            false => RFF_A::VALUE1,
            true => RFF_A::VALUE2,
        }
    }
    #[doc = "The received frame is not yet finished."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RFF_A::VALUE1
    }
    #[doc = "The received frame is finished."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RFF_A::VALUE2
    }
}
#[doc = "Field `RFF` writer - Receive Frame Finished"]
pub type RFF_W<'a, REG> = crate::BitWriter<'a, REG, RFF_A>;
impl<'a, REG> RFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The received frame is not yet finished."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RFF_A::VALUE1)
    }
    #[doc = "The received frame is finished."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RFF_A::VALUE2)
    }
}
#[doc = "Field `TFF` reader - Transmitter Frame Finished"]
pub type TFF_R = crate::BitReader<TFF_A>;
#[doc = "Transmitter Frame Finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFF_A {
        match self.bits {
            false => TFF_A::VALUE1,
            true => TFF_A::VALUE2,
        }
    }
    #[doc = "The transmitter frame is not yet finished."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TFF_A::VALUE1
    }
    #[doc = "The transmitter frame is finished."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TFF_A::VALUE2
    }
}
#[doc = "Field `TFF` writer - Transmitter Frame Finished"]
pub type TFF_W<'a, REG> = crate::BitWriter<'a, REG, TFF_A>;
impl<'a, REG> TFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmitter frame is not yet finished."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TFF_A::VALUE1)
    }
    #[doc = "The transmitter frame is finished."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TFF_A::VALUE2)
    }
}
#[doc = "Field `BUSY` reader - Transfer Status BUSY"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Transfer Status BUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "A data transfer does not take place."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUSY_A::VALUE1
    }
    #[doc = "A data transfer currently takes place."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
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
    pub const fn variant(&self) -> RSIF_A {
        match self.bits {
            false => RSIF_A::VALUE1,
            true => RSIF_A::VALUE2,
        }
    }
    #[doc = "A receiver start event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSIF_A::VALUE1
    }
    #[doc = "A receiver start event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSIF_A::VALUE2
    }
}
#[doc = "Field `RSIF` writer - Receiver Start Indication Flag"]
pub type RSIF_W<'a, REG> = crate::BitWriter<'a, REG, RSIF_A>;
impl<'a, REG> RSIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A receiver start event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RSIF_A::VALUE1)
    }
    #[doc = "A receiver start event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> DLIF_A {
        match self.bits {
            false => DLIF_A::VALUE1,
            true => DLIF_A::VALUE2,
        }
    }
    #[doc = "A data lost event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DLIF_A::VALUE1
    }
    #[doc = "A data lost event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DLIF_A::VALUE2
    }
}
#[doc = "Field `DLIF` writer - Data Lost Indication Flag"]
pub type DLIF_W<'a, REG> = crate::BitWriter<'a, REG, DLIF_A>;
impl<'a, REG> DLIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A data lost event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DLIF_A::VALUE1)
    }
    #[doc = "A data lost event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> TSIF_A {
        match self.bits {
            false => TSIF_A::VALUE1,
            true => TSIF_A::VALUE2,
        }
    }
    #[doc = "A transmit shift event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSIF_A::VALUE1
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSIF_A::VALUE2
    }
}
#[doc = "Field `TSIF` writer - Transmit Shift Indication Flag"]
pub type TSIF_W<'a, REG> = crate::BitWriter<'a, REG, TSIF_A>;
impl<'a, REG> TSIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A transmit shift event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSIF_A::VALUE1)
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> TBIF_A {
        match self.bits {
            false => TBIF_A::VALUE1,
            true => TBIF_A::VALUE2,
        }
    }
    #[doc = "A transmit buffer event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TBIF_A::VALUE1
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBIF_A::VALUE2
    }
}
#[doc = "Field `TBIF` writer - Transmit Buffer Indication Flag"]
pub type TBIF_W<'a, REG> = crate::BitWriter<'a, REG, TBIF_A>;
impl<'a, REG> TBIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A transmit buffer event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TBIF_A::VALUE1)
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> RIF_A {
        match self.bits {
            false => RIF_A::VALUE1,
            true => RIF_A::VALUE2,
        }
    }
    #[doc = "A receive event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RIF_A::VALUE1
    }
    #[doc = "A receive event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RIF_A::VALUE2
    }
}
#[doc = "Field `RIF` writer - Receive Indication Flag"]
pub type RIF_W<'a, REG> = crate::BitWriter<'a, REG, RIF_A>;
impl<'a, REG> RIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RIF_A::VALUE1)
    }
    #[doc = "A receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> AIF_A {
        match self.bits {
            false => AIF_A::VALUE1,
            true => AIF_A::VALUE2,
        }
    }
    #[doc = "An alternative receive event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AIF_A::VALUE1
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AIF_A::VALUE2
    }
}
#[doc = "Field `AIF` writer - Alternative Receive Indication Flag"]
pub type AIF_W<'a, REG> = crate::BitWriter<'a, REG, AIF_A>;
impl<'a, REG> AIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An alternative receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AIF_A::VALUE1)
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> BRGIF_A {
        match self.bits {
            false => BRGIF_A::VALUE1,
            true => BRGIF_A::VALUE2,
        }
    }
    #[doc = "A baud rate generator event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BRGIF_A::VALUE1
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BRGIF_A::VALUE2
    }
}
#[doc = "Field `BRGIF` writer - Baud Rate Generator Indication Flag"]
pub type BRGIF_W<'a, REG> = crate::BitWriter<'a, REG, BRGIF_A>;
impl<'a, REG> BRGIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A baud rate generator event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BRGIF_A::VALUE1)
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BRGIF_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Transmission Idle"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reception Idle"]
    #[inline(always)]
    pub fn rxidle(&self) -> RXIDLE_R {
        RXIDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization Break Detected"]
    #[inline(always)]
    pub fn sbd(&self) -> SBD_R {
        SBD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Collision Detected"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver Noise Detected"]
    #[inline(always)]
    pub fn rns(&self) -> RNS_R {
        RNS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Format Error in Stop Bit 0"]
    #[inline(always)]
    pub fn fer0(&self) -> FER0_R {
        FER0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Format Error in Stop Bit 1"]
    #[inline(always)]
    pub fn fer1(&self) -> FER1_R {
        FER1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Frame Finished"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitter Frame Finished"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer Status BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 0 - Transmission Idle"]
    #[inline(always)]
    #[must_use]
    pub fn txidle(&mut self) -> TXIDLE_W<PSR_ASCMODE_SPEC> {
        TXIDLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reception Idle"]
    #[inline(always)]
    #[must_use]
    pub fn rxidle(&mut self) -> RXIDLE_W<PSR_ASCMODE_SPEC> {
        RXIDLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization Break Detected"]
    #[inline(always)]
    #[must_use]
    pub fn sbd(&mut self) -> SBD_W<PSR_ASCMODE_SPEC> {
        SBD_W::new(self, 2)
    }
    #[doc = "Bit 3 - Collision Detected"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> COL_W<PSR_ASCMODE_SPEC> {
        COL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Noise Detected"]
    #[inline(always)]
    #[must_use]
    pub fn rns(&mut self) -> RNS_W<PSR_ASCMODE_SPEC> {
        RNS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Format Error in Stop Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fer0(&mut self) -> FER0_W<PSR_ASCMODE_SPEC> {
        FER0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Format Error in Stop Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn fer1(&mut self) -> FER1_W<PSR_ASCMODE_SPEC> {
        FER1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Frame Finished"]
    #[inline(always)]
    #[must_use]
    pub fn rff(&mut self) -> RFF_W<PSR_ASCMODE_SPEC> {
        RFF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Transmitter Frame Finished"]
    #[inline(always)]
    #[must_use]
    pub fn tff(&mut self) -> TFF_W<PSR_ASCMODE_SPEC> {
        TFF_W::new(self, 8)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsif(&mut self) -> RSIF_W<PSR_ASCMODE_SPEC> {
        RSIF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlif(&mut self) -> DLIF_W<PSR_ASCMODE_SPEC> {
        DLIF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsif(&mut self) -> TSIF_W<PSR_ASCMODE_SPEC> {
        TSIF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tbif(&mut self) -> TBIF_W<PSR_ASCMODE_SPEC> {
        TBIF_W::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rif(&mut self) -> RIF_W<PSR_ASCMODE_SPEC> {
        RIF_W::new(self, 14)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aif(&mut self) -> AIF_W<PSR_ASCMODE_SPEC> {
        AIF_W::new(self, 15)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn brgif(&mut self) -> BRGIF_W<PSR_ASCMODE_SPEC> {
        BRGIF_W::new(self, 16)
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
#[doc = "Protocol Status Register \\[ASC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_ascmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_ascmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSR_ASCMODE_SPEC;
impl crate::RegisterSpec for PSR_ASCMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr_ascmode::R`](R) reader structure"]
impl crate::Readable for PSR_ASCMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psr_ascmode::W`](W) writer structure"]
impl crate::Writable for PSR_ASCMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR_ASCMode to value 0"]
impl crate::Resettable for PSR_ASCMODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
