#[doc = "Register `EN_INT_STATUS_ERR` reader"]
pub type R = crate::R<EnIntStatusErrSpec>;
#[doc = "Register `EN_INT_STATUS_ERR` writer"]
pub type W = crate::W<EnIntStatusErrSpec>;
#[doc = "Command Timeout Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdTimeoutErrEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<CmdTimeoutErrEn> for bool {
    #[inline(always)]
    fn from(variant: CmdTimeoutErrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_TIMEOUT_ERR_EN` reader - Command Timeout Error Status Enable"]
pub type CmdTimeoutErrEnR = crate::BitReader<CmdTimeoutErrEn>;
impl CmdTimeoutErrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdTimeoutErrEn {
        match self.bits {
            false => CmdTimeoutErrEn::Value1,
            true => CmdTimeoutErrEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdTimeoutErrEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdTimeoutErrEn::Value2
    }
}
#[doc = "Field `CMD_TIMEOUT_ERR_EN` writer - Command Timeout Error Status Enable"]
pub type CmdTimeoutErrEnW<'a, REG> = crate::BitWriter<'a, REG, CmdTimeoutErrEn>;
impl<'a, REG> CmdTimeoutErrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdTimeoutErrEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdTimeoutErrEn::Value2)
    }
}
#[doc = "Command CRC Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdCrcErrEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<CmdCrcErrEn> for bool {
    #[inline(always)]
    fn from(variant: CmdCrcErrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_CRC_ERR_EN` reader - Command CRC Error Status Enable"]
pub type CmdCrcErrEnR = crate::BitReader<CmdCrcErrEn>;
impl CmdCrcErrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdCrcErrEn {
        match self.bits {
            false => CmdCrcErrEn::Value1,
            true => CmdCrcErrEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdCrcErrEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdCrcErrEn::Value2
    }
}
#[doc = "Field `CMD_CRC_ERR_EN` writer - Command CRC Error Status Enable"]
pub type CmdCrcErrEnW<'a, REG> = crate::BitWriter<'a, REG, CmdCrcErrEn>;
impl<'a, REG> CmdCrcErrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdCrcErrEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdCrcErrEn::Value2)
    }
}
#[doc = "Command End Bit Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdEndBitErrEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<CmdEndBitErrEn> for bool {
    #[inline(always)]
    fn from(variant: CmdEndBitErrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_END_BIT_ERR_EN` reader - Command End Bit Error Status Enable"]
pub type CmdEndBitErrEnR = crate::BitReader<CmdEndBitErrEn>;
impl CmdEndBitErrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdEndBitErrEn {
        match self.bits {
            false => CmdEndBitErrEn::Value1,
            true => CmdEndBitErrEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdEndBitErrEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdEndBitErrEn::Value2
    }
}
#[doc = "Field `CMD_END_BIT_ERR_EN` writer - Command End Bit Error Status Enable"]
pub type CmdEndBitErrEnW<'a, REG> = crate::BitWriter<'a, REG, CmdEndBitErrEn>;
impl<'a, REG> CmdEndBitErrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdEndBitErrEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdEndBitErrEn::Value2)
    }
}
#[doc = "Command Index Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdIndErrEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<CmdIndErrEn> for bool {
    #[inline(always)]
    fn from(variant: CmdIndErrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_IND_ERR_EN` reader - Command Index Error Status Enable"]
pub type CmdIndErrEnR = crate::BitReader<CmdIndErrEn>;
impl CmdIndErrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdIndErrEn {
        match self.bits {
            false => CmdIndErrEn::Value1,
            true => CmdIndErrEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdIndErrEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdIndErrEn::Value2
    }
}
#[doc = "Field `CMD_IND_ERR_EN` writer - Command Index Error Status Enable"]
pub type CmdIndErrEnW<'a, REG> = crate::BitWriter<'a, REG, CmdIndErrEn>;
impl<'a, REG> CmdIndErrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdIndErrEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdIndErrEn::Value2)
    }
}
#[doc = "Data Timeout Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTimeoutErrEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<DataTimeoutErrEn> for bool {
    #[inline(always)]
    fn from(variant: DataTimeoutErrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_TIMEOUT_ERR_EN` reader - Data Timeout Error Status Enable"]
pub type DataTimeoutErrEnR = crate::BitReader<DataTimeoutErrEn>;
impl DataTimeoutErrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataTimeoutErrEn {
        match self.bits {
            false => DataTimeoutErrEn::Value1,
            true => DataTimeoutErrEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DataTimeoutErrEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DataTimeoutErrEn::Value2
    }
}
#[doc = "Field `DATA_TIMEOUT_ERR_EN` writer - Data Timeout Error Status Enable"]
pub type DataTimeoutErrEnW<'a, REG> = crate::BitWriter<'a, REG, DataTimeoutErrEn>;
impl<'a, REG> DataTimeoutErrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DataTimeoutErrEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DataTimeoutErrEn::Value2)
    }
}
#[doc = "Data CRC Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataCrcErrEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<DataCrcErrEn> for bool {
    #[inline(always)]
    fn from(variant: DataCrcErrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_CRC_ERR_EN` reader - Data CRC Error Status Enable"]
pub type DataCrcErrEnR = crate::BitReader<DataCrcErrEn>;
impl DataCrcErrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataCrcErrEn {
        match self.bits {
            false => DataCrcErrEn::Value1,
            true => DataCrcErrEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DataCrcErrEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DataCrcErrEn::Value2
    }
}
#[doc = "Field `DATA_CRC_ERR_EN` writer - Data CRC Error Status Enable"]
pub type DataCrcErrEnW<'a, REG> = crate::BitWriter<'a, REG, DataCrcErrEn>;
impl<'a, REG> DataCrcErrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DataCrcErrEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DataCrcErrEn::Value2)
    }
}
#[doc = "Data End Bit Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataEndBitErrEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<DataEndBitErrEn> for bool {
    #[inline(always)]
    fn from(variant: DataEndBitErrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_END_BIT_ERR_EN` reader - Data End Bit Error Status Enable"]
pub type DataEndBitErrEnR = crate::BitReader<DataEndBitErrEn>;
impl DataEndBitErrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataEndBitErrEn {
        match self.bits {
            false => DataEndBitErrEn::Value1,
            true => DataEndBitErrEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DataEndBitErrEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DataEndBitErrEn::Value2
    }
}
#[doc = "Field `DATA_END_BIT_ERR_EN` writer - Data End Bit Error Status Enable"]
pub type DataEndBitErrEnW<'a, REG> = crate::BitWriter<'a, REG, DataEndBitErrEn>;
impl<'a, REG> DataEndBitErrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DataEndBitErrEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DataEndBitErrEn::Value2)
    }
}
#[doc = "Current Limit Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentLimitErrEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<CurrentLimitErrEn> for bool {
    #[inline(always)]
    fn from(variant: CurrentLimitErrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURRENT_LIMIT_ERR_EN` reader - Current Limit Error Status Enable"]
pub type CurrentLimitErrEnR = crate::BitReader<CurrentLimitErrEn>;
impl CurrentLimitErrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CurrentLimitErrEn {
        match self.bits {
            false => CurrentLimitErrEn::Value1,
            true => CurrentLimitErrEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CurrentLimitErrEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CurrentLimitErrEn::Value2
    }
}
#[doc = "Field `CURRENT_LIMIT_ERR_EN` writer - Current Limit Error Status Enable"]
pub type CurrentLimitErrEnW<'a, REG> = crate::BitWriter<'a, REG, CurrentLimitErrEn>;
impl<'a, REG> CurrentLimitErrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CurrentLimitErrEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CurrentLimitErrEn::Value2)
    }
}
#[doc = "Auto CMD12 Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcmdErrEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<AcmdErrEn> for bool {
    #[inline(always)]
    fn from(variant: AcmdErrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_ERR_EN` reader - Auto CMD12 Error Status Enable"]
pub type AcmdErrEnR = crate::BitReader<AcmdErrEn>;
impl AcmdErrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcmdErrEn {
        match self.bits {
            false => AcmdErrEn::Value1,
            true => AcmdErrEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AcmdErrEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AcmdErrEn::Value2
    }
}
#[doc = "Field `ACMD_ERR_EN` writer - Auto CMD12 Error Status Enable"]
pub type AcmdErrEnW<'a, REG> = crate::BitWriter<'a, REG, AcmdErrEn>;
impl<'a, REG> AcmdErrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AcmdErrEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AcmdErrEn::Value2)
    }
}
#[doc = "Target Response Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetRespErrEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<TargetRespErrEn> for bool {
    #[inline(always)]
    fn from(variant: TargetRespErrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TARGET_RESP_ERR_EN` reader - Target Response Error Status Enable"]
pub type TargetRespErrEnR = crate::BitReader<TargetRespErrEn>;
impl TargetRespErrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TargetRespErrEn {
        match self.bits {
            false => TargetRespErrEn::Value1,
            true => TargetRespErrEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TargetRespErrEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TargetRespErrEn::Value2
    }
}
#[doc = "Field `TARGET_RESP_ERR_EN` writer - Target Response Error Status Enable"]
pub type TargetRespErrEnW<'a, REG> = crate::BitWriter<'a, REG, TargetRespErrEn>;
impl<'a, REG> TargetRespErrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TargetRespErrEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TargetRespErrEn::Value2)
    }
}
#[doc = "Ceata Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CeataErrEn {
    #[doc = "0: Masked"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<CeataErrEn> for bool {
    #[inline(always)]
    fn from(variant: CeataErrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEATA_ERR_EN` reader - Ceata Error Status Enable"]
pub type CeataErrEnR = crate::BitReader<CeataErrEn>;
impl CeataErrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CeataErrEn {
        match self.bits {
            false => CeataErrEn::Value1,
            true => CeataErrEn::Value2,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CeataErrEn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CeataErrEn::Value2
    }
}
#[doc = "Field `CEATA_ERR_EN` writer - Ceata Error Status Enable"]
pub type CeataErrEnW<'a, REG> = crate::BitWriter<'a, REG, CeataErrEn>;
impl<'a, REG> CeataErrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CeataErrEn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CeataErrEn::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Command Timeout Error Status Enable"]
    #[inline(always)]
    pub fn cmd_timeout_err_en(&self) -> CmdTimeoutErrEnR {
        CmdTimeoutErrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn cmd_crc_err_en(&self) -> CmdCrcErrEnR {
        CmdCrcErrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cmd_end_bit_err_en(&self) -> CmdEndBitErrEnR {
        CmdEndBitErrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn cmd_ind_err_en(&self) -> CmdIndErrEnR {
        CmdIndErrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn data_timeout_err_en(&self) -> DataTimeoutErrEnR {
        DataTimeoutErrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn data_crc_err_en(&self) -> DataCrcErrEnR {
        DataCrcErrEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn data_end_bit_err_en(&self) -> DataEndBitErrEnR {
        DataEndBitErrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Status Enable"]
    #[inline(always)]
    pub fn current_limit_err_en(&self) -> CurrentLimitErrEnR {
        CurrentLimitErrEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn acmd_err_en(&self) -> AcmdErrEnR {
        AcmdErrEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Target Response Error Status Enable"]
    #[inline(always)]
    pub fn target_resp_err_en(&self) -> TargetRespErrEnR {
        TargetRespErrEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Ceata Error Status Enable"]
    #[inline(always)]
    pub fn ceata_err_en(&self) -> CeataErrEnR {
        CeataErrEnR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_timeout_err_en(&mut self) -> CmdTimeoutErrEnW<EnIntStatusErrSpec> {
        CmdTimeoutErrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Command CRC Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_err_en(&mut self) -> CmdCrcErrEnW<EnIntStatusErrSpec> {
        CmdCrcErrEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_end_bit_err_en(&mut self) -> CmdEndBitErrEnW<EnIntStatusErrSpec> {
        CmdEndBitErrEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ind_err_en(&mut self) -> CmdIndErrEnW<EnIntStatusErrSpec> {
        CmdIndErrEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn data_timeout_err_en(&mut self) -> DataTimeoutErrEnW<EnIntStatusErrSpec> {
        DataTimeoutErrEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn data_crc_err_en(&mut self) -> DataCrcErrEnW<EnIntStatusErrSpec> {
        DataCrcErrEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn data_end_bit_err_en(&mut self) -> DataEndBitErrEnW<EnIntStatusErrSpec> {
        DataEndBitErrEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Current Limit Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn current_limit_err_en(&mut self) -> CurrentLimitErrEnW<EnIntStatusErrSpec> {
        CurrentLimitErrEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmd_err_en(&mut self) -> AcmdErrEnW<EnIntStatusErrSpec> {
        AcmdErrEnW::new(self, 8)
    }
    #[doc = "Bit 12 - Target Response Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn target_resp_err_en(&mut self) -> TargetRespErrEnW<EnIntStatusErrSpec> {
        TargetRespErrEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Ceata Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ceata_err_en(&mut self) -> CeataErrEnW<EnIntStatusErrSpec> {
        CeataErrEnW::new(self, 13)
    }
}
#[doc = "Error Interrupt Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en_int_status_err::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en_int_status_err::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnIntStatusErrSpec;
impl crate::RegisterSpec for EnIntStatusErrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`en_int_status_err::R`](R) reader structure"]
impl crate::Readable for EnIntStatusErrSpec {}
#[doc = "`write(|w| ..)` method takes [`en_int_status_err::W`](W) writer structure"]
impl crate::Writable for EnIntStatusErrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EN_INT_STATUS_ERR to value 0"]
impl crate::Resettable for EnIntStatusErrSpec {
    const RESET_VALUE: u16 = 0;
}
