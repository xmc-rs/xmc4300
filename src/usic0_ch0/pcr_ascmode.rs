#[doc = "Register `PCR_ASCMode` reader"]
pub type R = crate::R<PcrAscmodeSpec>;
#[doc = "Register `PCR_ASCMode` writer"]
pub type W = crate::W<PcrAscmodeSpec>;
#[doc = "Sample Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smd {
    #[doc = "0: Only one sample is taken per bit time. The current input value is sampled."]
    Value1 = 0,
    #[doc = "1: Three samples are taken per bit time and a majority decision is made."]
    Value2 = 1,
}
impl From<Smd> for bool {
    #[inline(always)]
    fn from(variant: Smd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMD` reader - Sample Mode"]
pub type SmdR = crate::BitReader<Smd>;
impl SmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smd {
        match self.bits {
            false => Smd::Value1,
            true => Smd::Value2,
        }
    }
    #[doc = "Only one sample is taken per bit time. The current input value is sampled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smd::Value1
    }
    #[doc = "Three samples are taken per bit time and a majority decision is made."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smd::Value2
    }
}
#[doc = "Field `SMD` writer - Sample Mode"]
pub type SmdW<'a, REG> = crate::BitWriter<'a, REG, Smd>;
impl<'a, REG> SmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only one sample is taken per bit time. The current input value is sampled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smd::Value1)
    }
    #[doc = "Three samples are taken per bit time and a majority decision is made."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smd::Value2)
    }
}
#[doc = "Stop Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpb {
    #[doc = "0: The number of stop bits is 1."]
    Value1 = 0,
    #[doc = "1: The number of stop bits is 2."]
    Value2 = 1,
}
impl From<Stpb> for bool {
    #[inline(always)]
    fn from(variant: Stpb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPB` reader - Stop Bits"]
pub type StpbR = crate::BitReader<Stpb>;
impl StpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stpb {
        match self.bits {
            false => Stpb::Value1,
            true => Stpb::Value2,
        }
    }
    #[doc = "The number of stop bits is 1."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stpb::Value1
    }
    #[doc = "The number of stop bits is 2."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stpb::Value2
    }
}
#[doc = "Field `STPB` writer - Stop Bits"]
pub type StpbW<'a, REG> = crate::BitWriter<'a, REG, Stpb>;
impl<'a, REG> StpbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of stop bits is 1."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stpb::Value1)
    }
    #[doc = "The number of stop bits is 2."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stpb::Value2)
    }
}
#[doc = "Idle Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idm {
    #[doc = "0: The bus idle detection is switched off and bits PSR.TXIDLE and PSR.RXIDLE are set automatically to enable data transfers without checking the inputs before."]
    Value1 = 0,
    #[doc = "1: The bus is considered as idle after a number of consecutive passive bit times defined by SCTR.FLE plus 2 (in the case without parity bit) or plus 3 (in the case with parity bit)."]
    Value2 = 1,
}
impl From<Idm> for bool {
    #[inline(always)]
    fn from(variant: Idm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDM` reader - Idle Detection Mode"]
pub type IdmR = crate::BitReader<Idm>;
impl IdmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idm {
        match self.bits {
            false => Idm::Value1,
            true => Idm::Value2,
        }
    }
    #[doc = "The bus idle detection is switched off and bits PSR.TXIDLE and PSR.RXIDLE are set automatically to enable data transfers without checking the inputs before."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Idm::Value1
    }
    #[doc = "The bus is considered as idle after a number of consecutive passive bit times defined by SCTR.FLE plus 2 (in the case without parity bit) or plus 3 (in the case with parity bit)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Idm::Value2
    }
}
#[doc = "Field `IDM` writer - Idle Detection Mode"]
pub type IdmW<'a, REG> = crate::BitWriter<'a, REG, Idm>;
impl<'a, REG> IdmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The bus idle detection is switched off and bits PSR.TXIDLE and PSR.RXIDLE are set automatically to enable data transfers without checking the inputs before."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Idm::Value1)
    }
    #[doc = "The bus is considered as idle after a number of consecutive passive bit times defined by SCTR.FLE plus 2 (in the case without parity bit) or plus 3 (in the case with parity bit)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Idm::Value2)
    }
}
#[doc = "Synchronization Break Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbien {
    #[doc = "0: The interrupt generation is disabled."]
    Value1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    Value2 = 1,
}
impl From<Sbien> for bool {
    #[inline(always)]
    fn from(variant: Sbien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBIEN` reader - Synchronization Break Interrupt Enable"]
pub type SbienR = crate::BitReader<Sbien>;
impl SbienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbien {
        match self.bits {
            false => Sbien::Value1,
            true => Sbien::Value2,
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sbien::Value1
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sbien::Value2
    }
}
#[doc = "Field `SBIEN` writer - Synchronization Break Interrupt Enable"]
pub type SbienW<'a, REG> = crate::BitWriter<'a, REG, Sbien>;
impl<'a, REG> SbienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbien::Value1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sbien::Value2)
    }
}
#[doc = "Collision Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cden {
    #[doc = "0: The collision detection is disabled."]
    Value1 = 0,
    #[doc = "1: If a collision is detected, the transmitter stops its data transmission, outputs a 1, sets bit PSR.COL and generates a protocol interrupt. In order to allow data transmission again, PSR.COL has to be cleared by software."]
    Value2 = 1,
}
impl From<Cden> for bool {
    #[inline(always)]
    fn from(variant: Cden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDEN` reader - Collision Detection Enable"]
pub type CdenR = crate::BitReader<Cden>;
impl CdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cden {
        match self.bits {
            false => Cden::Value1,
            true => Cden::Value2,
        }
    }
    #[doc = "The collision detection is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cden::Value1
    }
    #[doc = "If a collision is detected, the transmitter stops its data transmission, outputs a 1, sets bit PSR.COL and generates a protocol interrupt. In order to allow data transmission again, PSR.COL has to be cleared by software."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cden::Value2
    }
}
#[doc = "Field `CDEN` writer - Collision Detection Enable"]
pub type CdenW<'a, REG> = crate::BitWriter<'a, REG, Cden>;
impl<'a, REG> CdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The collision detection is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cden::Value1)
    }
    #[doc = "If a collision is detected, the transmitter stops its data transmission, outputs a 1, sets bit PSR.COL and generates a protocol interrupt. In order to allow data transmission again, PSR.COL has to be cleared by software."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cden::Value2)
    }
}
#[doc = "Receiver Noise Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rnien {
    #[doc = "0: The interrupt generation is disabled."]
    Value1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    Value2 = 1,
}
impl From<Rnien> for bool {
    #[inline(always)]
    fn from(variant: Rnien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNIEN` reader - Receiver Noise Detection Interrupt Enable"]
pub type RnienR = crate::BitReader<Rnien>;
impl RnienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rnien {
        match self.bits {
            false => Rnien::Value1,
            true => Rnien::Value2,
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rnien::Value1
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rnien::Value2
    }
}
#[doc = "Field `RNIEN` writer - Receiver Noise Detection Interrupt Enable"]
pub type RnienW<'a, REG> = crate::BitWriter<'a, REG, Rnien>;
impl<'a, REG> RnienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rnien::Value1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rnien::Value2)
    }
}
#[doc = "Format Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feien {
    #[doc = "0: The interrupt generation is disabled."]
    Value1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    Value2 = 1,
}
impl From<Feien> for bool {
    #[inline(always)]
    fn from(variant: Feien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIEN` reader - Format Error Interrupt Enable"]
pub type FeienR = crate::BitReader<Feien>;
impl FeienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Feien {
        match self.bits {
            false => Feien::Value1,
            true => Feien::Value2,
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Feien::Value1
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Feien::Value2
    }
}
#[doc = "Field `FEIEN` writer - Format Error Interrupt Enable"]
pub type FeienW<'a, REG> = crate::BitWriter<'a, REG, Feien>;
impl<'a, REG> FeienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Feien::Value1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Feien::Value2)
    }
}
#[doc = "Frame Finished Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ffien {
    #[doc = "0: The interrupt generation is disabled."]
    Value1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    Value2 = 1,
}
impl From<Ffien> for bool {
    #[inline(always)]
    fn from(variant: Ffien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFIEN` reader - Frame Finished Interrupt Enable"]
pub type FfienR = crate::BitReader<Ffien>;
impl FfienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ffien {
        match self.bits {
            false => Ffien::Value1,
            true => Ffien::Value2,
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ffien::Value1
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ffien::Value2
    }
}
#[doc = "Field `FFIEN` writer - Frame Finished Interrupt Enable"]
pub type FfienW<'a, REG> = crate::BitWriter<'a, REG, Ffien>;
impl<'a, REG> FfienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ffien::Value1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ffien::Value2)
    }
}
#[doc = "Field `SP` reader - Sample Point"]
pub type SpR = crate::FieldReader;
#[doc = "Field `SP` writer - Sample Point"]
pub type SpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Pulse Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pl {
    #[doc = "0: The pulse length is equal to the bit length (no shortened 0)."]
    Value1 = 0,
    #[doc = "1: The pulse length of a 0 bit is 2 time quanta."]
    Value2 = 1,
    #[doc = "2: The pulse length of a 0 bit is 3 time quanta."]
    Value3 = 2,
    #[doc = "7: The pulse length of a 0 bit is 8 time quanta."]
    Value4 = 7,
}
impl From<Pl> for u8 {
    #[inline(always)]
    fn from(variant: Pl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pl {
    type Ux = u8;
}
#[doc = "Field `PL` reader - Pulse Length"]
pub type PlR = crate::FieldReader<Pl>;
impl PlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pl> {
        match self.bits {
            0 => Some(Pl::Value1),
            1 => Some(Pl::Value2),
            2 => Some(Pl::Value3),
            7 => Some(Pl::Value4),
            _ => None,
        }
    }
    #[doc = "The pulse length is equal to the bit length (no shortened 0)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pl::Value1
    }
    #[doc = "The pulse length of a 0 bit is 2 time quanta."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pl::Value2
    }
    #[doc = "The pulse length of a 0 bit is 3 time quanta."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pl::Value3
    }
    #[doc = "The pulse length of a 0 bit is 8 time quanta."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pl::Value4
    }
}
#[doc = "Field `PL` writer - Pulse Length"]
pub type PlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pl>;
impl<'a, REG> PlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The pulse length is equal to the bit length (no shortened 0)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pl::Value1)
    }
    #[doc = "The pulse length of a 0 bit is 2 time quanta."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pl::Value2)
    }
    #[doc = "The pulse length of a 0 bit is 3 time quanta."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pl::Value3)
    }
    #[doc = "The pulse length of a 0 bit is 8 time quanta."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pl::Value4)
    }
}
#[doc = "Receiver Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsten {
    #[doc = "0: Flag PSR\\[9\\]
is not modified depending on the receiver status."]
    Value1 = 0,
    #[doc = "1: Flag PSR\\[9\\]
is set during the complete reception of a frame."]
    Value2 = 1,
}
impl From<Rsten> for bool {
    #[inline(always)]
    fn from(variant: Rsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTEN` reader - Receiver Status Enable"]
pub type RstenR = crate::BitReader<Rsten>;
impl RstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsten {
        match self.bits {
            false => Rsten::Value1,
            true => Rsten::Value2,
        }
    }
    #[doc = "Flag PSR\\[9\\]
is not modified depending on the receiver status."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rsten::Value1
    }
    #[doc = "Flag PSR\\[9\\]
is set during the complete reception of a frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rsten::Value2
    }
}
#[doc = "Field `RSTEN` writer - Receiver Status Enable"]
pub type RstenW<'a, REG> = crate::BitWriter<'a, REG, Rsten>;
impl<'a, REG> RstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag PSR\\[9\\]
is not modified depending on the receiver status."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsten::Value1)
    }
    #[doc = "Flag PSR\\[9\\]
is set during the complete reception of a frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rsten::Value2)
    }
}
#[doc = "Transmitter Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsten {
    #[doc = "0: Flag PSR\\[9\\]
is not modified depending on the transmitter status."]
    Value1 = 0,
    #[doc = "1: Flag PSR\\[9\\]
is set during the complete transmission of a frame."]
    Value2 = 1,
}
impl From<Tsten> for bool {
    #[inline(always)]
    fn from(variant: Tsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTEN` reader - Transmitter Status Enable"]
pub type TstenR = crate::BitReader<Tsten>;
impl TstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsten {
        match self.bits {
            false => Tsten::Value1,
            true => Tsten::Value2,
        }
    }
    #[doc = "Flag PSR\\[9\\]
is not modified depending on the transmitter status."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsten::Value1
    }
    #[doc = "Flag PSR\\[9\\]
is set during the complete transmission of a frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsten::Value2
    }
}
#[doc = "Field `TSTEN` writer - Transmitter Status Enable"]
pub type TstenW<'a, REG> = crate::BitWriter<'a, REG, Tsten>;
impl<'a, REG> TstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag PSR\\[9\\]
is not modified depending on the transmitter status."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsten::Value1)
    }
    #[doc = "Flag PSR\\[9\\]
is set during the complete transmission of a frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsten::Value2)
    }
}
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclk {
    #[doc = "0: The MCLK generation is disabled and the MCLK signal is 0."]
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
    #[doc = "The MCLK generation is disabled and the MCLK signal is 0."]
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
    #[doc = "The MCLK generation is disabled and the MCLK signal is 0."]
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
    #[doc = "Bit 0 - Sample Mode"]
    #[inline(always)]
    pub fn smd(&self) -> SmdR {
        SmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop Bits"]
    #[inline(always)]
    pub fn stpb(&self) -> StpbR {
        StpbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Idle Detection Mode"]
    #[inline(always)]
    pub fn idm(&self) -> IdmR {
        IdmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Break Interrupt Enable"]
    #[inline(always)]
    pub fn sbien(&self) -> SbienR {
        SbienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Collision Detection Enable"]
    #[inline(always)]
    pub fn cden(&self) -> CdenR {
        CdenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver Noise Detection Interrupt Enable"]
    #[inline(always)]
    pub fn rnien(&self) -> RnienR {
        RnienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Format Error Interrupt Enable"]
    #[inline(always)]
    pub fn feien(&self) -> FeienR {
        FeienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame Finished Interrupt Enable"]
    #[inline(always)]
    pub fn ffien(&self) -> FfienR {
        FfienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Sample Point"]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Pulse Length"]
    #[inline(always)]
    pub fn pl(&self) -> PlR {
        PlR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Receiver Status Enable"]
    #[inline(always)]
    pub fn rsten(&self) -> RstenR {
        RstenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmitter Status Enable"]
    #[inline(always)]
    pub fn tsten(&self) -> TstenR {
        TstenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MclkR {
        MclkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample Mode"]
    #[inline(always)]
    #[must_use]
    pub fn smd(&mut self) -> SmdW<PcrAscmodeSpec> {
        SmdW::new(self, 0)
    }
    #[doc = "Bit 1 - Stop Bits"]
    #[inline(always)]
    #[must_use]
    pub fn stpb(&mut self) -> StpbW<PcrAscmodeSpec> {
        StpbW::new(self, 1)
    }
    #[doc = "Bit 2 - Idle Detection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn idm(&mut self) -> IdmW<PcrAscmodeSpec> {
        IdmW::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronization Break Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sbien(&mut self) -> SbienW<PcrAscmodeSpec> {
        SbienW::new(self, 3)
    }
    #[doc = "Bit 4 - Collision Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cden(&mut self) -> CdenW<PcrAscmodeSpec> {
        CdenW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Noise Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rnien(&mut self) -> RnienW<PcrAscmodeSpec> {
        RnienW::new(self, 5)
    }
    #[doc = "Bit 6 - Format Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feien(&mut self) -> FeienW<PcrAscmodeSpec> {
        FeienW::new(self, 6)
    }
    #[doc = "Bit 7 - Frame Finished Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffien(&mut self) -> FfienW<PcrAscmodeSpec> {
        FfienW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SpW<PcrAscmodeSpec> {
        SpW::new(self, 8)
    }
    #[doc = "Bits 13:15 - Pulse Length"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PlW<PcrAscmodeSpec> {
        PlW::new(self, 13)
    }
    #[doc = "Bit 16 - Receiver Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsten(&mut self) -> RstenW<PcrAscmodeSpec> {
        RstenW::new(self, 16)
    }
    #[doc = "Bit 17 - Transmitter Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsten(&mut self) -> TstenW<PcrAscmodeSpec> {
        TstenW::new(self, 17)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mclk(&mut self) -> MclkW<PcrAscmodeSpec> {
        MclkW::new(self, 31)
    }
}
#[doc = "Protocol Control Register \\[ASC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_ascmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_ascmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrAscmodeSpec;
impl crate::RegisterSpec for PcrAscmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr_ascmode::R`](R) reader structure"]
impl crate::Readable for PcrAscmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`pcr_ascmode::W`](W) writer structure"]
impl crate::Writable for PcrAscmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCR_ASCMode to value 0"]
impl crate::Resettable for PcrAscmodeSpec {
    const RESET_VALUE: u32 = 0;
}
