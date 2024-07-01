#[doc = "Register `PCR_ASCMode` reader"]
pub type R = crate::R<PCR_ASCMODE_SPEC>;
#[doc = "Register `PCR_ASCMode` writer"]
pub type W = crate::W<PCR_ASCMODE_SPEC>;
#[doc = "Sample Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMD_A {
    #[doc = "0: Only one sample is taken per bit time. The current input value is sampled."]
    VALUE1 = 0,
    #[doc = "1: Three samples are taken per bit time and a majority decision is made."]
    VALUE2 = 1,
}
impl From<SMD_A> for bool {
    #[inline(always)]
    fn from(variant: SMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMD` reader - Sample Mode"]
pub type SMD_R = crate::BitReader<SMD_A>;
impl SMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMD_A {
        match self.bits {
            false => SMD_A::VALUE1,
            true => SMD_A::VALUE2,
        }
    }
    #[doc = "Only one sample is taken per bit time. The current input value is sampled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMD_A::VALUE1
    }
    #[doc = "Three samples are taken per bit time and a majority decision is made."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMD_A::VALUE2
    }
}
#[doc = "Field `SMD` writer - Sample Mode"]
pub type SMD_W<'a, REG> = crate::BitWriter<'a, REG, SMD_A>;
impl<'a, REG> SMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only one sample is taken per bit time. The current input value is sampled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMD_A::VALUE1)
    }
    #[doc = "Three samples are taken per bit time and a majority decision is made."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMD_A::VALUE2)
    }
}
#[doc = "Stop Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STPB_A {
    #[doc = "0: The number of stop bits is 1."]
    VALUE1 = 0,
    #[doc = "1: The number of stop bits is 2."]
    VALUE2 = 1,
}
impl From<STPB_A> for bool {
    #[inline(always)]
    fn from(variant: STPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPB` reader - Stop Bits"]
pub type STPB_R = crate::BitReader<STPB_A>;
impl STPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STPB_A {
        match self.bits {
            false => STPB_A::VALUE1,
            true => STPB_A::VALUE2,
        }
    }
    #[doc = "The number of stop bits is 1."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STPB_A::VALUE1
    }
    #[doc = "The number of stop bits is 2."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STPB_A::VALUE2
    }
}
#[doc = "Field `STPB` writer - Stop Bits"]
pub type STPB_W<'a, REG> = crate::BitWriter<'a, REG, STPB_A>;
impl<'a, REG> STPB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of stop bits is 1."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STPB_A::VALUE1)
    }
    #[doc = "The number of stop bits is 2."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STPB_A::VALUE2)
    }
}
#[doc = "Idle Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDM_A {
    #[doc = "0: The bus idle detection is switched off and bits PSR.TXIDLE and PSR.RXIDLE are set automatically to enable data transfers without checking the inputs before."]
    VALUE1 = 0,
    #[doc = "1: The bus is considered as idle after a number of consecutive passive bit times defined by SCTR.FLE plus 2 (in the case without parity bit) or plus 3 (in the case with parity bit)."]
    VALUE2 = 1,
}
impl From<IDM_A> for bool {
    #[inline(always)]
    fn from(variant: IDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDM` reader - Idle Detection Mode"]
pub type IDM_R = crate::BitReader<IDM_A>;
impl IDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDM_A {
        match self.bits {
            false => IDM_A::VALUE1,
            true => IDM_A::VALUE2,
        }
    }
    #[doc = "The bus idle detection is switched off and bits PSR.TXIDLE and PSR.RXIDLE are set automatically to enable data transfers without checking the inputs before."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IDM_A::VALUE1
    }
    #[doc = "The bus is considered as idle after a number of consecutive passive bit times defined by SCTR.FLE plus 2 (in the case without parity bit) or plus 3 (in the case with parity bit)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IDM_A::VALUE2
    }
}
#[doc = "Field `IDM` writer - Idle Detection Mode"]
pub type IDM_W<'a, REG> = crate::BitWriter<'a, REG, IDM_A>;
impl<'a, REG> IDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The bus idle detection is switched off and bits PSR.TXIDLE and PSR.RXIDLE are set automatically to enable data transfers without checking the inputs before."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IDM_A::VALUE1)
    }
    #[doc = "The bus is considered as idle after a number of consecutive passive bit times defined by SCTR.FLE plus 2 (in the case without parity bit) or plus 3 (in the case with parity bit)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IDM_A::VALUE2)
    }
}
#[doc = "Synchronization Break Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBIEN_A {
    #[doc = "0: The interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<SBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SBIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBIEN` reader - Synchronization Break Interrupt Enable"]
pub type SBIEN_R = crate::BitReader<SBIEN_A>;
impl SBIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBIEN_A {
        match self.bits {
            false => SBIEN_A::VALUE1,
            true => SBIEN_A::VALUE2,
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SBIEN_A::VALUE1
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SBIEN_A::VALUE2
    }
}
#[doc = "Field `SBIEN` writer - Synchronization Break Interrupt Enable"]
pub type SBIEN_W<'a, REG> = crate::BitWriter<'a, REG, SBIEN_A>;
impl<'a, REG> SBIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SBIEN_A::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SBIEN_A::VALUE2)
    }
}
#[doc = "Collision Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDEN_A {
    #[doc = "0: The collision detection is disabled."]
    VALUE1 = 0,
    #[doc = "1: If a collision is detected, the transmitter stops its data transmission, outputs a 1, sets bit PSR.COL and generates a protocol interrupt. In order to allow data transmission again, PSR.COL has to be cleared by software."]
    VALUE2 = 1,
}
impl From<CDEN_A> for bool {
    #[inline(always)]
    fn from(variant: CDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDEN` reader - Collision Detection Enable"]
pub type CDEN_R = crate::BitReader<CDEN_A>;
impl CDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CDEN_A {
        match self.bits {
            false => CDEN_A::VALUE1,
            true => CDEN_A::VALUE2,
        }
    }
    #[doc = "The collision detection is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDEN_A::VALUE1
    }
    #[doc = "If a collision is detected, the transmitter stops its data transmission, outputs a 1, sets bit PSR.COL and generates a protocol interrupt. In order to allow data transmission again, PSR.COL has to be cleared by software."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDEN_A::VALUE2
    }
}
#[doc = "Field `CDEN` writer - Collision Detection Enable"]
pub type CDEN_W<'a, REG> = crate::BitWriter<'a, REG, CDEN_A>;
impl<'a, REG> CDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The collision detection is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CDEN_A::VALUE1)
    }
    #[doc = "If a collision is detected, the transmitter stops its data transmission, outputs a 1, sets bit PSR.COL and generates a protocol interrupt. In order to allow data transmission again, PSR.COL has to be cleared by software."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CDEN_A::VALUE2)
    }
}
#[doc = "Receiver Noise Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNIEN_A {
    #[doc = "0: The interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<RNIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNIEN` reader - Receiver Noise Detection Interrupt Enable"]
pub type RNIEN_R = crate::BitReader<RNIEN_A>;
impl RNIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNIEN_A {
        match self.bits {
            false => RNIEN_A::VALUE1,
            true => RNIEN_A::VALUE2,
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RNIEN_A::VALUE1
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RNIEN_A::VALUE2
    }
}
#[doc = "Field `RNIEN` writer - Receiver Noise Detection Interrupt Enable"]
pub type RNIEN_W<'a, REG> = crate::BitWriter<'a, REG, RNIEN_A>;
impl<'a, REG> RNIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RNIEN_A::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RNIEN_A::VALUE2)
    }
}
#[doc = "Format Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIEN_A {
    #[doc = "0: The interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<FEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: FEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIEN` reader - Format Error Interrupt Enable"]
pub type FEIEN_R = crate::BitReader<FEIEN_A>;
impl FEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FEIEN_A {
        match self.bits {
            false => FEIEN_A::VALUE1,
            true => FEIEN_A::VALUE2,
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FEIEN_A::VALUE1
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FEIEN_A::VALUE2
    }
}
#[doc = "Field `FEIEN` writer - Format Error Interrupt Enable"]
pub type FEIEN_W<'a, REG> = crate::BitWriter<'a, REG, FEIEN_A>;
impl<'a, REG> FEIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FEIEN_A::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FEIEN_A::VALUE2)
    }
}
#[doc = "Frame Finished Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFIEN_A {
    #[doc = "0: The interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<FFIEN_A> for bool {
    #[inline(always)]
    fn from(variant: FFIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFIEN` reader - Frame Finished Interrupt Enable"]
pub type FFIEN_R = crate::BitReader<FFIEN_A>;
impl FFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FFIEN_A {
        match self.bits {
            false => FFIEN_A::VALUE1,
            true => FFIEN_A::VALUE2,
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FFIEN_A::VALUE1
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FFIEN_A::VALUE2
    }
}
#[doc = "Field `FFIEN` writer - Frame Finished Interrupt Enable"]
pub type FFIEN_W<'a, REG> = crate::BitWriter<'a, REG, FFIEN_A>;
impl<'a, REG> FFIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FFIEN_A::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FFIEN_A::VALUE2)
    }
}
#[doc = "Field `SP` reader - Sample Point"]
pub type SP_R = crate::FieldReader;
#[doc = "Field `SP` writer - Sample Point"]
pub type SP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Pulse Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PL_A {
    #[doc = "0: The pulse length is equal to the bit length (no shortened 0)."]
    VALUE1 = 0,
    #[doc = "1: The pulse length of a 0 bit is 2 time quanta."]
    VALUE2 = 1,
    #[doc = "2: The pulse length of a 0 bit is 3 time quanta."]
    VALUE3 = 2,
    #[doc = "7: The pulse length of a 0 bit is 8 time quanta."]
    VALUE4 = 7,
}
impl From<PL_A> for u8 {
    #[inline(always)]
    fn from(variant: PL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PL_A {
    type Ux = u8;
}
impl crate::IsEnum for PL_A {}
#[doc = "Field `PL` reader - Pulse Length"]
pub type PL_R = crate::FieldReader<PL_A>;
impl PL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PL_A> {
        match self.bits {
            0 => Some(PL_A::VALUE1),
            1 => Some(PL_A::VALUE2),
            2 => Some(PL_A::VALUE3),
            7 => Some(PL_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "The pulse length is equal to the bit length (no shortened 0)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PL_A::VALUE1
    }
    #[doc = "The pulse length of a 0 bit is 2 time quanta."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PL_A::VALUE2
    }
    #[doc = "The pulse length of a 0 bit is 3 time quanta."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PL_A::VALUE3
    }
    #[doc = "The pulse length of a 0 bit is 8 time quanta."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PL_A::VALUE4
    }
}
#[doc = "Field `PL` writer - Pulse Length"]
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PL_A>;
impl<'a, REG> PL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The pulse length is equal to the bit length (no shortened 0)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PL_A::VALUE1)
    }
    #[doc = "The pulse length of a 0 bit is 2 time quanta."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PL_A::VALUE2)
    }
    #[doc = "The pulse length of a 0 bit is 3 time quanta."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PL_A::VALUE3)
    }
    #[doc = "The pulse length of a 0 bit is 8 time quanta."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PL_A::VALUE4)
    }
}
#[doc = "Receiver Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTEN_A {
    #[doc = "0: Flag PSR\\[9\\]
is not modified depending on the receiver status."]
    VALUE1 = 0,
    #[doc = "1: Flag PSR\\[9\\]
is set during the complete reception of a frame."]
    VALUE2 = 1,
}
impl From<RSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTEN` reader - Receiver Status Enable"]
pub type RSTEN_R = crate::BitReader<RSTEN_A>;
impl RSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTEN_A {
        match self.bits {
            false => RSTEN_A::VALUE1,
            true => RSTEN_A::VALUE2,
        }
    }
    #[doc = "Flag PSR\\[9\\]
is not modified depending on the receiver status."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSTEN_A::VALUE1
    }
    #[doc = "Flag PSR\\[9\\]
is set during the complete reception of a frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSTEN_A::VALUE2
    }
}
#[doc = "Field `RSTEN` writer - Receiver Status Enable"]
pub type RSTEN_W<'a, REG> = crate::BitWriter<'a, REG, RSTEN_A>;
impl<'a, REG> RSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag PSR\\[9\\]
is not modified depending on the receiver status."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RSTEN_A::VALUE1)
    }
    #[doc = "Flag PSR\\[9\\]
is set during the complete reception of a frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RSTEN_A::VALUE2)
    }
}
#[doc = "Transmitter Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTEN_A {
    #[doc = "0: Flag PSR\\[9\\]
is not modified depending on the transmitter status."]
    VALUE1 = 0,
    #[doc = "1: Flag PSR\\[9\\]
is set during the complete transmission of a frame."]
    VALUE2 = 1,
}
impl From<TSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTEN` reader - Transmitter Status Enable"]
pub type TSTEN_R = crate::BitReader<TSTEN_A>;
impl TSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSTEN_A {
        match self.bits {
            false => TSTEN_A::VALUE1,
            true => TSTEN_A::VALUE2,
        }
    }
    #[doc = "Flag PSR\\[9\\]
is not modified depending on the transmitter status."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSTEN_A::VALUE1
    }
    #[doc = "Flag PSR\\[9\\]
is set during the complete transmission of a frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSTEN_A::VALUE2
    }
}
#[doc = "Field `TSTEN` writer - Transmitter Status Enable"]
pub type TSTEN_W<'a, REG> = crate::BitWriter<'a, REG, TSTEN_A>;
impl<'a, REG> TSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag PSR\\[9\\]
is not modified depending on the transmitter status."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTEN_A::VALUE1)
    }
    #[doc = "Flag PSR\\[9\\]
is set during the complete transmission of a frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSTEN_A::VALUE2)
    }
}
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCLK_A {
    #[doc = "0: The MCLK generation is disabled and the MCLK signal is 0."]
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
#[doc = "Field `MCLK` reader - Master Clock Enable"]
pub type MCLK_R = crate::BitReader<MCLK_A>;
impl MCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCLK_A {
        match self.bits {
            false => MCLK_A::VALUE1,
            true => MCLK_A::VALUE2,
        }
    }
    #[doc = "The MCLK generation is disabled and the MCLK signal is 0."]
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
    #[doc = "The MCLK generation is disabled and the MCLK signal is 0."]
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
    #[doc = "Bit 0 - Sample Mode"]
    #[inline(always)]
    pub fn smd(&self) -> SMD_R {
        SMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop Bits"]
    #[inline(always)]
    pub fn stpb(&self) -> STPB_R {
        STPB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Idle Detection Mode"]
    #[inline(always)]
    pub fn idm(&self) -> IDM_R {
        IDM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Break Interrupt Enable"]
    #[inline(always)]
    pub fn sbien(&self) -> SBIEN_R {
        SBIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Collision Detection Enable"]
    #[inline(always)]
    pub fn cden(&self) -> CDEN_R {
        CDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver Noise Detection Interrupt Enable"]
    #[inline(always)]
    pub fn rnien(&self) -> RNIEN_R {
        RNIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Format Error Interrupt Enable"]
    #[inline(always)]
    pub fn feien(&self) -> FEIEN_R {
        FEIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame Finished Interrupt Enable"]
    #[inline(always)]
    pub fn ffien(&self) -> FFIEN_R {
        FFIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Sample Point"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Pulse Length"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Receiver Status Enable"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmitter Status Enable"]
    #[inline(always)]
    pub fn tsten(&self) -> TSTEN_R {
        TSTEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MCLK_R {
        MCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample Mode"]
    #[inline(always)]
    #[must_use]
    pub fn smd(&mut self) -> SMD_W<PCR_ASCMODE_SPEC> {
        SMD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Stop Bits"]
    #[inline(always)]
    #[must_use]
    pub fn stpb(&mut self) -> STPB_W<PCR_ASCMODE_SPEC> {
        STPB_W::new(self, 1)
    }
    #[doc = "Bit 2 - Idle Detection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn idm(&mut self) -> IDM_W<PCR_ASCMODE_SPEC> {
        IDM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronization Break Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sbien(&mut self) -> SBIEN_W<PCR_ASCMODE_SPEC> {
        SBIEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Collision Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cden(&mut self) -> CDEN_W<PCR_ASCMODE_SPEC> {
        CDEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Noise Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rnien(&mut self) -> RNIEN_W<PCR_ASCMODE_SPEC> {
        RNIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Format Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feien(&mut self) -> FEIEN_W<PCR_ASCMODE_SPEC> {
        FEIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Frame Finished Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffien(&mut self) -> FFIEN_W<PCR_ASCMODE_SPEC> {
        FFIEN_W::new(self, 7)
    }
    #[doc = "Bits 8:12 - Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<PCR_ASCMODE_SPEC> {
        SP_W::new(self, 8)
    }
    #[doc = "Bits 13:15 - Pulse Length"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<PCR_ASCMODE_SPEC> {
        PL_W::new(self, 13)
    }
    #[doc = "Bit 16 - Receiver Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsten(&mut self) -> RSTEN_W<PCR_ASCMODE_SPEC> {
        RSTEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Transmitter Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsten(&mut self) -> TSTEN_W<PCR_ASCMODE_SPEC> {
        TSTEN_W::new(self, 17)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mclk(&mut self) -> MCLK_W<PCR_ASCMODE_SPEC> {
        MCLK_W::new(self, 31)
    }
}
#[doc = "Protocol Control Register \\[ASC Mode\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr_ascmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr_ascmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCR_ASCMODE_SPEC;
impl crate::RegisterSpec for PCR_ASCMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr_ascmode::R`](R) reader structure"]
impl crate::Readable for PCR_ASCMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcr_ascmode::W`](W) writer structure"]
impl crate::Writable for PCR_ASCMODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCR_ASCMode to value 0"]
impl crate::Resettable for PCR_ASCMODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
