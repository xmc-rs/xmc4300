#[doc = "Register `INT_STATUS_ERR` reader"]
pub type R = crate::R<IntStatusErrSpec>;
#[doc = "Register `INT_STATUS_ERR` writer"]
pub type W = crate::W<IntStatusErrSpec>;
#[doc = "Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdTimeoutErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: Timeout"]
    Value2 = 1,
}
impl From<CmdTimeoutErr> for bool {
    #[inline(always)]
    fn from(variant: CmdTimeoutErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_TIMEOUT_ERR` reader - Command Timeout Error"]
pub type CmdTimeoutErrR = crate::BitReader<CmdTimeoutErr>;
impl CmdTimeoutErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdTimeoutErr {
        match self.bits {
            false => CmdTimeoutErr::Value1,
            true => CmdTimeoutErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdTimeoutErr::Value1
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdTimeoutErr::Value2
    }
}
#[doc = "Field `CMD_TIMEOUT_ERR` writer - Command Timeout Error"]
pub type CmdTimeoutErrW<'a, REG> = crate::BitWriter<'a, REG, CmdTimeoutErr>;
impl<'a, REG> CmdTimeoutErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdTimeoutErr::Value1)
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdTimeoutErr::Value2)
    }
}
#[doc = "Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdCrcErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: CRC Error Generated"]
    Value2 = 1,
}
impl From<CmdCrcErr> for bool {
    #[inline(always)]
    fn from(variant: CmdCrcErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_CRC_ERR` reader - Command CRC Error"]
pub type CmdCrcErrR = crate::BitReader<CmdCrcErr>;
impl CmdCrcErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdCrcErr {
        match self.bits {
            false => CmdCrcErr::Value1,
            true => CmdCrcErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdCrcErr::Value1
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdCrcErr::Value2
    }
}
#[doc = "Field `CMD_CRC_ERR` writer - Command CRC Error"]
pub type CmdCrcErrW<'a, REG> = crate::BitWriter<'a, REG, CmdCrcErr>;
impl<'a, REG> CmdCrcErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdCrcErr::Value1)
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdCrcErr::Value2)
    }
}
#[doc = "Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdEndBitErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: End Bit Error Generated"]
    Value2 = 1,
}
impl From<CmdEndBitErr> for bool {
    #[inline(always)]
    fn from(variant: CmdEndBitErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_END_BIT_ERR` reader - Command End Bit Error"]
pub type CmdEndBitErrR = crate::BitReader<CmdEndBitErr>;
impl CmdEndBitErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdEndBitErr {
        match self.bits {
            false => CmdEndBitErr::Value1,
            true => CmdEndBitErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdEndBitErr::Value1
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdEndBitErr::Value2
    }
}
#[doc = "Field `CMD_END_BIT_ERR` writer - Command End Bit Error"]
pub type CmdEndBitErrW<'a, REG> = crate::BitWriter<'a, REG, CmdEndBitErr>;
impl<'a, REG> CmdEndBitErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdEndBitErr::Value1)
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdEndBitErr::Value2)
    }
}
#[doc = "Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdIndErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: Error"]
    Value2 = 1,
}
impl From<CmdIndErr> for bool {
    #[inline(always)]
    fn from(variant: CmdIndErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_IND_ERR` reader - Command Index Error"]
pub type CmdIndErrR = crate::BitReader<CmdIndErr>;
impl CmdIndErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdIndErr {
        match self.bits {
            false => CmdIndErr::Value1,
            true => CmdIndErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdIndErr::Value1
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdIndErr::Value2
    }
}
#[doc = "Field `CMD_IND_ERR` writer - Command Index Error"]
pub type CmdIndErrW<'a, REG> = crate::BitWriter<'a, REG, CmdIndErr>;
impl<'a, REG> CmdIndErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdIndErr::Value1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdIndErr::Value2)
    }
}
#[doc = "Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTimeoutErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: Timeout"]
    Value2 = 1,
}
impl From<DataTimeoutErr> for bool {
    #[inline(always)]
    fn from(variant: DataTimeoutErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_TIMEOUT_ERR` reader - Data Timeout Error"]
pub type DataTimeoutErrR = crate::BitReader<DataTimeoutErr>;
impl DataTimeoutErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataTimeoutErr {
        match self.bits {
            false => DataTimeoutErr::Value1,
            true => DataTimeoutErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DataTimeoutErr::Value1
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DataTimeoutErr::Value2
    }
}
#[doc = "Field `DATA_TIMEOUT_ERR` writer - Data Timeout Error"]
pub type DataTimeoutErrW<'a, REG> = crate::BitWriter<'a, REG, DataTimeoutErr>;
impl<'a, REG> DataTimeoutErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DataTimeoutErr::Value1)
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DataTimeoutErr::Value2)
    }
}
#[doc = "Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataCrcErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: Error"]
    Value2 = 1,
}
impl From<DataCrcErr> for bool {
    #[inline(always)]
    fn from(variant: DataCrcErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_CRC_ERR` reader - Data CRC Error"]
pub type DataCrcErrR = crate::BitReader<DataCrcErr>;
impl DataCrcErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataCrcErr {
        match self.bits {
            false => DataCrcErr::Value1,
            true => DataCrcErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DataCrcErr::Value1
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DataCrcErr::Value2
    }
}
#[doc = "Field `DATA_CRC_ERR` writer - Data CRC Error"]
pub type DataCrcErrW<'a, REG> = crate::BitWriter<'a, REG, DataCrcErr>;
impl<'a, REG> DataCrcErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DataCrcErr::Value1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DataCrcErr::Value2)
    }
}
#[doc = "Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataEndBitErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: Error"]
    Value2 = 1,
}
impl From<DataEndBitErr> for bool {
    #[inline(always)]
    fn from(variant: DataEndBitErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_END_BIT_ERR` reader - Data End Bit Error"]
pub type DataEndBitErrR = crate::BitReader<DataEndBitErr>;
impl DataEndBitErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataEndBitErr {
        match self.bits {
            false => DataEndBitErr::Value1,
            true => DataEndBitErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DataEndBitErr::Value1
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DataEndBitErr::Value2
    }
}
#[doc = "Field `DATA_END_BIT_ERR` writer - Data End Bit Error"]
pub type DataEndBitErrW<'a, REG> = crate::BitWriter<'a, REG, DataEndBitErr>;
impl<'a, REG> DataEndBitErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DataEndBitErr::Value1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DataEndBitErr::Value2)
    }
}
#[doc = "Current Limit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentLimitErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: Power Fail"]
    Value2 = 1,
}
impl From<CurrentLimitErr> for bool {
    #[inline(always)]
    fn from(variant: CurrentLimitErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURRENT_LIMIT_ERR` reader - Current Limit Error"]
pub type CurrentLimitErrR = crate::BitReader<CurrentLimitErr>;
impl CurrentLimitErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CurrentLimitErr {
        match self.bits {
            false => CurrentLimitErr::Value1,
            true => CurrentLimitErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CurrentLimitErr::Value1
    }
    #[doc = "Power Fail"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CurrentLimitErr::Value2
    }
}
#[doc = "Field `CURRENT_LIMIT_ERR` writer - Current Limit Error"]
pub type CurrentLimitErrW<'a, REG> = crate::BitWriter<'a, REG, CurrentLimitErr>;
impl<'a, REG> CurrentLimitErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CurrentLimitErr::Value1)
    }
    #[doc = "Power Fail"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CurrentLimitErr::Value2)
    }
}
#[doc = "Auto CMD Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcmdErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: Error"]
    Value2 = 1,
}
impl From<AcmdErr> for bool {
    #[inline(always)]
    fn from(variant: AcmdErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_ERR` reader - Auto CMD Error"]
pub type AcmdErrR = crate::BitReader<AcmdErr>;
impl AcmdErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcmdErr {
        match self.bits {
            false => AcmdErr::Value1,
            true => AcmdErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AcmdErr::Value1
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AcmdErr::Value2
    }
}
#[doc = "Field `ACMD_ERR` writer - Auto CMD Error"]
pub type AcmdErrW<'a, REG> = crate::BitWriter<'a, REG, AcmdErr>;
impl<'a, REG> AcmdErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AcmdErr::Value1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AcmdErr::Value2)
    }
}
#[doc = "Ceata Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CeataErr {
    #[doc = "0: no error"]
    Value1 = 0,
    #[doc = "1: error"]
    Value2 = 1,
}
impl From<CeataErr> for bool {
    #[inline(always)]
    fn from(variant: CeataErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEATA_ERR` reader - Ceata Error Status"]
pub type CeataErrR = crate::BitReader<CeataErr>;
impl CeataErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CeataErr {
        match self.bits {
            false => CeataErr::Value1,
            true => CeataErr::Value2,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CeataErr::Value1
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CeataErr::Value2
    }
}
#[doc = "Field `CEATA_ERR` writer - Ceata Error Status"]
pub type CeataErrW<'a, REG> = crate::BitWriter<'a, REG, CeataErr>;
impl<'a, REG> CeataErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CeataErr::Value1)
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CeataErr::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline(always)]
    pub fn cmd_timeout_err(&self) -> CmdTimeoutErrR {
        CmdTimeoutErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline(always)]
    pub fn cmd_crc_err(&self) -> CmdCrcErrR {
        CmdCrcErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&self) -> CmdEndBitErrR {
        CmdEndBitErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline(always)]
    pub fn cmd_ind_err(&self) -> CmdIndErrR {
        CmdIndErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline(always)]
    pub fn data_timeout_err(&self) -> DataTimeoutErrR {
        DataTimeoutErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline(always)]
    pub fn data_crc_err(&self) -> DataCrcErrR {
        DataCrcErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline(always)]
    pub fn data_end_bit_err(&self) -> DataEndBitErrR {
        DataEndBitErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline(always)]
    pub fn current_limit_err(&self) -> CurrentLimitErrR {
        CurrentLimitErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline(always)]
    pub fn acmd_err(&self) -> AcmdErrR {
        AcmdErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - Ceata Error Status"]
    #[inline(always)]
    pub fn ceata_err(&self) -> CeataErrR {
        CeataErrR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_timeout_err(&mut self) -> CmdTimeoutErrW<IntStatusErrSpec> {
        CmdTimeoutErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_err(&mut self) -> CmdCrcErrW<IntStatusErrSpec> {
        CmdCrcErrW::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_end_bit_err(&mut self) -> CmdEndBitErrW<IntStatusErrSpec> {
        CmdEndBitErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ind_err(&mut self) -> CmdIndErrW<IntStatusErrSpec> {
        CmdIndErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn data_timeout_err(&mut self) -> DataTimeoutErrW<IntStatusErrSpec> {
        DataTimeoutErrW::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn data_crc_err(&mut self) -> DataCrcErrW<IntStatusErrSpec> {
        DataCrcErrW::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn data_end_bit_err(&mut self) -> DataEndBitErrW<IntStatusErrSpec> {
        DataEndBitErrW::new(self, 6)
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline(always)]
    #[must_use]
    pub fn current_limit_err(&mut self) -> CurrentLimitErrW<IntStatusErrSpec> {
        CurrentLimitErrW::new(self, 7)
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline(always)]
    #[must_use]
    pub fn acmd_err(&mut self) -> AcmdErrW<IntStatusErrSpec> {
        AcmdErrW::new(self, 8)
    }
    #[doc = "Bit 13 - Ceata Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn ceata_err(&mut self) -> CeataErrW<IntStatusErrSpec> {
        CeataErrW::new(self, 13)
    }
}
#[doc = "Error Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status_err::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status_err::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusErrSpec;
impl crate::RegisterSpec for IntStatusErrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`int_status_err::R`](R) reader structure"]
impl crate::Readable for IntStatusErrSpec {}
#[doc = "`write(|w| ..)` method takes [`int_status_err::W`](W) writer structure"]
impl crate::Writable for IntStatusErrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INT_STATUS_ERR to value 0"]
impl crate::Resettable for IntStatusErrSpec {
    const RESET_VALUE: u16 = 0;
}
