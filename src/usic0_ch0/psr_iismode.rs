#[doc = "Register `PSR_IISMode` reader"]
pub type R = crate::R<PSR_IISMODE_SPEC>;
#[doc = "Register `PSR_IISMode` writer"]
pub type W = crate::W<PSR_IISMODE_SPEC>;
#[doc = "Field `WA` reader - Word Address"]
pub type WA_R = crate::BitReader<WA_A>;
#[doc = "Word Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WA_A {
    #[doc = "0: WA has been sampled 0."]
    VALUE1 = 0,
    #[doc = "1: WA has been sampled 1."]
    VALUE2 = 1,
}
impl From<WA_A> for bool {
    #[inline(always)]
    fn from(variant: WA_A) -> Self {
        variant as u8 != 0
    }
}
impl WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WA_A {
        match self.bits {
            false => WA_A::VALUE1,
            true => WA_A::VALUE2,
        }
    }
    #[doc = "WA has been sampled 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WA_A::VALUE1
    }
    #[doc = "WA has been sampled 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WA_A::VALUE2
    }
}
#[doc = "Field `WA` writer - Word Address"]
pub type WA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WA_A>;
impl<'a, REG, const O: u8> WA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WA has been sampled 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WA_A::VALUE1)
    }
    #[doc = "WA has been sampled 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WA_A::VALUE2)
    }
}
#[doc = "Field `DX2S` reader - DX2S Status"]
pub type DX2S_R = crate::BitReader<DX2S_A>;
#[doc = "DX2S Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DX2S_A {
    #[doc = "0: DX2S is 0."]
    VALUE1 = 0,
    #[doc = "1: DX2S is 1."]
    VALUE2 = 1,
}
impl From<DX2S_A> for bool {
    #[inline(always)]
    fn from(variant: DX2S_A) -> Self {
        variant as u8 != 0
    }
}
impl DX2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DX2S_A {
        match self.bits {
            false => DX2S_A::VALUE1,
            true => DX2S_A::VALUE2,
        }
    }
    #[doc = "DX2S is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DX2S_A::VALUE1
    }
    #[doc = "DX2S is 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DX2S_A::VALUE2
    }
}
#[doc = "Field `DX2S` writer - DX2S Status"]
pub type DX2S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DX2S_A>;
impl<'a, REG, const O: u8> DX2S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DX2S is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DX2S_A::VALUE1)
    }
    #[doc = "DX2S is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DX2S_A::VALUE2)
    }
}
#[doc = "Field `DX2TEV` reader - DX2T Event Detected"]
pub type DX2TEV_R = crate::BitReader<DX2TEV_A>;
#[doc = "DX2T Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DX2TEV_A {
    #[doc = "0: The DX2T signal has not been activated."]
    VALUE1 = 0,
    #[doc = "1: The DX2T signal has been activated."]
    VALUE2 = 1,
}
impl From<DX2TEV_A> for bool {
    #[inline(always)]
    fn from(variant: DX2TEV_A) -> Self {
        variant as u8 != 0
    }
}
impl DX2TEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DX2TEV_A {
        match self.bits {
            false => DX2TEV_A::VALUE1,
            true => DX2TEV_A::VALUE2,
        }
    }
    #[doc = "The DX2T signal has not been activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DX2TEV_A::VALUE1
    }
    #[doc = "The DX2T signal has been activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DX2TEV_A::VALUE2
    }
}
#[doc = "Field `DX2TEV` writer - DX2T Event Detected"]
pub type DX2TEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DX2TEV_A>;
impl<'a, REG, const O: u8> DX2TEV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DX2T signal has not been activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DX2TEV_A::VALUE1)
    }
    #[doc = "The DX2T signal has been activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DX2TEV_A::VALUE2)
    }
}
#[doc = "Field `WAFE` reader - WA Falling Edge Event"]
pub type WAFE_R = crate::BitReader<WAFE_A>;
#[doc = "WA Falling Edge Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAFE_A {
    #[doc = "0: A WA falling edge has not been generated."]
    VALUE1 = 0,
    #[doc = "1: A WA falling edge has been generated."]
    VALUE2 = 1,
}
impl From<WAFE_A> for bool {
    #[inline(always)]
    fn from(variant: WAFE_A) -> Self {
        variant as u8 != 0
    }
}
impl WAFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAFE_A {
        match self.bits {
            false => WAFE_A::VALUE1,
            true => WAFE_A::VALUE2,
        }
    }
    #[doc = "A WA falling edge has not been generated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAFE_A::VALUE1
    }
    #[doc = "A WA falling edge has been generated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAFE_A::VALUE2
    }
}
#[doc = "Field `WAFE` writer - WA Falling Edge Event"]
pub type WAFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WAFE_A>;
impl<'a, REG, const O: u8> WAFE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A WA falling edge has not been generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WAFE_A::VALUE1)
    }
    #[doc = "A WA falling edge has been generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WAFE_A::VALUE2)
    }
}
#[doc = "Field `WARE` reader - WA Rising Edge Event"]
pub type WARE_R = crate::BitReader<WARE_A>;
#[doc = "WA Rising Edge Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WARE_A {
    #[doc = "0: A WA rising edge has not been generated."]
    VALUE1 = 0,
    #[doc = "1: A WA rising edge has been generated."]
    VALUE2 = 1,
}
impl From<WARE_A> for bool {
    #[inline(always)]
    fn from(variant: WARE_A) -> Self {
        variant as u8 != 0
    }
}
impl WARE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WARE_A {
        match self.bits {
            false => WARE_A::VALUE1,
            true => WARE_A::VALUE2,
        }
    }
    #[doc = "A WA rising edge has not been generated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WARE_A::VALUE1
    }
    #[doc = "A WA rising edge has been generated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WARE_A::VALUE2
    }
}
#[doc = "Field `WARE` writer - WA Rising Edge Event"]
pub type WARE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WARE_A>;
impl<'a, REG, const O: u8> WARE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A WA rising edge has not been generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WARE_A::VALUE1)
    }
    #[doc = "A WA rising edge has been generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WARE_A::VALUE2)
    }
}
#[doc = "Field `END` reader - WA Generation End"]
pub type END_R = crate::BitReader<END_A>;
#[doc = "WA Generation End\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum END_A {
    #[doc = "0: The WA generation has not yet ended (if it is running and WAGEN has been cleared)."]
    VALUE1 = 0,
    #[doc = "1: The WA generation has ended (if it has been running)."]
    VALUE2 = 1,
}
impl From<END_A> for bool {
    #[inline(always)]
    fn from(variant: END_A) -> Self {
        variant as u8 != 0
    }
}
impl END_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> END_A {
        match self.bits {
            false => END_A::VALUE1,
            true => END_A::VALUE2,
        }
    }
    #[doc = "The WA generation has not yet ended (if it is running and WAGEN has been cleared)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == END_A::VALUE1
    }
    #[doc = "The WA generation has ended (if it has been running)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == END_A::VALUE2
    }
}
#[doc = "Field `END` writer - WA Generation End"]
pub type END_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, END_A>;
impl<'a, REG, const O: u8> END_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The WA generation has not yet ended (if it is running and WAGEN has been cleared)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(END_A::VALUE1)
    }
    #[doc = "The WA generation has ended (if it has been running)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(END_A::VALUE2)
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
pub type RSIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RSIF_A>;
impl<'a, REG, const O: u8> RSIF_W<'a, REG, O>
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
pub type DLIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DLIF_A>;
impl<'a, REG, const O: u8> DLIF_W<'a, REG, O>
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
pub type TSIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TSIF_A>;
impl<'a, REG, const O: u8> TSIF_W<'a, REG, O>
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
pub type TBIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TBIF_A>;
impl<'a, REG, const O: u8> TBIF_W<'a, REG, O>
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
pub type RIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RIF_A>;
impl<'a, REG, const O: u8> RIF_W<'a, REG, O>
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
pub type AIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AIF_A>;
impl<'a, REG, const O: u8> AIF_W<'a, REG, O>
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
pub type BRGIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BRGIF_A>;
impl<'a, REG, const O: u8> BRGIF_W<'a, REG, O>
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
    #[doc = "Bit 0 - Word Address"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DX2S Status"]
    #[inline(always)]
    pub fn dx2s(&self) -> DX2S_R {
        DX2S_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DX2T Event Detected"]
    #[inline(always)]
    pub fn dx2tev(&self) -> DX2TEV_R {
        DX2TEV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WA Falling Edge Event"]
    #[inline(always)]
    pub fn wafe(&self) -> WAFE_R {
        WAFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WA Rising Edge Event"]
    #[inline(always)]
    pub fn ware(&self) -> WARE_R {
        WARE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WA Generation End"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bit 0 - Word Address"]
    #[inline(always)]
    #[must_use]
    pub fn wa(&mut self) -> WA_W<PSR_IISMODE_SPEC, 0> {
        WA_W::new(self)
    }
    #[doc = "Bit 1 - DX2S Status"]
    #[inline(always)]
    #[must_use]
    pub fn dx2s(&mut self) -> DX2S_W<PSR_IISMODE_SPEC, 1> {
        DX2S_W::new(self)
    }
    #[doc = "Bit 3 - DX2T Event Detected"]
    #[inline(always)]
    #[must_use]
    pub fn dx2tev(&mut self) -> DX2TEV_W<PSR_IISMODE_SPEC, 3> {
        DX2TEV_W::new(self)
    }
    #[doc = "Bit 4 - WA Falling Edge Event"]
    #[inline(always)]
    #[must_use]
    pub fn wafe(&mut self) -> WAFE_W<PSR_IISMODE_SPEC, 4> {
        WAFE_W::new(self)
    }
    #[doc = "Bit 5 - WA Rising Edge Event"]
    #[inline(always)]
    #[must_use]
    pub fn ware(&mut self) -> WARE_W<PSR_IISMODE_SPEC, 5> {
        WARE_W::new(self)
    }
    #[doc = "Bit 6 - WA Generation End"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> END_W<PSR_IISMODE_SPEC, 6> {
        END_W::new(self)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsif(&mut self) -> RSIF_W<PSR_IISMODE_SPEC, 10> {
        RSIF_W::new(self)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlif(&mut self) -> DLIF_W<PSR_IISMODE_SPEC, 11> {
        DLIF_W::new(self)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsif(&mut self) -> TSIF_W<PSR_IISMODE_SPEC, 12> {
        TSIF_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tbif(&mut self) -> TBIF_W<PSR_IISMODE_SPEC, 13> {
        TBIF_W::new(self)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rif(&mut self) -> RIF_W<PSR_IISMODE_SPEC, 14> {
        RIF_W::new(self)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aif(&mut self) -> AIF_W<PSR_IISMODE_SPEC, 15> {
        AIF_W::new(self)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn brgif(&mut self) -> BRGIF_W<PSR_IISMODE_SPEC, 16> {
        BRGIF_W::new(self)
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
#[doc = "Protocol Status Register \\[IIS Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_iismode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_iismode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSR_IISMODE_SPEC;
impl crate::RegisterSpec for PSR_IISMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr_iismode::R`](R) reader structure"]
impl crate::Readable for PSR_IISMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psr_iismode::W`](W) writer structure"]
impl crate::Writable for PSR_IISMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSR_IISMode to value 0"]
impl crate::Resettable for PSR_IISMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
