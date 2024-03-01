#[doc = "Register `EEP_CONT_STAT` reader"]
pub type R = crate::R<EepContStatSpec>;
#[doc = "Register `EEP_CONT_STAT` writer"]
pub type W = crate::W<EepContStatSpec>;
#[doc = "ECAT write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WEn {
    #[doc = "0: Write requests are disabled"]
    Value1 = 0,
    #[doc = "1: Write requests are enabled"]
    Value2 = 1,
}
impl From<WEn> for bool {
    #[inline(always)]
    fn from(variant: WEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `W_EN` reader - ECAT write enable"]
pub type WEnR = crate::BitReader<WEn>;
impl WEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WEn {
        match self.bits {
            false => WEn::Value1,
            true => WEn::Value2,
        }
    }
    #[doc = "Write requests are disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WEn::Value1
    }
    #[doc = "Write requests are enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WEn::Value2
    }
}
#[doc = "EEPROM emulation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emul {
    #[doc = "0: Normal operation (I2C interface used)"]
    Value1 = 0,
    #[doc = "1: PDI emulates EEPROM (I2C not used)"]
    Value2 = 1,
}
impl From<Emul> for bool {
    #[inline(always)]
    fn from(variant: Emul) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMUL` reader - EEPROM emulation"]
pub type EmulR = crate::BitReader<Emul>;
impl EmulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emul {
        match self.bits {
            false => Emul::Value1,
            true => Emul::Value2,
        }
    }
    #[doc = "Normal operation (I2C interface used)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Emul::Value1
    }
    #[doc = "PDI emulates EEPROM (I2C not used)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Emul::Value2
    }
}
#[doc = "Supported number of EEPROM read bytes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bytes {
    #[doc = "0: 4 Bytes"]
    Value1 = 0,
    #[doc = "1: 8 Bytes"]
    Value2 = 1,
}
impl From<Bytes> for bool {
    #[inline(always)]
    fn from(variant: Bytes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYTES` reader - Supported number of EEPROM read bytes"]
pub type BytesR = crate::BitReader<Bytes>;
impl BytesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bytes {
        match self.bits {
            false => Bytes::Value1,
            true => Bytes::Value2,
        }
    }
    #[doc = "4 Bytes"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bytes::Value1
    }
    #[doc = "8 Bytes"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bytes::Value2
    }
}
#[doc = "Selected EEPROM Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alg {
    #[doc = "0: 1 address byte (1 KBit - 16 KBit EEPROMs)"]
    Value1 = 0,
    #[doc = "1: 2 address bytes (32 KBit - 4 MBit EEPROMs)"]
    Value2 = 1,
}
impl From<Alg> for bool {
    #[inline(always)]
    fn from(variant: Alg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALG` reader - Selected EEPROM Algorithm"]
pub type AlgR = crate::BitReader<Alg>;
impl AlgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alg {
        match self.bits {
            false => Alg::Value1,
            true => Alg::Value2,
        }
    }
    #[doc = "1 address byte (1 KBit - 16 KBit EEPROMs)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Alg::Value1
    }
    #[doc = "2 address bytes (32 KBit - 4 MBit EEPROMs)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Alg::Value2
    }
}
#[doc = "Command register\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CmdReg {
    #[doc = "0: No command/EEPROM idle (clear error bits)"]
    Value1 = 0,
    #[doc = "1: Read"]
    Value2 = 1,
    #[doc = "2: Write"]
    Value3 = 2,
    #[doc = "4: Reload"]
    Value4 = 4,
}
impl From<CmdReg> for u8 {
    #[inline(always)]
    fn from(variant: CmdReg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CmdReg {
    type Ux = u8;
}
#[doc = "Field `CMD_REG` reader - Command register"]
pub type CmdRegR = crate::FieldReader<CmdReg>;
impl CmdRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CmdReg> {
        match self.bits {
            0 => Some(CmdReg::Value1),
            1 => Some(CmdReg::Value2),
            2 => Some(CmdReg::Value3),
            4 => Some(CmdReg::Value4),
            _ => None,
        }
    }
    #[doc = "No command/EEPROM idle (clear error bits)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdReg::Value1
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdReg::Value2
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CmdReg::Value3
    }
    #[doc = "Reload"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CmdReg::Value4
    }
}
#[doc = "Field `CMD_REG` writer - Command register"]
pub type CmdRegW<'a, REG> = crate::FieldWriter<'a, REG, 3, CmdReg>;
impl<'a, REG> CmdRegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No command/EEPROM idle (clear error bits)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdReg::Value1)
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdReg::Value2)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CmdReg::Value3)
    }
    #[doc = "Reload"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CmdReg::Value4)
    }
}
#[doc = "Checksum Error at in ESC Configuration Area\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Checksum OK"]
    Value1 = 0,
    #[doc = "1: Checksum error"]
    Value2 = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Checksum Error at in ESC Configuration Area"]
pub type ErrorR = crate::BitReader<Error>;
impl ErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Error {
        match self.bits {
            false => Error::Value1,
            true => Error::Value2,
        }
    }
    #[doc = "Checksum OK"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Error::Value1
    }
    #[doc = "Checksum error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Error::Value2
    }
}
#[doc = "EEPROM loading status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LStat {
    #[doc = "0: EEPROM loaded, device information OK"]
    Value1 = 0,
    #[doc = "1: EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure)"]
    Value2 = 1,
}
impl From<LStat> for bool {
    #[inline(always)]
    fn from(variant: LStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L_STAT` reader - EEPROM loading status"]
pub type LStatR = crate::BitReader<LStat>;
impl LStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LStat {
        match self.bits {
            false => LStat::Value1,
            true => LStat::Value2,
        }
    }
    #[doc = "EEPROM loaded, device information OK"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LStat::Value1
    }
    #[doc = "EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LStat::Value2
    }
}
#[doc = "Error Acknowledge/Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorAc {
    #[doc = "0: No error"]
    Value1 = 0,
    #[doc = "1: Missing EEPROM acknowledge or invalid command"]
    Value2 = 1,
}
impl From<ErrorAc> for bool {
    #[inline(always)]
    fn from(variant: ErrorAc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR_AC` reader - Error Acknowledge/Command"]
pub type ErrorAcR = crate::BitReader<ErrorAc>;
impl ErrorAcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErrorAc {
        match self.bits {
            false => ErrorAc::Value1,
            true => ErrorAc::Value2,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ErrorAc::Value1
    }
    #[doc = "Missing EEPROM acknowledge or invalid command"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ErrorAc::Value2
    }
}
#[doc = "Field `ERROR_AC` writer - Error Acknowledge/Command"]
pub type ErrorAcW<'a, REG> = crate::BitWriter<'a, REG, ErrorAc>;
impl<'a, REG> ErrorAcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorAc::Value1)
    }
    #[doc = "Missing EEPROM acknowledge or invalid command"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorAc::Value2)
    }
}
#[doc = "Error Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorWe {
    #[doc = "0: No error"]
    Value1 = 0,
    #[doc = "1: Write Command without Write enable"]
    Value2 = 1,
}
impl From<ErrorWe> for bool {
    #[inline(always)]
    fn from(variant: ErrorWe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR_WE` reader - Error Write Enable"]
pub type ErrorWeR = crate::BitReader<ErrorWe>;
impl ErrorWeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErrorWe {
        match self.bits {
            false => ErrorWe::Value1,
            true => ErrorWe::Value2,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ErrorWe::Value1
    }
    #[doc = "Write Command without Write enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ErrorWe::Value2
    }
}
#[doc = "Busy\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: EEPROM Interface is idle"]
    Value1 = 0,
    #[doc = "1: EEPROM Interface is busy"]
    Value2 = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy"]
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
    #[doc = "EEPROM Interface is idle"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Busy::Value1
    }
    #[doc = "EEPROM Interface is busy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Busy::Value2
    }
}
impl R {
    #[doc = "Bit 0 - ECAT write enable"]
    #[inline(always)]
    pub fn w_en(&self) -> WEnR {
        WEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - EEPROM emulation"]
    #[inline(always)]
    pub fn emul(&self) -> EmulR {
        EmulR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Supported number of EEPROM read bytes"]
    #[inline(always)]
    pub fn bytes(&self) -> BytesR {
        BytesR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selected EEPROM Algorithm"]
    #[inline(always)]
    pub fn alg(&self) -> AlgR {
        AlgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Command register"]
    #[inline(always)]
    pub fn cmd_reg(&self) -> CmdRegR {
        CmdRegR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Checksum Error at in ESC Configuration Area"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EEPROM loading status"]
    #[inline(always)]
    pub fn l_stat(&self) -> LStatR {
        LStatR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error Acknowledge/Command"]
    #[inline(always)]
    pub fn error_ac(&self) -> ErrorAcR {
        ErrorAcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error Write Enable"]
    #[inline(always)]
    pub fn error_we(&self) -> ErrorWeR {
        ErrorWeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - Command register"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_reg(&mut self) -> CmdRegW<EepContStatSpec> {
        CmdRegW::new(self, 8)
    }
    #[doc = "Bit 13 - Error Acknowledge/Command"]
    #[inline(always)]
    #[must_use]
    pub fn error_ac(&mut self) -> ErrorAcW<EepContStatSpec> {
        ErrorAcW::new(self, 13)
    }
}
#[doc = "EEPROM Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eep_cont_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eep_cont_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EepContStatSpec;
impl crate::RegisterSpec for EepContStatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eep_cont_stat::R`](R) reader structure"]
impl crate::Readable for EepContStatSpec {}
#[doc = "`write(|w| ..)` method takes [`eep_cont_stat::W`](W) writer structure"]
impl crate::Writable for EepContStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EEP_CONT_STAT to value 0x9460"]
impl crate::Resettable for EepContStatSpec {
    const RESET_VALUE: u16 = 0x9460;
}
