#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: The USIC channel is disabled. All protocol-related state machines are set to an idle state."]
    Value1 = 0,
    #[doc = "1: The SSC (SPI) protocol is selected."]
    Value2 = 1,
    #[doc = "2: The ASC (SCI, UART) protocol is selected."]
    Value3 = 2,
    #[doc = "3: The IIS protocol is selected."]
    Value4 = 3,
    #[doc = "4: The IIC protocol is selected."]
    Value5 = 4,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Operating Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Value1),
            1 => Some(Mode::Value2),
            2 => Some(Mode::Value3),
            3 => Some(Mode::Value4),
            4 => Some(Mode::Value5),
            _ => None,
        }
    }
    #[doc = "The USIC channel is disabled. All protocol-related state machines are set to an idle state."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mode::Value1
    }
    #[doc = "The SSC (SPI) protocol is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mode::Value2
    }
    #[doc = "The ASC (SCI, UART) protocol is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Mode::Value3
    }
    #[doc = "The IIS protocol is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Mode::Value4
    }
    #[doc = "The IIC protocol is selected."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Mode::Value5
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The USIC channel is disabled. All protocol-related state machines are set to an idle state."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value1)
    }
    #[doc = "The SSC (SPI) protocol is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value2)
    }
    #[doc = "The ASC (SCI, UART) protocol is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value3)
    }
    #[doc = "The IIS protocol is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value4)
    }
    #[doc = "The IIC protocol is selected."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value5)
    }
}
#[doc = "Hardware Port Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hpcen {
    #[doc = "0: The hardware port control is disabled."]
    Value1 = 0,
    #[doc = "1: The hardware port control is enabled for DX0 and DOUT0."]
    Value2 = 1,
    #[doc = "2: The hardware port control is enabled for DX3, DX0 and DOUT\\[1:0\\]."]
    Value3 = 2,
    #[doc = "3: The hardware port control is enabled for DX0, DX\\[5:3\\]
and DOUT\\[3:0\\]."]
    Value4 = 3,
}
impl From<Hpcen> for u8 {
    #[inline(always)]
    fn from(variant: Hpcen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hpcen {
    type Ux = u8;
}
impl crate::IsEnum for Hpcen {}
#[doc = "Field `HPCEN` reader - Hardware Port Control Enable"]
pub type HpcenR = crate::FieldReader<Hpcen>;
impl HpcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hpcen {
        match self.bits {
            0 => Hpcen::Value1,
            1 => Hpcen::Value2,
            2 => Hpcen::Value3,
            3 => Hpcen::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "The hardware port control is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hpcen::Value1
    }
    #[doc = "The hardware port control is enabled for DX0 and DOUT0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hpcen::Value2
    }
    #[doc = "The hardware port control is enabled for DX3, DX0 and DOUT\\[1:0\\]."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Hpcen::Value3
    }
    #[doc = "The hardware port control is enabled for DX0, DX\\[5:3\\]
and DOUT\\[3:0\\]."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Hpcen::Value4
    }
}
#[doc = "Field `HPCEN` writer - Hardware Port Control Enable"]
pub type HpcenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hpcen, crate::Safe>;
impl<'a, REG> HpcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The hardware port control is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hpcen::Value1)
    }
    #[doc = "The hardware port control is enabled for DX0 and DOUT0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hpcen::Value2)
    }
    #[doc = "The hardware port control is enabled for DX3, DX0 and DOUT\\[1:0\\]."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Hpcen::Value3)
    }
    #[doc = "The hardware port control is enabled for DX0, DX\\[5:3\\]
and DOUT\\[3:0\\]."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Hpcen::Value4)
    }
}
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pm {
    #[doc = "0: The parity generation is disabled."]
    Value1 = 0,
    #[doc = "2: Even parity is selected (parity bit = 1 on odd number of 1s in data, parity bit = 0 on even number of 1s in data)."]
    Value3 = 2,
    #[doc = "3: Odd parity is selected (parity bit = 0 on odd number of 1s in data, parity bit = 1 on even number of 1s in data)."]
    Value4 = 3,
}
impl From<Pm> for u8 {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pm {
    type Ux = u8;
}
impl crate::IsEnum for Pm {}
#[doc = "Field `PM` reader - Parity Mode"]
pub type PmR = crate::FieldReader<Pm>;
impl PmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pm> {
        match self.bits {
            0 => Some(Pm::Value1),
            2 => Some(Pm::Value3),
            3 => Some(Pm::Value4),
            _ => None,
        }
    }
    #[doc = "The parity generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pm::Value1
    }
    #[doc = "Even parity is selected (parity bit = 1 on odd number of 1s in data, parity bit = 0 on even number of 1s in data)."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pm::Value3
    }
    #[doc = "Odd parity is selected (parity bit = 0 on odd number of 1s in data, parity bit = 1 on even number of 1s in data)."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pm::Value4
    }
}
#[doc = "Field `PM` writer - Parity Mode"]
pub type PmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pm>;
impl<'a, REG> PmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The parity generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::Value1)
    }
    #[doc = "Even parity is selected (parity bit = 1 on odd number of 1s in data, parity bit = 0 on even number of 1s in data)."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::Value3)
    }
    #[doc = "Odd parity is selected (parity bit = 0 on odd number of 1s in data, parity bit = 1 on even number of 1s in data)."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::Value4)
    }
}
#[doc = "Receiver Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsien {
    #[doc = "0: The receiver start interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The receiver start interrupt is enabled. In case of a receiver start event, the service request output SRx indicated by INPR.TBINP is activated."]
    Value2 = 1,
}
impl From<Rsien> for bool {
    #[inline(always)]
    fn from(variant: Rsien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSIEN` reader - Receiver Start Interrupt Enable"]
pub type RsienR = crate::BitReader<Rsien>;
impl RsienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsien {
        match self.bits {
            false => Rsien::Value1,
            true => Rsien::Value2,
        }
    }
    #[doc = "The receiver start interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rsien::Value1
    }
    #[doc = "The receiver start interrupt is enabled. In case of a receiver start event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rsien::Value2
    }
}
#[doc = "Field `RSIEN` writer - Receiver Start Interrupt Enable"]
pub type RsienW<'a, REG> = crate::BitWriter<'a, REG, Rsien>;
impl<'a, REG> RsienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receiver start interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsien::Value1)
    }
    #[doc = "The receiver start interrupt is enabled. In case of a receiver start event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rsien::Value2)
    }
}
#[doc = "Data Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlien {
    #[doc = "0: The data lost interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The data lost interrupt is enabled. In case of a data lost event, the service request output SRx indicated by INPR.PINP is activated."]
    Value2 = 1,
}
impl From<Dlien> for bool {
    #[inline(always)]
    fn from(variant: Dlien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLIEN` reader - Data Lost Interrupt Enable"]
pub type DlienR = crate::BitReader<Dlien>;
impl DlienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlien {
        match self.bits {
            false => Dlien::Value1,
            true => Dlien::Value2,
        }
    }
    #[doc = "The data lost interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dlien::Value1
    }
    #[doc = "The data lost interrupt is enabled. In case of a data lost event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dlien::Value2
    }
}
#[doc = "Field `DLIEN` writer - Data Lost Interrupt Enable"]
pub type DlienW<'a, REG> = crate::BitWriter<'a, REG, Dlien>;
impl<'a, REG> DlienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data lost interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlien::Value1)
    }
    #[doc = "The data lost interrupt is enabled. In case of a data lost event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dlien::Value2)
    }
}
#[doc = "Transmit Shift Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsien {
    #[doc = "0: The transmit shift interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The transmit shift interrupt is enabled. In case of a transmit shift interrupt event, the service request output SRx indicated by INPR.TSINP is activated."]
    Value2 = 1,
}
impl From<Tsien> for bool {
    #[inline(always)]
    fn from(variant: Tsien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIEN` reader - Transmit Shift Interrupt Enable"]
pub type TsienR = crate::BitReader<Tsien>;
impl TsienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsien {
        match self.bits {
            false => Tsien::Value1,
            true => Tsien::Value2,
        }
    }
    #[doc = "The transmit shift interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsien::Value1
    }
    #[doc = "The transmit shift interrupt is enabled. In case of a transmit shift interrupt event, the service request output SRx indicated by INPR.TSINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsien::Value2
    }
}
#[doc = "Field `TSIEN` writer - Transmit Shift Interrupt Enable"]
pub type TsienW<'a, REG> = crate::BitWriter<'a, REG, Tsien>;
impl<'a, REG> TsienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmit shift interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsien::Value1)
    }
    #[doc = "The transmit shift interrupt is enabled. In case of a transmit shift interrupt event, the service request output SRx indicated by INPR.TSINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsien::Value2)
    }
}
#[doc = "Transmit Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbien {
    #[doc = "0: The transmit buffer interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The transmit buffer interrupt is enabled. In case of a transmit buffer event, the service request output SRx indicated by INPR.TBINP is activated."]
    Value2 = 1,
}
impl From<Tbien> for bool {
    #[inline(always)]
    fn from(variant: Tbien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBIEN` reader - Transmit Buffer Interrupt Enable"]
pub type TbienR = crate::BitReader<Tbien>;
impl TbienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbien {
        match self.bits {
            false => Tbien::Value1,
            true => Tbien::Value2,
        }
    }
    #[doc = "The transmit buffer interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tbien::Value1
    }
    #[doc = "The transmit buffer interrupt is enabled. In case of a transmit buffer event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tbien::Value2
    }
}
#[doc = "Field `TBIEN` writer - Transmit Buffer Interrupt Enable"]
pub type TbienW<'a, REG> = crate::BitWriter<'a, REG, Tbien>;
impl<'a, REG> TbienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmit buffer interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tbien::Value1)
    }
    #[doc = "The transmit buffer interrupt is enabled. In case of a transmit buffer event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tbien::Value2)
    }
}
#[doc = "Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rien {
    #[doc = "0: The receive interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The receive interrupt is enabled. In case of a receive event, the service request output SRx indicated by INPR.RINP is activated."]
    Value2 = 1,
}
impl From<Rien> for bool {
    #[inline(always)]
    fn from(variant: Rien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIEN` reader - Receive Interrupt Enable"]
pub type RienR = crate::BitReader<Rien>;
impl RienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rien {
        match self.bits {
            false => Rien::Value1,
            true => Rien::Value2,
        }
    }
    #[doc = "The receive interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rien::Value1
    }
    #[doc = "The receive interrupt is enabled. In case of a receive event, the service request output SRx indicated by INPR.RINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rien::Value2
    }
}
#[doc = "Field `RIEN` writer - Receive Interrupt Enable"]
pub type RienW<'a, REG> = crate::BitWriter<'a, REG, Rien>;
impl<'a, REG> RienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receive interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rien::Value1)
    }
    #[doc = "The receive interrupt is enabled. In case of a receive event, the service request output SRx indicated by INPR.RINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rien::Value2)
    }
}
#[doc = "Alternative Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aien {
    #[doc = "0: The alternative receive interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The alternative receive interrupt is enabled. In case of an alternative receive event, the service request output SRx indicated by INPR.AINP is activated."]
    Value2 = 1,
}
impl From<Aien> for bool {
    #[inline(always)]
    fn from(variant: Aien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIEN` reader - Alternative Receive Interrupt Enable"]
pub type AienR = crate::BitReader<Aien>;
impl AienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aien {
        match self.bits {
            false => Aien::Value1,
            true => Aien::Value2,
        }
    }
    #[doc = "The alternative receive interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Aien::Value1
    }
    #[doc = "The alternative receive interrupt is enabled. In case of an alternative receive event, the service request output SRx indicated by INPR.AINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Aien::Value2
    }
}
#[doc = "Field `AIEN` writer - Alternative Receive Interrupt Enable"]
pub type AienW<'a, REG> = crate::BitWriter<'a, REG, Aien>;
impl<'a, REG> AienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The alternative receive interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Aien::Value1)
    }
    #[doc = "The alternative receive interrupt is enabled. In case of an alternative receive event, the service request output SRx indicated by INPR.AINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Aien::Value2)
    }
}
#[doc = "Baud Rate Generator Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brgien {
    #[doc = "0: The baud rate generator interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: The baud rate generator interrupt is enabled. In case of a baud rate generator event, the service request output SRx indicated by INPR.PINP is activated."]
    Value2 = 1,
}
impl From<Brgien> for bool {
    #[inline(always)]
    fn from(variant: Brgien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRGIEN` reader - Baud Rate Generator Interrupt Enable"]
pub type BrgienR = crate::BitReader<Brgien>;
impl BrgienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brgien {
        match self.bits {
            false => Brgien::Value1,
            true => Brgien::Value2,
        }
    }
    #[doc = "The baud rate generator interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Brgien::Value1
    }
    #[doc = "The baud rate generator interrupt is enabled. In case of a baud rate generator event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Brgien::Value2
    }
}
#[doc = "Field `BRGIEN` writer - Baud Rate Generator Interrupt Enable"]
pub type BrgienW<'a, REG> = crate::BitWriter<'a, REG, Brgien>;
impl<'a, REG> BrgienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The baud rate generator interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Brgien::Value1)
    }
    #[doc = "The baud rate generator interrupt is enabled. In case of a baud rate generator event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Brgien::Value2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Hardware Port Control Enable"]
    #[inline(always)]
    pub fn hpcen(&self) -> HpcenR {
        HpcenR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Parity Mode"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Receiver Start Interrupt Enable"]
    #[inline(always)]
    pub fn rsien(&self) -> RsienR {
        RsienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Lost Interrupt Enable"]
    #[inline(always)]
    pub fn dlien(&self) -> DlienR {
        DlienR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Shift Interrupt Enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TsienR {
        TsienR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn tbien(&self) -> TbienR {
        TbienR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rien(&self) -> RienR {
        RienR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Alternative Receive Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&self) -> AienR {
        AienR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Baud Rate Generator Interrupt Enable"]
    #[inline(always)]
    pub fn brgien(&self) -> BrgienR {
        BrgienR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CcrSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Hardware Port Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpcen(&mut self) -> HpcenW<CcrSpec> {
        HpcenW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Parity Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PmW<CcrSpec> {
        PmW::new(self, 8)
    }
    #[doc = "Bit 10 - Receiver Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsien(&mut self) -> RsienW<CcrSpec> {
        RsienW::new(self, 10)
    }
    #[doc = "Bit 11 - Data Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlien(&mut self) -> DlienW<CcrSpec> {
        DlienW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Shift Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TsienW<CcrSpec> {
        TsienW::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbien(&mut self) -> TbienW<CcrSpec> {
        TbienW::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RienW<CcrSpec> {
        RienW::new(self, 14)
    }
    #[doc = "Bit 15 - Alternative Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aien(&mut self) -> AienW<CcrSpec> {
        AienW::new(self, 15)
    }
    #[doc = "Bit 16 - Baud Rate Generator Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brgien(&mut self) -> BrgienW<CcrSpec> {
        BrgienW::new(self, 16)
    }
}
#[doc = "Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0;
}
