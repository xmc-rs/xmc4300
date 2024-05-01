#[doc = "Register `PSR_IISMode` reader"]
pub type R = crate::R<PsrIismodeSpec>;
#[doc = "Register `PSR_IISMode` writer"]
pub type W = crate::W<PsrIismodeSpec>;
#[doc = "Word Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wa {
    #[doc = "0: WA has been sampled 0."]
    Value1 = 0,
    #[doc = "1: WA has been sampled 1."]
    Value2 = 1,
}
impl From<Wa> for bool {
    #[inline(always)]
    fn from(variant: Wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WA` reader - Word Address"]
pub type WaR = crate::BitReader<Wa>;
impl WaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wa {
        match self.bits {
            false => Wa::Value1,
            true => Wa::Value2,
        }
    }
    #[doc = "WA has been sampled 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wa::Value1
    }
    #[doc = "WA has been sampled 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wa::Value2
    }
}
#[doc = "Field `WA` writer - Word Address"]
pub type WaW<'a, REG> = crate::BitWriter<'a, REG, Wa>;
impl<'a, REG> WaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WA has been sampled 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wa::Value1)
    }
    #[doc = "WA has been sampled 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wa::Value2)
    }
}
#[doc = "DX2S Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dx2s {
    #[doc = "0: DX2S is 0."]
    Value1 = 0,
    #[doc = "1: DX2S is 1."]
    Value2 = 1,
}
impl From<Dx2s> for bool {
    #[inline(always)]
    fn from(variant: Dx2s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DX2S` reader - DX2S Status"]
pub type Dx2sR = crate::BitReader<Dx2s>;
impl Dx2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dx2s {
        match self.bits {
            false => Dx2s::Value1,
            true => Dx2s::Value2,
        }
    }
    #[doc = "DX2S is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dx2s::Value1
    }
    #[doc = "DX2S is 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dx2s::Value2
    }
}
#[doc = "Field `DX2S` writer - DX2S Status"]
pub type Dx2sW<'a, REG> = crate::BitWriter<'a, REG, Dx2s>;
impl<'a, REG> Dx2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DX2S is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dx2s::Value1)
    }
    #[doc = "DX2S is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dx2s::Value2)
    }
}
#[doc = "DX2T Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dx2tev {
    #[doc = "0: The DX2T signal has not been activated."]
    Value1 = 0,
    #[doc = "1: The DX2T signal has been activated."]
    Value2 = 1,
}
impl From<Dx2tev> for bool {
    #[inline(always)]
    fn from(variant: Dx2tev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DX2TEV` reader - DX2T Event Detected"]
pub type Dx2tevR = crate::BitReader<Dx2tev>;
impl Dx2tevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dx2tev {
        match self.bits {
            false => Dx2tev::Value1,
            true => Dx2tev::Value2,
        }
    }
    #[doc = "The DX2T signal has not been activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dx2tev::Value1
    }
    #[doc = "The DX2T signal has been activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dx2tev::Value2
    }
}
#[doc = "Field `DX2TEV` writer - DX2T Event Detected"]
pub type Dx2tevW<'a, REG> = crate::BitWriter<'a, REG, Dx2tev>;
impl<'a, REG> Dx2tevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DX2T signal has not been activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dx2tev::Value1)
    }
    #[doc = "The DX2T signal has been activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dx2tev::Value2)
    }
}
#[doc = "WA Falling Edge Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wafe {
    #[doc = "0: A WA falling edge has not been generated."]
    Value1 = 0,
    #[doc = "1: A WA falling edge has been generated."]
    Value2 = 1,
}
impl From<Wafe> for bool {
    #[inline(always)]
    fn from(variant: Wafe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAFE` reader - WA Falling Edge Event"]
pub type WafeR = crate::BitReader<Wafe>;
impl WafeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wafe {
        match self.bits {
            false => Wafe::Value1,
            true => Wafe::Value2,
        }
    }
    #[doc = "A WA falling edge has not been generated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wafe::Value1
    }
    #[doc = "A WA falling edge has been generated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wafe::Value2
    }
}
#[doc = "Field `WAFE` writer - WA Falling Edge Event"]
pub type WafeW<'a, REG> = crate::BitWriter<'a, REG, Wafe>;
impl<'a, REG> WafeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A WA falling edge has not been generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wafe::Value1)
    }
    #[doc = "A WA falling edge has been generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wafe::Value2)
    }
}
#[doc = "WA Rising Edge Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ware {
    #[doc = "0: A WA rising edge has not been generated."]
    Value1 = 0,
    #[doc = "1: A WA rising edge has been generated."]
    Value2 = 1,
}
impl From<Ware> for bool {
    #[inline(always)]
    fn from(variant: Ware) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WARE` reader - WA Rising Edge Event"]
pub type WareR = crate::BitReader<Ware>;
impl WareR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ware {
        match self.bits {
            false => Ware::Value1,
            true => Ware::Value2,
        }
    }
    #[doc = "A WA rising edge has not been generated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ware::Value1
    }
    #[doc = "A WA rising edge has been generated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ware::Value2
    }
}
#[doc = "Field `WARE` writer - WA Rising Edge Event"]
pub type WareW<'a, REG> = crate::BitWriter<'a, REG, Ware>;
impl<'a, REG> WareW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A WA rising edge has not been generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ware::Value1)
    }
    #[doc = "A WA rising edge has been generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ware::Value2)
    }
}
#[doc = "WA Generation End\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    #[doc = "0: The WA generation has not yet ended (if it is running and WAGEN has been cleared)."]
    Value1 = 0,
    #[doc = "1: The WA generation has ended (if it has been running)."]
    Value2 = 1,
}
impl From<End> for bool {
    #[inline(always)]
    fn from(variant: End) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` reader - WA Generation End"]
pub type EndR = crate::BitReader<End>;
impl EndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> End {
        match self.bits {
            false => End::Value1,
            true => End::Value2,
        }
    }
    #[doc = "The WA generation has not yet ended (if it is running and WAGEN has been cleared)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == End::Value1
    }
    #[doc = "The WA generation has ended (if it has been running)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == End::Value2
    }
}
#[doc = "Field `END` writer - WA Generation End"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG, End>;
impl<'a, REG> EndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The WA generation has not yet ended (if it is running and WAGEN has been cleared)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(End::Value1)
    }
    #[doc = "The WA generation has ended (if it has been running)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(End::Value2)
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
    #[doc = "Bit 0 - Word Address"]
    #[inline(always)]
    pub fn wa(&self) -> WaR {
        WaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DX2S Status"]
    #[inline(always)]
    pub fn dx2s(&self) -> Dx2sR {
        Dx2sR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DX2T Event Detected"]
    #[inline(always)]
    pub fn dx2tev(&self) -> Dx2tevR {
        Dx2tevR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WA Falling Edge Event"]
    #[inline(always)]
    pub fn wafe(&self) -> WafeR {
        WafeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WA Rising Edge Event"]
    #[inline(always)]
    pub fn ware(&self) -> WareR {
        WareR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WA Generation End"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bit 0 - Word Address"]
    #[inline(always)]
    #[must_use]
    pub fn wa(&mut self) -> WaW<PsrIismodeSpec> {
        WaW::new(self, 0)
    }
    #[doc = "Bit 1 - DX2S Status"]
    #[inline(always)]
    #[must_use]
    pub fn dx2s(&mut self) -> Dx2sW<PsrIismodeSpec> {
        Dx2sW::new(self, 1)
    }
    #[doc = "Bit 3 - DX2T Event Detected"]
    #[inline(always)]
    #[must_use]
    pub fn dx2tev(&mut self) -> Dx2tevW<PsrIismodeSpec> {
        Dx2tevW::new(self, 3)
    }
    #[doc = "Bit 4 - WA Falling Edge Event"]
    #[inline(always)]
    #[must_use]
    pub fn wafe(&mut self) -> WafeW<PsrIismodeSpec> {
        WafeW::new(self, 4)
    }
    #[doc = "Bit 5 - WA Rising Edge Event"]
    #[inline(always)]
    #[must_use]
    pub fn ware(&mut self) -> WareW<PsrIismodeSpec> {
        WareW::new(self, 5)
    }
    #[doc = "Bit 6 - WA Generation End"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> EndW<PsrIismodeSpec> {
        EndW::new(self, 6)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsif(&mut self) -> RsifW<PsrIismodeSpec> {
        RsifW::new(self, 10)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlif(&mut self) -> DlifW<PsrIismodeSpec> {
        DlifW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsif(&mut self) -> TsifW<PsrIismodeSpec> {
        TsifW::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tbif(&mut self) -> TbifW<PsrIismodeSpec> {
        TbifW::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rif(&mut self) -> RifW<PsrIismodeSpec> {
        RifW::new(self, 14)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aif(&mut self) -> AifW<PsrIismodeSpec> {
        AifW::new(self, 15)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn brgif(&mut self) -> BrgifW<PsrIismodeSpec> {
        BrgifW::new(self, 16)
    }
}
#[doc = "Protocol Status Register \\[IIS Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_iismode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_iismode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrIismodeSpec;
impl crate::RegisterSpec for PsrIismodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr_iismode::R`](R) reader structure"]
impl crate::Readable for PsrIismodeSpec {}
#[doc = "`write(|w| ..)` method takes [`psr_iismode::W`](W) writer structure"]
impl crate::Writable for PsrIismodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR_IISMode to value 0"]
impl crate::Resettable for PsrIismodeSpec {
    const RESET_VALUE: u32 = 0;
}
